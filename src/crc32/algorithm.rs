// Copyright 2025 Don MacAskill. Licensed under MIT or Apache-2.0 and Zlib.

//! This module provides the CRC-32 algorithm implementations for areas where it differs from
//! CRC-64.

#![cfg(any(
    target_arch = "x86",
    target_arch = "x86_64",
    target_arch = "aarch64",
    target_arch = "wasm32",
))]

use crate::enums::Reflector;
use crate::structs::CrcState;
use crate::traits::{ArchOps, EnhancedCrcWidth};

crate::crc32::width32_ops::impl_scaled_width32!(crate::structs::Width32, u32::MAX, 0);

/// Process inputs smaller than 16 bytes
#[inline]
#[cfg_attr(
    any(target_arch = "x86", target_arch = "x86_64"),
    target_feature(enable = "ssse3,sse4.1,pclmulqdq")
)]
#[cfg_attr(target_arch = "aarch64", target_feature(enable = "aes"))]
pub(crate) unsafe fn process_0_to_15<T: ArchOps, W: EnhancedCrcWidth>(
    data: &[u8],
    state: &mut CrcState<T::Vector>,
    reflector: &Reflector<T::Vector>,
    keys: &[u64; 23],
    ops: &T,
) -> W::Value
where
    T::Vector: Copy,
{
    crate::crc32::width32_ops::process_0_to_15::<T, W>(data, state, reflector, keys, ops)
}
