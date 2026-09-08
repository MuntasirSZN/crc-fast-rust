// Copyright 2025 Don MacAskill. Licensed under MIT or Apache-2.0 and Zlib.

//! `wasm32` manual-SIMD backend over the stable `simd128` instruction set.
//!
//! Every intrinsic used here is `#[stable(feature = "wasm_simd", since =
//! "1.54.0")]`, observed in the installed toolchain source
//! (`library/stdarch/crates/core_arch/src/wasm32/simd128.rs`), so this
//! backend ships under the crate's `rust-version = "1.89"` floor with no
//! nightly gates and no `core::simd` dependency.
//!
//! Lane convention (matches the `x86`/`aarch64` backends exactly): lane 0 is
//! bits `0..64` (bytes `0..8`), lane 1 is bits `64..128` (bytes `8..16`).
//! `simd128` has no carryless-multiply instruction, so the four
//! `carryless_mul_*` methods unpack both lanes, run the scalar bitwise core
//! (`crate::arch::software::carryless_mul_u64`), and re-pack. `xor3` is two
//! `v128.xor`s. Byte shifts are single `i8x16.shuffle`s against a zero
//! vector (LLVM `simd_shuffle` addressing: indices `0..16` select from the
//! first operand, `16..32` from the second).

#![cfg(target_arch = "wasm32")]

use crate::arch::software::carryless_mul_u64;
use crate::traits::ArchOps;
use core::arch::wasm32::*;

/// `wasm32` `simd128` implementation of [`ArchOps`].
///
/// Inherit the default [`ArchOps::process_enhanced_simd_blocks`] (returns
/// `false`): the generic fold path handles all block processing.
#[derive(Debug, Copy, Clone)]
pub struct WasmSimd128Ops;

impl WasmSimd128Ops {
    /// Pack two `u64` lanes: `lane0` into bits `0..64`, `lane1` into `64..128`.
    ///
    /// Requires `simd128`.
    #[inline]
    #[target_feature(enable = "simd128")]
    unsafe fn pack_lanes(&self, lane0: u64, lane1: u64) -> v128 {
        u64x2_replace_lane::<1>(u64x2_splat(lane0), lane1)
    }

    /// Sixteen zero bytes.
    ///
    /// Requires `simd128`.
    #[inline]
    #[target_feature(enable = "simd128")]
    unsafe fn zero(&self) -> v128 {
        u8x16_splat(0)
    }
}

impl ArchOps for WasmSimd128Ops {
    type Vector = v128;

    /// Requires `simd128`.
    #[inline]
    #[target_feature(enable = "simd128")]
    unsafe fn create_vector_from_u64_pair(
        &self,
        high: u64,
        low: u64,
        reflected: bool,
    ) -> Self::Vector {
        // Same layout as the x86/aarch64 backends: reflected puts `high` in
        // lane 0 and `low` in lane 1, non-reflected the reverse.
        if reflected {
            self.pack_lanes(high, low)
        } else {
            self.pack_lanes(low, high)
        }
    }

    /// Requires `simd128`.
    #[inline]
    #[target_feature(enable = "simd128")]
    unsafe fn create_vector_from_u64_pair_non_reflected(
        &self,
        high: u64,
        low: u64,
    ) -> Self::Vector {
        self.pack_lanes(low, high)
    }

    /// Requires `simd128`.
    #[inline]
    #[target_feature(enable = "simd128")]
    unsafe fn create_vector_from_u64(&self, value: u64, high: bool) -> Self::Vector {
        if high {
            self.pack_lanes(0, value)
        } else {
            self.pack_lanes(value, 0)
        }
    }

    /// Requires `simd128`.
    #[inline]
    #[target_feature(enable = "simd128")]
    unsafe fn extract_u64s(&self, vector: Self::Vector) -> [u64; 2] {
        [
            u64x2_extract_lane::<0>(vector),
            u64x2_extract_lane::<1>(vector),
        ]
    }

    /// Requires `simd128`.
    #[inline]
    #[target_feature(enable = "simd128")]
    unsafe fn extract_poly64s(&self, vector: Self::Vector) -> [u64; 2] {
        // Polynomial and integer lanes coincide, as on x86.
        self.extract_u64s(vector)
    }

