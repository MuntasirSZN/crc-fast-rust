// Copyright 2025 Don MacAskill. Licensed under MIT or Apache-2.0 and Zlib.

//! `wasm32` manual-SIMD backend (`simd128`, stable since 1.54).

#![cfg(target_arch = "wasm32")]

pub mod simd128;
