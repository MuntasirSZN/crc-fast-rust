// Copyright 2025 Don MacAskill. Licensed under MIT or Apache-2.0 and Zlib.

#![allow(dead_code)]

use crate::traits::{CrcCalculator, CrcWidth};
use crate::{arch, cache, CrcAlgorithm, CrcParams};

/// CRC algorithm parameters matching the CRC catalogue specification.
///
/// This struct describes a CRC algorithm using the fields specified by the
/// [Catalogue of parametrised CRC algorithms](https://reveng.sourceforge.io/crc-catalogue/all.htm).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Algorithm<W> {
    /// The number of bit cells in the linear feedback shift register; the degree of the generator
    /// polynomial, minus one.
    pub width: u8,
    /// The generator polynomial that sets the feedback tap positions of the shift register.
    pub poly: W,
    /// The settings of the bit cells at the start of each calculation, before reading the first
    /// message bit.
    pub init: W,
    /// If `true`, characters are read bit-by-bit, least significant bit (LSB) first;
    /// if `false`, most significant bit (MSB) first.
    pub refin: bool,
    /// If `true`, the contents of the register after reading the last message bit are reflected
    /// before presentation; if `false`, they are unreflected.
    pub refout: bool,
    /// The XOR value applied to the contents of the register after the last message bit has been
    /// read and after the optional reflection.
    pub xorout: W,
    /// The contents of the register after initialising, reading the UTF-8 string `"123456789"`,
    /// optionally reflecting, and applying the final XOR.
    pub check: W,
    /// The contents of the register after initialising, reading an error-free codeword and
    /// optionally reflecting the register, but not applying the final XOR.
    pub residue: W,
}

/// CRC-5 width implementation
#[derive(Clone, Copy)]
pub struct Width5;

impl CrcWidth for Width5 {
    const WIDTH: u32 = 5;
    type Value = u8;
}

/// CRC-16 width implementation
#[derive(Clone, Copy)]
pub struct Width16;

impl CrcWidth for Width16 {
    const WIDTH: u32 = 16;
    type Value = u16;
}

/// CRC-31 width implementation
#[derive(Clone, Copy)]
pub struct Width31;

impl CrcWidth for Width31 {
    const WIDTH: u32 = 31;
    type Value = u32;
}

/// CRC-32 width implementation
#[derive(Clone, Copy)]
pub struct Width32;

impl CrcWidth for Width32 {
    const WIDTH: u32 = 32;
    type Value = u32;
}

/// CRC-64 width implementation
#[derive(Clone, Copy)]
pub struct Width64;

impl CrcWidth for Width64 {
    const WIDTH: u32 = 64;
    type Value = u64;
}

/// CRC State wrapper to manage the SIMD operations and reflection mode
#[derive(Debug, Clone, Copy)]
pub struct CrcState<T> {
    pub value: T,
    pub reflected: bool,
}

pub(crate) struct Calculator {}

impl CrcCalculator for Calculator {
    #[inline(always)]
    fn calculate(state: u64, data: &[u8], params: &CrcParams) -> u64 {
        unsafe { arch::update(state, data, params) }
    }
}

impl CrcParams {
    /// Fallible constructor for custom CRC parameters (panic-free, uses `exn`).
    ///
    /// Returns `Err(Exn<UnsupportedWidth>)` if `width` is not 16, 32, or 64.
    #[cfg(feature = "alloc")]
    pub fn try_new(
        name: &'static str,
        width: u8,
        poly: u64,
        init: u64,
        reflected: bool,
        xorout: u64,
        check: u64,
    ) -> exn::Result<Self, crate::error::UnsupportedWidth> {
        if width != 5 && width != 16 && width != 31 && width != 32 && width != 64 {
            exn::bail!(crate::error::UnsupportedWidth(width));
        }
        let keys_array = cache::get_or_generate_keys(width, poly, reflected);
        let keys = crate::CrcKeysStorage::from_keys_fold_256(keys_array);

        let init_algorithm = if reflected {
            match width {
                5 => {
                    let mut rev = 0u8;
                    let init_u8 = init as u8;
                    for i in 0..5 {
                        if (init_u8 >> i) & 1 == 1 {
                            rev |= 1 << (4 - i);
                        }
                    }
                    rev as u64
                }
                16 => (init as u16).reverse_bits() as u64,
                _ => init,
            }
        } else {
            init
        };

        Ok(Self {
            algorithm: CrcAlgorithm::CrcCustom,
            name,
            width,
            poly,
            init,
            init_algorithm,
            refin: reflected,
            refout: reflected,
            xorout,
            check,
            keys,
        })
    }