    /// Requires `simd128`.
    #[inline]
    #[target_feature(enable = "simd128")]
    unsafe fn xor_vectors(&self, a: Self::Vector, b: Self::Vector) -> Self::Vector {
        v128_xor(a, b)
    }

    /// Requires `simd128`.
    #[inline]
    #[target_feature(enable = "simd128")]
    unsafe fn load_bytes(&self, ptr: *const u8) -> Self::Vector {
        v128_load(ptr as *const v128)
    }

    /// Requires `simd128`.
    #[inline]
    #[target_feature(enable = "simd128")]
    unsafe fn load_aligned(&self, ptr: *const [u64; 2]) -> Self::Vector {
        // `v128.load` has no alignment requirement, matching the backends'
        // unaligned loads.
        v128_load(ptr as *const v128)
    }

    /// Requires `simd128`.
    #[inline]
    #[target_feature(enable = "simd128")]
    unsafe fn shuffle_bytes(&self, data: Self::Vector, mask: Self::Vector) -> Self::Vector {
        // `i8x16.swizzle` matches the `aarch64` table lookup (`vqtbl1q_u8`):
        // lanes whose unsigned index exceeds 15 yield zero.
        i8x16_swizzle(data, mask)
    }

    /// Requires `simd128`.
    #[inline]
    #[target_feature(enable = "simd128")]
    unsafe fn blend_vectors(
        &self,
        a: Self::Vector,
        b: Self::Vector,
        mask: Self::Vector,
    ) -> Self::Vector {
        // Select bytes from `b` where the mask MSB is set, else from `a`
        // (same contract as `_mm_blendv_epi8` / `vbslq_u8`).
        let msb = i8x16_lt(mask, self.zero());
        v128_bitselect(b, a, msb)
    }

    /// Requires `simd128`.
    #[inline]
    #[target_feature(enable = "simd128")]
    unsafe fn shift_left_8(&self, vector: Self::Vector) -> Self::Vector {
        i8x16_shuffle::<0, 1, 2, 3, 4, 5, 6, 7, 16, 17, 18, 19, 20, 21, 22, 23>(self.zero(), vector)
    }

    /// Requires `simd128`.
    #[inline]
    #[target_feature(enable = "simd128")]
    unsafe fn set_all_bytes(&self, value: u8) -> Self::Vector {
        u8x16_splat(value)
    }

    /// Requires `simd128`.
    #[inline]
    #[target_feature(enable = "simd128")]
    unsafe fn create_compare_mask(&self, vector: Self::Vector) -> Self::Vector {
        // `0xFF` bytes where the input MSB is set (same as `vcltq_s8(v, 0)`).
        i8x16_lt(vector, self.zero())
    }

    /// Requires `simd128`.
    #[inline]
    #[target_feature(enable = "simd128")]
    unsafe fn and_vectors(&self, a: Self::Vector, b: Self::Vector) -> Self::Vector {
        v128_and(a, b)
    }

    /// Requires `simd128`.
    #[inline]
    #[target_feature(enable = "simd128")]
    unsafe fn shift_right_32(&self, vector: Self::Vector) -> Self::Vector {
        i8x16_shuffle::<4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19>(
            vector,
            self.zero(),
        )
    }

    /// Requires `simd128`.
    #[inline]
    #[target_feature(enable = "simd128")]
    unsafe fn shift_left_32(&self, vector: Self::Vector) -> Self::Vector {
        i8x16_shuffle::<0, 1, 2, 3, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27>(
            self.zero(),
            vector,
        )
    }

    /// Requires `simd128`.
    #[inline]
    #[target_feature(enable = "simd128")]
    unsafe fn create_vector_from_u32(&self, value: u32, high: bool) -> Self::Vector {
        if high {
            u32x4_replace_lane::<3>(self.zero(), value)
        } else {
            u32x4_replace_lane::<0>(self.zero(), value)
        }
    }

