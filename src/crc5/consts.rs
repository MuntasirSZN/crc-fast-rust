// Copyright 2025 Don MacAskill. Licensed under MIT or Apache-2.0 and Zlib.

#![allow(dead_code)]

use crate::consts::NAME_CRC5_USB;
use crate::structs::Algorithm;
use crate::CrcAlgorithm;
use crate::CrcParams;

// Native CRC-5 algorithm constants matching the CRC catalogue specification
// https://reveng.sourceforge.io/crc-catalogue/all.htm

pub const CRC_5_USB: Algorithm<u8> = Algorithm {
    width: 5,
    poly: 0x05,
    init: 0x1f,
    refin: true,
    refout: true,
    xorout: 0x1f,
    check: 0x19,
    residue: 0x06,
};

// width=5 poly=0x05 init=0x1f refin=true refout=true xorout=0x1f check=0x19 residue=0x06 name="CRC-5/USB"
pub const CRC5_USB: CrcParams = CrcParams {
    name: NAME_CRC5_USB,
    algorithm: CrcAlgorithm::Crc5Usb,
    width: 5,
    poly: CRC_5_USB.poly as u64,
    init: CRC_5_USB.init as u64,
    init_algorithm: CRC_5_USB.init as u64,
    refin: CRC_5_USB.refin,
    refout: CRC_5_USB.refout,
    xorout: CRC_5_USB.xorout as u64,
    check: CRC_5_USB.check as u64,
    keys: crate::CrcKeysStorage::from_keys_fold_256(KEYS_05_REFLECTED),
};

// CRC-5/USB - reflected, scaled to 32-bit space: poly 0x05 <<27 | 1<<32 = 0x128000000
// Keys generated via CRC-32 exponents with scaled polynomial
pub const KEYS_05_REFLECTED: [u64; 23] = [
    0x0, 0xa, 0x16, 0x2, 0x14, 0xa, 0x14, 0x85763e69, 0x29, 0x20, 0x8, 0x34, 0x24, 0x3c, 0x26,
    0x18, 0x6, 0x3e, 0x32, 0x38, 0xe, 0x28, 0xa,
];

// Placeholder - will be replaced by generated values
// This is intentionally computed at build time via generate module; these constants are
// precomputed for performance. If you modify poly, regenerate via `cargo run --features cli --bin get-custom-params` etc.