    /// Creates custom CRC parameters for a given set of Rocksoft CRC parameters.
    ///
    /// Uses an internal cache to avoid regenerating folding keys for identical parameter sets.
    /// The first call with a given set of parameters will generate and cache the keys, while
    /// subsequent calls with the same parameters will use the cached keys for optimal performance.
    ///
    /// Does not support mis-matched refin/refout parameters, so both must be true or both false.
    ///
    /// Rocksoft parameters for lots of variants: https://reveng.sourceforge.io/crc-catalogue/all.htm
    ///
    /// # Panic-free
    ///
    /// This function is now panic-free. For unsupported widths it returns a best-effort
    /// params (with zeroed keys) and relies on `try_new` for proper error handling.
    pub fn new(
        name: &'static str,
        width: u8,
        poly: u64,
        init: u64,
        reflected: bool,
        xorout: u64,
        check: u64,
    ) -> Self {
        // Validate width is supported (panic-free: fallback to try_new, and on error create dummy)
        if width != 5 && width != 16 && width != 31 && width != 32 && width != 64 {
            // Keep backwards compat but panic-free: create dummy with zero keys
            // Caller should use `try_new` to get proper `Exn` error.
            let keys = crate::CrcKeysStorage::from_keys_fold_256([0; 23]);
            let init_algorithm = if reflected {
                match width {
                    5 => {
                        let mut rev = 0u8;
                        let init_u8 = init as u8;
                        for i in 0..5 {
                            if (init_u8 >> i) & 1 == 1 {
                                rev |= 1 << (4 - i);
                            }
                        }
                        rev as u64
                    }
                    16 => (init as u16).reverse_bits() as u64,
                    _ => init,
                }
            } else {
                init
            };
            return Self {
                algorithm: CrcAlgorithm::CrcCustom,
                name,
                width,
                poly,
                init,
                init_algorithm,
                refin: reflected,
                refout: reflected,
                xorout,
                check,
                keys,
            };
        }
        let keys_array = cache::get_or_generate_keys(width, poly, reflected);
        let keys = crate::CrcKeysStorage::from_keys_fold_256(keys_array);

        // For reflected CRCs, bit-reverse the init value for the SIMD algorithm
        let init_algorithm = if reflected {
            match width {
                5 => {
                    let mut rev = 0u8;
                    let init_u8 = init as u8;
                    for i in 0..5 {
                        if (init_u8 >> i) & 1 == 1 {
                            rev |= 1 << (4 - i);
                        }
                    }
                    rev as u64
                }
                16 => (init as u16).reverse_bits() as u64,
                _ => init,
            }
        } else {
            init
        };

        Self {
            algorithm: CrcAlgorithm::CrcCustom,
            name,
            width,
            poly,
            init,
            init_algorithm,
            refin: reflected,
            refout: reflected,
            xorout,
            check,
            keys,
        }
    }

    /// Gets a key at the specified index, returning 0 if out of bounds.
    /// This provides safe access regardless of internal key storage format.
    #[inline(always)]
    pub fn get_key(&self, index: usize) -> u64 {
        self.keys.get_key(index)
    }

    /// Gets a key at the specified index, returning None if out of bounds.
    /// This provides optional key access for cases where bounds checking is needed.
    #[inline(always)]
    pub fn get_key_checked(&self, index: usize) -> Option<u64> {
        if index < self.keys.key_count() {
            Some(self.keys.get_key(index))
        } else {
            None
        }
    }

    /// Returns the number of keys available in this CrcParams instance.
    #[inline(always)]
    pub fn key_count(&self) -> usize {
        self.keys.key_count()
    }
}
