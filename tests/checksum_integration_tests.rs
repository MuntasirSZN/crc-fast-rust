// Copyright 2025 Don MacAskill. Licensed under MIT or Apache-2.0 and Zlib.

#![cfg(feature = "cli")]

use std::fs;
use std::path::PathBuf;
use std::process::Command;

/// Build the `checksum` binary and return its absolute path.
/// Uses `cargo metadata --format-version=1 --no-deps` to discover the exact
/// `target_directory` — robust regardless of CARGO_TARGET_DIR or host/target.
fn checksum_binary() -> PathBuf {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR")
        .expect("CARGO_MANIFEST_DIR not set (tests must run via cargo)");

    // 1. Build the binary with the cli feature (no-op if already built and up-to-date)
    let build_status = Command::new("cargo")
        .args(["build", "--quiet", "--features", "cli", "--bin", "checksum"])
        .current_dir(&manifest_dir)
        .status()
        .expect("Failed to execute cargo build");
    if !build_status.success() {
        panic!("cargo build of `checksum` failed (status: {build_status})");
    }

    // 2. Use `cargo metadata` to find the exact target_directory (handles cross builds)
    let meta_output = Command::new("cargo")
        .args(["metadata", "--format-version=1", "--no-deps"])
        .current_dir(&manifest_dir)
        .output()
        .expect("Failed to execute cargo metadata");
    if !meta_output.status.success() {
        panic!(
            "cargo metadata failed:\nstderr: {}",
            String::from_utf8_lossy(&meta_output.stderr)
        );
    }
    // Parse just `target_directory` from the JSON without pulling in serde_json.
    let stdout = String::from_utf8_lossy(&meta_output.stdout);
    let target_directory = extract_json_string(&stdout, "target_directory")
        .unwrap_or_else(|| panic!("metadata.target_directory missing in:\n{stdout}"));

    // cargo uses one of:
    //   <target_dir>/debug/checksum                        (host)
    //   <target_dir>/<triple>/debug/checksum               (cross)
    let exe_suffix = std::env::consts::EXE_SUFFIX;
    let bin_name = format!("checksum{exe_suffix}");
    let target_root = PathBuf::from(target_directory);

    for dir in &["debug", "release"] {
        let direct = target_root.join(dir).join(&bin_name);
        if direct.exists() {
            return direct;
        }
    }
    if let Ok(entries) = std::fs::read_dir(&target_root) {
        for entry in entries.flatten() {
            for dir in &["debug", "release"] {
                let candidate = entry.path().join(dir).join(&bin_name);
                if candidate.exists() {
                    return candidate;
                }
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

/// Tiny JSON string-value extractor for `cargo metadata --format-version=1` output.
/// Avoids pulling in `serde_json` as a dev-dependency just to read one field.
/// Handles simple cases like `"target_directory":"/abs/path"` (no escaped quotes/backslash in value).
fn extract_json_string(json: &str, key: &str) -> Option<String> {
    let needle = format!("\"{}\":\"", key);
    let start = json.find(&needle)? + needle.len();
    let rest = &json[start..];
    let end = rest.find('"')?;
    Some(rest[..end].to_string())
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
