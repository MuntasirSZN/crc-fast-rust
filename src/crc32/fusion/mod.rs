// Copyright 2025 Don MacAskill. Licensed under MIT or Apache-2.0 and Zlib.

//! This module provides support for calculating CRC-32/ISO-HDLC and CRC-32/ISCSI using
//! fusion techniques.
//!
//! https://www.corsix.org/content/fast-crc32c-4k
//! https://www.corsix.org/content/alternative-exposition-crc32_4k_pclmulqdq
//! https://dougallj.wordpress.com/2022/05/22/faster-crc32-on-the-apple-m1/
//! https://github.com/corsix/fast-crc32/

mod aarch64;
mod x86;

/// Only AArch64 has native CRC-32/ISO-HDLC instructions
#[inline(always)]
#[cfg(target_arch = "aarch64")]
pub(crate) fn crc32_iso_hdlc(state: u32, data: &[u8]) -> u32 {
    aarch64::crc32_iso_hdlc(state, data)
}

/// Both AArch64 and x86 have native CRC-32/ISCSI instructions
#[inline(always)]
pub(crate) fn crc32_iscsi(state: u32, data: &[u8]) -> u32 {
    #[cfg(target_arch = "aarch64")]
    {
        aarch64::crc32_iscsi(state, data)
    }

    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    {
        x86::crc32_iscsi(state, data)
    }
}

/// SSE4.2-without-PCLMULQDQ fallback (old Atoms): sequential native CRC32C,
/// correct without PCLMUL-based stream recombination.
#[inline(always)]
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub(crate) fn crc32_iscsi_sse42_only(state: u32, data: &[u8]) -> u32 {
    unsafe { x86::crc32_iscsi_sse42_only(state, data) }
}

/// CRC-only (`__crc32*`, no AES/PMULL) small-buffer paths for AArch64 builds
/// carrying `+crc` but no AES tier. Safe: the callees only require `crc`.
#[cfg(target_arch = "aarch64")]
pub(crate) use aarch64::{crc32_iscsi_small_fast, crc32_iso_hdlc_small_fast};
