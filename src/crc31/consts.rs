// Copyright 2025 Don MacAskill. Licensed under MIT or Apache-2.0 and Zlib.

#![allow(dead_code)]

use crate::consts::NAME_CRC31_PHILIPS;
use crate::structs::Algorithm;
use crate::CrcAlgorithm;
use crate::CrcParams;

// Native CRC-31 algorithm constants matching the CRC catalogue specification
// https://reveng.sourceforge.io/crc-catalogue/all.htm

pub const CRC_31_PHILIPS: Algorithm<u32> = Algorithm {
    width: 31,
    poly: 0x04c11db7,
    init: 0x7fffffff,
    refin: false,
    refout: false,
    xorout: 0x7fffffff,
    check: 0x0ce9e46c,
    residue: 0x4eaf26f1,
};

// width=31 poly=0x04c11db7 init=0x7fffffff refin=false refout=false xorout=0x7fffffff check=0x0ce9e46c residue=0x4eaf26f1 name="CRC-31/PHILIPS"
pub const CRC31_PHILIPS: CrcParams = CrcParams {
    name: NAME_CRC31_PHILIPS,
    algorithm: CrcAlgorithm::Crc31Philips,
    width: 31,
    poly: CRC_31_PHILIPS.poly as u64,
    init: CRC_31_PHILIPS.init as u64,
    init_algorithm: CRC_31_PHILIPS.init as u64,
    refin: CRC_31_PHILIPS.refin,
    refout: CRC_31_PHILIPS.refout,
    xorout: CRC_31_PHILIPS.xorout as u64,
    check: CRC_31_PHILIPS.check as u64,
    keys: crate::CrcKeysStorage::from_keys_fold_256(KEYS_04C11DB7_FORWARD_31),
};

// Keys for CRC-31 forward scaled: poly 0x04c11db7 <<1 | 1<<32 = 0x109823b6e
// Exponents same as CRC32 forward
pub const KEYS_04C11DB7_FORWARD_31: [u64; 23] = [
    0x0,
    0x518d4b8000000000,
    0x835a083200000000,
    0x562822a800000000,
    0xddfa5ac400000000,
    0x518d4b8000000000,
    0x4647972200000000,
    0x109c1009b,
    0x109823b6e,
    0x2f3c79400000000,
    0x8d35e28800000000,
    0x78e8083a00000000,
    0x87095fba00000000,
    0xa9c78e400000000,
    0x6ab10b7600000000,
    0x5cbe635400000000,
    0x1b346d3200000000,
    0xac23448a00000000,
    0xedff437600000000,
    0xc9f2de9a00000000,
    0xc87a083c00000000,
    0xf824afa200000000,
    0xf6ad007e00000000,
];
