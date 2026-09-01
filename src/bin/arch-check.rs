// Copyright 2025 Don MacAskill. Licensed under MIT or Apache-2.0 and Zlib.

//! This is a simple program that checks if the target architecture supports certain features.

#[cfg(target_arch = "aarch64")]
cpufeatures::new!(aarch64_aes, "aes");
#[cfg(target_arch = "aarch64")]
cpufeatures::new!(aarch64_sha3, "sha3");
#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
cpufeatures::new!(x86_sse2, "sse2");
#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
cpufeatures::new!(x86_sse41, "sse4.1");
#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
cpufeatures::new!(x86_pclmulqdq, "pclmulqdq");
#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
cpufeatures::new!(x86_avx2, "avx2");
#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
cpufeatures::new!(x86_vpclmulqdq, "vpclmulqdq");
#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
cpufeatures::new!(x86_avx512f, "avx512f");
#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
cpufeatures::new!(x86_avx512vl, "avx512vl");

use crc_fast::get_calculator_target;
use crc_fast::CrcAlgorithm::{Crc32Iscsi, Crc32IsoHdlc, Crc64Nvme};

fn main() {
    // Check the target architecture and call the appropriate function
    #[cfg(target_arch = "aarch64")]
    aarch64_features();

    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    x86_features();

    print_targets();

    print_cpu_info();
}

#[cfg(target_arch = "aarch64")]
fn aarch64_features() {
    let checkmark: char = '✓';

    println!("[AArch64] Checking for features...");

    // NEON is mandatory on aarch64; use compile-time check
    if cfg!(target_feature = "neon") {
        println!("  {checkmark} NEON",);
    } else {
        println!("  x NEON");
    }

    if cfg!(target_feature = "crc") {
        println!("  {checkmark} CRC",);
    } else {
        println!("  x CRC");
    }

    if aarch64_sha3::get() {
        println!("  {checkmark} SHA3\n",);
    } else {
        println!("  x SHA3\n");
    }
}

#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
fn x86_features() {
    let checkmark: char = '✓';

    println!("[X86] Checking for features...");

    if x86_sse2::get() {
        println!("  {checkmark} SSE2",);
    } else {
        println!("  x SSE2");
    }

    if x86_sse41::get() {
        println!("  {checkmark} SSE4.1",);
    } else {
        println!("  x SSE4.1");
    }

    if x86_pclmulqdq::get() {
        println!("  {checkmark} PCLMULQDQ",);
    } else {
        println!("  x PCLMULQDQ");
    }

    if x86_avx2::get() {
        println!("  {checkmark} AVX2",);
    } else {
        println!("  x AVX2");
    }

    if x86_vpclmulqdq::get() {
        println!("  {checkmark} VPCLMULQDQ",);
    } else {
        println!("  x VPCLMULQDQ");
    }

    if x86_avx512f::get() {
        println!("  {checkmark} AVX512F",);
    } else {
        println!("  x AVX512F");
    }

    if x86_avx512vl::get() {
        println!("  {checkmark} AVX512VL\n",);
    } else {
        println!("  x AVX512VL\n");
    }
}

/// Print the acceleration targets
fn print_targets() {
    let checkmark: char = '✓';

    println!("[Acceleration targets]");

    println!(
        "  {} CRC-32/ISCSI target: {}",
        checkmark,
        get_calculator_target(Crc32Iscsi)
    );
    println!(
        "  {} CRC-32/ISO-HDLC target: {}",
        checkmark,
        get_calculator_target(Crc32IsoHdlc)
    );
    println!(
        "  {} CRC-64/NVME target: {}\n",
        checkmark,
        get_calculator_target(Crc64Nvme)
    );
}

/// Print the first entry of /proc/cpuinfo if it's available
fn print_cpu_info() {
    println!("\n[CPU Info]");
    if let Ok(cpuinfo) = std::fs::read_to_string("/proc/cpuinfo") {
        // Split the content by double newlines and take the first entry
        if let Some(first_cpu) = cpuinfo.split("\n\n").next() {
            println!("{first_cpu}",);
        } else {
            println!("No CPU information found.");
        }
    } else {
        println!("Failed to read /proc/cpuinfo. This may not be available on your platform.\n");
    }
}