    /// Requires `simd128`.
    #[inline]
    #[target_feature(enable = "simd128")]
    unsafe fn shift_left_4(&self, vector: Self::Vector) -> Self::Vector {
        self.shift_left_32(vector)
    }

    /// Requires `simd128`.
    #[inline]
    #[target_feature(enable = "simd128")]
    unsafe fn shift_right_4(&self, vector: Self::Vector) -> Self::Vector {
        self.shift_right_32(vector)
    }

    /// Requires `simd128`.
    #[inline]
    #[target_feature(enable = "simd128")]
    unsafe fn shift_right_8(&self, vector: Self::Vector) -> Self::Vector {
        i8x16_shuffle::<8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23>(
            vector,
            self.zero(),
        )
    }

    /// Requires `simd128`.
    #[inline]
    #[target_feature(enable = "simd128")]
    unsafe fn shift_right_5(&self, vector: Self::Vector) -> Self::Vector {
        i8x16_shuffle::<5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20>(
            vector,
            self.zero(),
        )
    }

    /// Requires `simd128`.
    #[inline]
    #[target_feature(enable = "simd128")]
    unsafe fn shift_right_6(&self, vector: Self::Vector) -> Self::Vector {
        i8x16_shuffle::<6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21>(
            vector,
            self.zero(),
        )
    }

    /// Requires `simd128`.
    #[inline]
    #[target_feature(enable = "simd128")]
    unsafe fn shift_right_7(&self, vector: Self::Vector) -> Self::Vector {
        i8x16_shuffle::<7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22>(
            vector,
            self.zero(),
        )
    }

    /// Requires `simd128`.
    #[inline]
    #[target_feature(enable = "simd128")]
    unsafe fn shift_right_12(&self, vector: Self::Vector) -> Self::Vector {
        i8x16_shuffle::<12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27>(
            vector,
            self.zero(),
        )
    }

    /// Requires `simd128`.
    #[inline]
    #[target_feature(enable = "simd128")]
    unsafe fn shift_left_12(&self, vector: Self::Vector) -> Self::Vector {
        i8x16_shuffle::<0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 16, 17, 18, 19>(self.zero(), vector)
    }

    /// Requires `simd128`.
    #[inline]
    #[target_feature(enable = "simd128")]
    unsafe fn carryless_mul_00(&self, a: Self::Vector, b: Self::Vector) -> Self::Vector {
        let product = carryless_mul_u64(u64x2_extract_lane::<0>(a), u64x2_extract_lane::<0>(b));
        self.pack_lanes(product as u64, (product >> 64) as u64)
    }

    /// Requires `simd128`.
    #[inline]
    #[target_feature(enable = "simd128")]
    unsafe fn carryless_mul_01(&self, a: Self::Vector, b: Self::Vector) -> Self::Vector {
        let product = carryless_mul_u64(u64x2_extract_lane::<1>(a), u64x2_extract_lane::<0>(b));
        self.pack_lanes(product as u64, (product >> 64) as u64)
    }

    /// Requires `simd128`.
    #[inline]
    #[target_feature(enable = "simd128")]
    unsafe fn carryless_mul_10(&self, a: Self::Vector, b: Self::Vector) -> Self::Vector {
        let product = carryless_mul_u64(u64x2_extract_lane::<0>(a), u64x2_extract_lane::<1>(b));
        self.pack_lanes(product as u64, (product >> 64) as u64)
    }

    /// Requires `simd128`.
    #[inline]
    #[target_feature(enable = "simd128")]
    unsafe fn carryless_mul_11(&self, a: Self::Vector, b: Self::Vector) -> Self::Vector {
        let product = carryless_mul_u64(u64x2_extract_lane::<1>(a), u64x2_extract_lane::<1>(b));
        self.pack_lanes(product as u64, (product >> 64) as u64)
    }

    /// Requires `simd128`.
    #[inline]
    #[target_feature(enable = "simd128")]
    unsafe fn xor3_vectors(
        &self,
        a: Self::Vector,
        b: Self::Vector,
        c: Self::Vector,
    ) -> Self::Vector {
        // No `xor3` instruction on `simd128`: two `v128.xor`s.
        v128_xor(v128_xor(a, b), c)
    }
}
