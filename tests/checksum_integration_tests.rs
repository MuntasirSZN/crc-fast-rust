// Copyright 2025 Don MacAskill. Licensed under MIT or Apache-2.0 and Zlib.

#![cfg(feature = "cli")]

use std::fs;
use std::path::PathBuf;
use std::process::Command;

/// Build the `checksum` binary and return its absolute path.
/// cargo emits the binary path on stdout (last line) when invoked without `--message-format=json`,
/// and the same path is used internally for `target/debug/...` plus the per-target triple subdir.
fn checksum_binary() -> PathBuf {
    let output = Command::new("cargo")
        .args(["build", "--quiet", "--features", "cli", "--bin", "checksum"])
        .output()
        .expect("Failed to execute cargo build");

    if !output.status.success() {
        panic!(
            "cargo build failed:\nstdout: {}\nstderr: {}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
    }

    // Resolve the binary path. cargo places it in one of:
    //   target/debug/checksum            (host build)
    //   target/<triple>/debug/checksum   (cross build, e.g. CI macos x86_64 vs aarch64)
    // We discover by walking `target/` for the binary named `checksum[.exe]`.
    let exe_suffix = std::env::consts::EXE_SUFFIX;
    let target_root = std::env::var("CARGO_TARGET_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("target"));
    let bin_name = format!("checksum{exe_suffix}");

    let direct = target_root.join("debug").join(&bin_name);
    if direct.exists() {
        return direct;
    }

    // Fallback: walk target/<triple>/debug/{bin_name} (some CI setups)
    if let Ok(entries) = std::fs::read_dir(&target_root) {
        for entry in entries.flatten() {
            let candidate = entry.path().join("debug").join(&bin_name);
            if candidate.exists() {
                return candidate;
            }
        }
    }

    panic!(
        "checksum binary not found in {} (suffix `{}`) after cargo build",
        target_root.display(),
        exe_suffix
    );
}

/// Run the prebuilt `checksum` binary with `args` and assert it succeeded.
fn run_checksum_assert_success(bin: &std::path::Path, args: &[&str]) {
    let output = Command::new(bin)
        .args(args)
        .output()
        .expect("Failed to execute checksum");
    if !output.status.success() {
        panic!(
            "checksum {:?} failed (status {:?}):\nstdout: {}\nstderr: {}",
            args,
            output.status,
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
    }
}

/// Run the prebuilt `checksum` binary and assert it failed, returning the stderr.
fn run_checksum_assert_failure(bin: &std::path::Path, args: &[&str]) -> String {
    let output = Command::new(bin)
        .args(args)
        .output()
        .expect("Failed to execute checksum");
    if output.status.success() {
        panic!(
            "checksum {:?} unexpectedly succeeded:\nstdout: {}\nstderr: {}",
            args,
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
    }
    String::from_utf8_lossy(&output.stderr).into_owned()
}

#[test]
#[cfg_attr(miri, ignore)] // Miri doesn't allow this due to isolation restrictions
fn test_benchmark_flag_parsing() {
    let bin = checksum_binary();
    let output = Command::new(&bin)
        .args(["-a", "CRC-32/ISCSI", "-b", "--duration", "0.1"])
        .output()
        .expect("Failed to execute checksum");
    assert!(
        output.status.success(),
        "Command should succeed with -b flag"
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Algorithm: CRC-32/ISCSI"));
    assert!(stdout.contains("Throughput:"));
    assert!(stdout.contains("GiB/s"));
}

#[test]
#[cfg_attr(miri, ignore)] // Miri doesn't allow this due to isolation restrictions
fn test_benchmark_with_size_parameter() {
    let bin = checksum_binary();
    let stdout = String::from_utf8_lossy(
        &Command::new(&bin)
            .args(["-a", "CRC-32/ISCSI", "-b", "--size", "1024"])
            .output()
            .expect("Failed to execute checksum")
            .stdout,
    )
    .into_owned();
    assert!(stdout.contains("Data Size: 1,024 bytes"));
}

#[test]
#[cfg_attr(miri, ignore)] // Miri doesn't allow this due to isolation restrictions
fn test_benchmark_with_duration_parameter() {
    let bin = checksum_binary();
    let stdout = String::from_utf8_lossy(
        &Command::new(&bin)
            .args(["-a", "CRC-32/ISCSI", "-b", "--duration", "1.0"])
            .output()
            .expect("Failed to execute checksum")
            .stdout,
    )
    .into_owned();
    assert!(stdout.contains("Duration: 1."));
}

#[test]
#[cfg_attr(miri, ignore)] // Miri doesn't allow this due to isolation restrictions
fn test_benchmark_invalid_size() {
    let bin = checksum_binary();
    let stderr = run_checksum_assert_failure(&bin, &["-a", "CRC-32/ISCSI", "-b", "--size", "0"]);
    assert!(stderr.contains("Size must be greater than 0"));
}

#[test]
#[cfg_attr(miri, ignore)] // Miri doesn't allow this due to isolation restrictions
fn test_benchmark_invalid_duration() {
    let bin = checksum_binary();
    let stderr =
        run_checksum_assert_failure(&bin, &["-a", "CRC-32/ISCSI", "-b", "--duration", "0"]);
    assert!(stderr.contains("Duration must be greater than 0"));
}

#[test]
#[cfg_attr(miri, ignore)] // Miri doesn't allow this due to isolation restrictions
fn test_benchmark_with_file_input() {
    let bin = checksum_binary();

    // Use a unique temp file to avoid races when tests run in parallel
    let mut temp_path = std::env::temp_dir();
    temp_path.push(format!(
        "test_benchmark_file_{}_{}.txt",
        std::process::id(),
        format!("{:?}", std::thread::current().id()).len()
    ));
    fs::write(&temp_path, "Hello, benchmark world!").expect("Failed to create test file");
    let temp_str = temp_path.to_str().expect("temp path not utf8");

    let output = Command::new(&bin)
        .args([
            "-a",
            "CRC-32/ISCSI",
            "-b",
            "-f",
            temp_str,
            "--duration",
            "0.5",
        ])
        .output()
        .expect("Failed to execute checksum");

    // Clean up
    let _ = fs::remove_file(&temp_path);

    if !output.status.success() {
        panic!(
            "checksum failed (status {:?}):\nstdout: {}\nstderr: {}",
            output.status,
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
    }
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Data Size: 23 bytes"));
}

#[test]
#[cfg_attr(miri, ignore)] // Miri doesn't allow this due to isolation restrictions
fn test_benchmark_with_string_input() {
    let bin = checksum_binary();
    let stdout = String::from_utf8_lossy(
        &Command::new(&bin)
            .args([
                "-a",
                "CRC-32/ISCSI",
                "-b",
                "-s",
                "test string",
                "--duration",
                "0.5",
            ])
            .output()
            .expect("Failed to execute checksum")
            .stdout,
    )
    .into_owned();
    assert!(stdout.contains("Data Size: 11 bytes"));
}

#[test]
#[cfg_attr(miri, ignore)] // Miri doesn't allow this due to isolation restrictions
fn test_benchmark_different_algorithms() {
    let bin = checksum_binary();
    let algorithms = ["CRC-32/ISCSI", "CRC-64/NVME"];

    for algorithm in &algorithms {
        run_checksum_assert_success(&bin, &["-a", algorithm, "-b", "--duration", "0.5"]);
        let stdout = String::from_utf8_lossy(
            &Command::new(&bin)
                .args(["-a", algorithm, "-b", "--duration", "0.5"])
                .output()
                .expect("Failed to execute checksum")
                .stdout,
        )
        .into_owned();
        assert!(
            stdout.contains(&format!("Algorithm: {}", algorithm)),
            "Algorithm {} should work; got stdout: {}",
            algorithm,
            stdout
        );
    }
}

#[test]
#[cfg_attr(miri, ignore)] // Miri doesn't allow this due to isolation restrictions
fn test_benchmark_size_without_benchmark_flag() {
    let bin = checksum_binary();
    let stderr = run_checksum_assert_failure(&bin, &["-a", "CRC-32/ISCSI", "--size", "1024"]);
    assert!(stderr.contains("--size and --duration can only be used with -b flag"));
}

#[test]
#[cfg_attr(miri, ignore)] // Miri doesn't allow this due to isolation restrictions
fn test_benchmark_nonexistent_file() {
    let bin = checksum_binary();
    // Use an absolute path to a guaranteed-missing file in the system temp dir,
    // so it cannot accidentally exist from a previous test run.
    let mut missing_path = std::env::temp_dir();
    missing_path.push(format!(
        "definitely_missing_{}_{}_{}.txt",
        std::process::id(),
        format!("{:?}", std::thread::current().id()).len(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0)
    ));
    assert!(!missing_path.exists(), "test file leaked from prior run");

    let stderr = run_checksum_assert_failure(
        &bin,
        &[
            "-a",
            "CRC-32/ISCSI",
            "-b",
            "-f",
            missing_path.to_str().expect("temp path not utf8"),
        ],
    );
    assert!(
        stderr.contains("File not found"),
        "expected 'File not found' in stderr, got: {}",
        stderr
    );
}
