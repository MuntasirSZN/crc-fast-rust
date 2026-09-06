// Copyright 2025 Don MacAskill. Licensed under MIT or Apache-2.0 and Zlib.

#![allow(dead_code)]

use crate::consts::{NAME_CRC5_EPC_C1G2, NAME_CRC5_G_704, NAME_CRC5_USB};
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

pub const CRC_5_EPC_C1G2: Algorithm<u8> = Algorithm {
    width: 5,
    poly: 0x09,
    init: 0x09,
    refin: false,
    refout: false,
    xorout: 0x00,
    check: 0x00,
    residue: 0x00,
};

pub const CRC_5_G_704: Algorithm<u8> = Algorithm {
    width: 5,
    poly: 0x15,
    init: 0x00,
    refin: true,
    refout: true,
    xorout: 0x00,
    check: 0x07,
    residue: 0x00,
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

// width=5 poly=0x09 init=0x09 refin=false refout=false xorout=0x00 check=0x00 residue=0x00 name="CRC-5/EPC-C1G2"
pub const CRC5_EPC_C1G2: CrcParams = CrcParams {
    name: NAME_CRC5_EPC_C1G2,
    algorithm: CrcAlgorithm::Crc5EpcC1G2,
    width: 5,
    poly: CRC_5_EPC_C1G2.poly as u64,
    init: CRC_5_EPC_C1G2.init as u64,
    init_algorithm: CRC_5_EPC_C1G2.init as u64,
    refin: CRC_5_EPC_C1G2.refin,
    refout: CRC_5_EPC_C1G2.refout,
    xorout: CRC_5_EPC_C1G2.xorout as u64,
    check: CRC_5_EPC_C1G2.check as u64,
    keys: crate::CrcKeysStorage::from_keys_fold_256(KEYS_09_FORWARD),
};

// width=5 poly=0x15 init=0x00 refin=true refout=true xorout=0x00 check=0x07 residue=0x00 name="CRC-5/G-704"
pub const CRC5_G_704: CrcParams = CrcParams {
    name: NAME_CRC5_G_704,
    algorithm: CrcAlgorithm::Crc5G704,
    width: 5,
    poly: CRC_5_G_704.poly as u64,
    init: CRC_5_G_704.init as u64,
    init_algorithm: CRC_5_G_704.init as u64,
    refin: CRC_5_G_704.refin,
    refout: CRC_5_G_704.refout,
    xorout: CRC_5_G_704.xorout as u64,
    check: CRC_5_G_704.check as u64,
    keys: crate::CrcKeysStorage::from_keys_fold_256(KEYS_15_REFLECTED),
};

// CRC-5/USB - reflected, scaled to 32-bit space: poly 0x05 <<27 | 1<<32 = 0x128000000
// Keys generated via CRC-32 exponents with scaled polynomial
pub const KEYS_05_REFLECTED: [u64; 23] = [
    0x0, 0xa, 0x16, 0x2, 0x14, 0xa, 0x14, 0x85763e69, 0x29, 0x20, 0x8, 0x34, 0x24, 0x3c, 0x26,
    0x18, 0x6, 0x3e, 0x32, 0x38, 0xe, 0x28, 0xa,
];

pub const KEYS_09_FORWARD: [u64; 23] = [
    0x0000000000000000,
    0x6800000000000000,
    0xe800000000000000,
    0x8000000000000000,
    0x9000000000000000,
    0x6800000000000000,
    0x9000000000000000,
    0x000000015d8f9a42,
    0x0000000148000000,
    0x0800000000000000,
    0x2000000000000000,
    0xb000000000000000,
    0x5000000000000000,
    0x7000000000000000,
    0x8800000000000000,
    0xd800000000000000,
    0xb800000000000000,
    0x3000000000000000,
    0xc000000000000000,
    0x7800000000000000,
    0xa800000000000000,
    0x4800000000000000,
    0x6800000000000000,
];

pub const KEYS_15_REFLECTED: [u64; 23] = [
    0x0000000000000000,
    0x0000000000000026,
    0x000000000000002c,
    0x000000000000002a,
    0x0000000000000026,
    0x0000000000000026,
    0x0000000000000034,
    0x00000001c29b8537,
    0x000000000000002b,
    0x000000000000000e,
    0x0000000000000010,
    0x0000000000000002,
    0x000000000000001a,
    0x000000000000001c,
    0x0000000000000020,
    0x0000000000000004,
    0x0000000000000034,
    0x0000000000000038,
    0x0000000000000016,
    0x0000000000000008,
    0x000000000000003e,
    0x0000000000000026,
    0x000000000000002c,
];

// Placeholder - will be replaced by generated values
// This is intentionally computed at build time via generate module; these constants are
// precomputed for performance. If you modify poly, regenerate via `cargo run --features cli --bin get-custom-params` etc.
