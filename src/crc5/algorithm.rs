// Copyright 2025 Don MacAskill. Licensed under MIT or Apache-2.0 and Zlib.

//! This module provides the CRC-5 algorithm implementations.
//!
//! CRC-5 computation is performed by scaling 5-bit values to 32-bit space,
//! using the shared width32_ops module, and then scaling the result back to 5 bits.
//! For the reflected CRC-5/USB variant, no scaling is needed (state in low bits).
//! For forward variants, values are shifted left by 27 (32-5).

#![cfg(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64"))]

use crate::enums::Reflector;
use crate::structs::CrcState;
use crate::traits::{ArchOps, EnhancedCrcWidth};

impl EnhancedCrcWidth for crate::structs::Width5 {
    #[inline(always)]
    fn load_constants(reflected: bool) -> [[u64; 2]; 4] {
        crate::crc32::width32_ops::load_constants(reflected)
    }

    #[inline(always)]
    unsafe fn create_state<T: ArchOps>(
        value: Self::Value,
        reflected: bool,
        ops: &T,
    ) -> CrcState<T::Vector>
    where
        T::Vector: Copy,
    {
        let vector = if reflected {
            // Reflected: state in low 32 bits, mask to 5 bits
            ops.create_vector_from_u32((value as u32) & 0x1f, false)
        } else {
            // Forward: scale to 32-bit space, shift left 27
            let scaled = ((value as u32) & 0x1f) << 27;
            ops.create_vector_from_u32(scaled, true)
        };

        CrcState {
            value: vector,
            reflected,
        }
    }

    #[inline(always)]
    unsafe fn extract_result<T: ArchOps>(vector: T::Vector, reflected: bool, ops: &T) -> Self::Value
    where
        T::Vector: Copy,
    {
        let u64s = ops.extract_u64s(vector);

        if reflected {
            (u64s[0] as u8) & 0x1f
        } else {
            (((u64s[1] >> 32) >> 27) as u8) & 0x1f
        }
    }

    #[inline(always)]
    unsafe fn fold_16<T: ArchOps>(
        state: &mut CrcState<T::Vector>,
        coeff: T::Vector,
        data_to_xor: T::Vector,
        ops: &T,
    ) where
        T::Vector: Copy,
    {
        crate::crc32::width32_ops::fold_16(state, coeff, data_to_xor, ops)
    }

    #[inline(always)]
    unsafe fn fold_width<T: ArchOps>(state: &mut CrcState<T::Vector>, high: u64, low: u64, ops: &T)
    where
        T::Vector: Copy,
    {
        crate::crc32::width32_ops::fold_width(state, high, low, ops)
    }

    #[inline(always)]
    unsafe fn barrett_reduction<T: ArchOps>(
        state: &CrcState<T::Vector>,
        poly: u64,
        mu: u64,
        ops: &T,
    ) -> Self::Value
    where
        T::Vector: Copy,
    {
        let u64s = crate::crc32::width32_ops::barrett_reduction(state, poly, mu, ops);
        if state.reflected {
            (u64s[1] as u8) & 0x1f
        } else {
            (((u64s[0] >> 32) >> 27) as u8) & 0x1f
        }
    }

    #[inline(always)]
    unsafe fn create_coefficient<T: ArchOps>(
        high: u64,
        low: u64,
        _reflected: bool,
        ops: &T,
    ) -> T::Vector
    where
        T::Vector: Copy,
    {
        crate::crc32::width32_ops::create_coefficient(high, low, ops)
    }

    #[inline(always)]
    unsafe fn perform_final_reduction<T: ArchOps>(
        state: T::Vector,
        reflected: bool,
        keys: &[u64; 23],
        ops: &T,
    ) -> Self::Value
    where
        T::Vector: Copy,
    {
        let u64s = crate::crc32::width32_ops::perform_final_reduction(state, reflected, keys, ops);
        if reflected {
            (u64s[1] as u8) & 0x1f
        } else {
            (((u64s[0] >> 32) >> 27) as u8) & 0x1f
        }
    }

    #[inline(always)]
    fn get_last_bytes_table_ptr(reflected: bool, remaining_len: usize) -> (*const u8, usize) {
        crate::crc32::width32_ops::get_last_bytes_table_ptr(reflected, remaining_len)
    }
}

/// Process inputs smaller than 16 bytes for CRC-5
#[allow(dead_code)]
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
