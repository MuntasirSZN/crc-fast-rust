// Copyright 2025 Don MacAskill. Licensed under MIT or Apache-2.0 and Zlib.

#![allow(dead_code)]

use crate::consts::{
    NAME_CRC8_AUTOSAR, NAME_CRC8_BLUETOOTH, NAME_CRC8_CDMA2000, NAME_CRC8_DARC, NAME_CRC8_DVB_S2,
    NAME_CRC8_GSM_A, NAME_CRC8_GSM_B, NAME_CRC8_HITAG, NAME_CRC8_I_432_1, NAME_CRC8_I_CODE,
    NAME_CRC8_LTE, NAME_CRC8_MAXIM_DOW, NAME_CRC8_MIFARE_MAD, NAME_CRC8_NRSC_5,
    NAME_CRC8_OPENSAFETY, NAME_CRC8_ROHC, NAME_CRC8_SAE_J1850, NAME_CRC8_SMBUS,
    NAME_CRC8_TECH_3250, NAME_CRC8_WCDMA,
};
use crate::structs::Algorithm;
use crate::CrcAlgorithm;
use crate::CrcParams;

// Native CRC-8 algorithm constants matching the CRC catalogue specification
// https://reveng.sourceforge.io/crc-catalogue/all.htm

pub const CRC_8_SMBUS: Algorithm<u8> = Algorithm {
    width: 8,
    poly: 0x07,
    init: 0x00,
    refin: false,
    refout: false,
    xorout: 0x00,
    check: 0xf4,
    residue: 0x00,
};

pub const CRC_8_I_432_1: Algorithm<u8> = Algorithm {
    width: 8,
    poly: 0x07,
    init: 0x00,
    refin: false,
    refout: false,
    xorout: 0x55,
    check: 0xa1,
    residue: 0xac,
};

pub const CRC_8_ROHC: Algorithm<u8> = Algorithm {
    width: 8,
    poly: 0x07,
    init: 0xff,
    refin: true,
    refout: true,
    xorout: 0x00,
    check: 0xd0,
    residue: 0x00,
};

pub const CRC_8_GSM_A: Algorithm<u8> = Algorithm {
    width: 8,
    poly: 0x1d,
    init: 0x00,
    refin: false,
    refout: false,
    xorout: 0x00,
    check: 0x37,
    residue: 0x00,
};

pub const CRC_8_MIFARE_MAD: Algorithm<u8> = Algorithm {
    width: 8,
    poly: 0x1d,
    init: 0xc7,
    refin: false,
    refout: false,
    xorout: 0x00,
    check: 0x99,
    residue: 0x00,
};

pub const CRC_8_I_CODE: Algorithm<u8> = Algorithm {
    width: 8,
    poly: 0x1d,
    init: 0xfd,
    refin: false,
    refout: false,
    xorout: 0x00,
    check: 0x7e,
    residue: 0x00,
};

pub const CRC_8_HITAG: Algorithm<u8> = Algorithm {
    width: 8,
    poly: 0x1d,
    init: 0xff,
    refin: false,
    refout: false,
    xorout: 0x00,
    check: 0xb4,
    residue: 0x00,
};

pub const CRC_8_SAE_J1850: Algorithm<u8> = Algorithm {
    width: 8,
    poly: 0x1d,
    init: 0xff,
    refin: false,
    refout: false,
    xorout: 0xff,
    check: 0x4b,
    residue: 0xc4,
};

pub const CRC_8_TECH_3250: Algorithm<u8> = Algorithm {
    width: 8,
    poly: 0x1d,
    init: 0xff,
    refin: true,
    refout: true,
    xorout: 0x00,
    check: 0x97,
    residue: 0x00,
};

pub const CRC_8_OPENSAFETY: Algorithm<u8> = Algorithm {
    width: 8,
    poly: 0x2f,
    init: 0x00,
    refin: false,
    refout: false,
    xorout: 0x00,
    check: 0x3e,
    residue: 0x00,
};

pub const CRC_8_AUTOSAR: Algorithm<u8> = Algorithm {
    width: 8,
    poly: 0x2f,
    init: 0xff,
    refin: false,
    refout: false,
    xorout: 0xff,
    check: 0xdf,
    residue: 0x42,
};

pub const CRC_8_MAXIM_DOW: Algorithm<u8> = Algorithm {
    width: 8,
    poly: 0x31,
    init: 0x00,
    refin: true,
    refout: true,
    xorout: 0x00,
    check: 0xa1,
    residue: 0x00,
};

pub const CRC_8_NRSC_5: Algorithm<u8> = Algorithm {
    width: 8,
    poly: 0x31,
    init: 0xff,
    refin: false,
    refout: false,
    xorout: 0x00,
    check: 0xf7,
    residue: 0x00,
};

pub const CRC_8_DARC: Algorithm<u8> = Algorithm {
    width: 8,
    poly: 0x39,
    init: 0x00,
    refin: true,
    refout: true,
    xorout: 0x00,
    check: 0x15,
    residue: 0x00,
};

pub const CRC_8_GSM_B: Algorithm<u8> = Algorithm {
    width: 8,
    poly: 0x49,
    init: 0x00,
    refin: false,
    refout: false,
    xorout: 0xff,
    check: 0x94,
    residue: 0x53,
};

pub const CRC_8_LTE: Algorithm<u8> = Algorithm {
    width: 8,
    poly: 0x9b,
    init: 0x00,
    refin: false,
    refout: false,
    xorout: 0x00,
    check: 0xea,
    residue: 0x00,
};

pub const CRC_8_WCDMA: Algorithm<u8> = Algorithm {
    width: 8,
    poly: 0x9b,
    init: 0x00,
    refin: true,
    refout: true,
    xorout: 0x00,
    check: 0x25,
    residue: 0x00,
};

pub const CRC_8_CDMA2000: Algorithm<u8> = Algorithm {
    width: 8,
    poly: 0x9b,
    init: 0xff,
    refin: false,
    refout: false,
    xorout: 0x00,
    check: 0xda,
    residue: 0x00,
};

pub const CRC_8_BLUETOOTH: Algorithm<u8> = Algorithm {
    width: 8,
    poly: 0xa7,
    init: 0x00,
    refin: true,
    refout: true,
    xorout: 0x00,
    check: 0x26,
    residue: 0x00,
};

pub const CRC_8_DVB_S2: Algorithm<u8> = Algorithm {
    width: 8,
    poly: 0xd5,
    init: 0x00,
    refin: false,
    refout: false,
    xorout: 0x00,
    check: 0xbc,
    residue: 0x00,
};

// width=8 poly=0x07 init=0x00 refin=false refout=false xorout=0x00 check=0xf4 residue=0x00 name="CRC-8/SMBUS"
pub const CRC8_SMBUS: CrcParams = CrcParams {
    name: NAME_CRC8_SMBUS,
    algorithm: CrcAlgorithm::Crc8Smbus,
    width: 8,
    poly: CRC_8_SMBUS.poly as u64,
    init: CRC_8_SMBUS.init as u64,
    init_algorithm: CRC_8_SMBUS.init as u64,
    refin: CRC_8_SMBUS.refin,
    refout: CRC_8_SMBUS.refout,
    xorout: CRC_8_SMBUS.xorout as u64,
    check: CRC_8_SMBUS.check as u64,
    keys: crate::CrcKeysStorage::from_keys_fold_256(KEYS_07_FORWARD),
};

// width=8 poly=0x07 init=0x00 refin=false refout=false xorout=0x55 check=0xa1 residue=0xac name="CRC-8/I-432-1"
pub const CRC8_I_432_1: CrcParams = CrcParams {
    name: NAME_CRC8_I_432_1,
    algorithm: CrcAlgorithm::Crc8I4321,
    width: 8,
    poly: CRC_8_I_432_1.poly as u64,
    init: CRC_8_I_432_1.init as u64,
    init_algorithm: CRC_8_I_432_1.init as u64,
    refin: CRC_8_I_432_1.refin,
    refout: CRC_8_I_432_1.refout,
    xorout: CRC_8_I_432_1.xorout as u64,
    check: CRC_8_I_432_1.check as u64,
    keys: crate::CrcKeysStorage::from_keys_fold_256(KEYS_07_FORWARD),
};

// width=8 poly=0x07 init=0xff refin=true refout=true xorout=0x00 check=0xd0 residue=0x00 name="CRC-8/ROHC"
pub const CRC8_ROHC: CrcParams = CrcParams {
    name: NAME_CRC8_ROHC,
    algorithm: CrcAlgorithm::Crc8Rohc,
    width: 8,
    poly: CRC_8_ROHC.poly as u64,
    init: CRC_8_ROHC.init as u64,
    init_algorithm: CRC_8_ROHC.init as u64,
    refin: CRC_8_ROHC.refin,
    refout: CRC_8_ROHC.refout,
    xorout: CRC_8_ROHC.xorout as u64,
    check: CRC_8_ROHC.check as u64,
    keys: crate::CrcKeysStorage::from_keys_fold_256(KEYS_07_REFLECTED),
};

// width=8 poly=0x1d init=0x00 refin=false refout=false xorout=0x00 check=0x37 residue=0x00 name="CRC-8/GSM-A"
pub const CRC8_GSM_A: CrcParams = CrcParams {
    name: NAME_CRC8_GSM_A,
    algorithm: CrcAlgorithm::Crc8GsmA,
    width: 8,
    poly: CRC_8_GSM_A.poly as u64,
    init: CRC_8_GSM_A.init as u64,
    init_algorithm: CRC_8_GSM_A.init as u64,
    refin: CRC_8_GSM_A.refin,
    refout: CRC_8_GSM_A.refout,
    xorout: CRC_8_GSM_A.xorout as u64,
    check: CRC_8_GSM_A.check as u64,
    keys: crate::CrcKeysStorage::from_keys_fold_256(KEYS_1D_FORWARD),
};

// width=8 poly=0x1d init=0xc7 refin=false refout=false xorout=0x00 check=0x99 residue=0x00 name="CRC-8/MIFARE-MAD"
pub const CRC8_MIFARE_MAD: CrcParams = CrcParams {
    name: NAME_CRC8_MIFARE_MAD,
    algorithm: CrcAlgorithm::Crc8MifareMad,
    width: 8,
    poly: CRC_8_MIFARE_MAD.poly as u64,
    init: CRC_8_MIFARE_MAD.init as u64,
    init_algorithm: CRC_8_MIFARE_MAD.init as u64,
    refin: CRC_8_MIFARE_MAD.refin,
    refout: CRC_8_MIFARE_MAD.refout,
    xorout: CRC_8_MIFARE_MAD.xorout as u64,
    check: CRC_8_MIFARE_MAD.check as u64,
    keys: crate::CrcKeysStorage::from_keys_fold_256(KEYS_1D_FORWARD),
};

// width=8 poly=0x1d init=0xfd refin=false refout=false xorout=0x00 check=0x7e residue=0x00 name="CRC-8/I-CODE"
pub const CRC8_I_CODE: CrcParams = CrcParams {
    name: NAME_CRC8_I_CODE,
    algorithm: CrcAlgorithm::Crc8ICode,
    width: 8,
    poly: CRC_8_I_CODE.poly as u64,
    init: CRC_8_I_CODE.init as u64,
    init_algorithm: CRC_8_I_CODE.init as u64,
    refin: CRC_8_I_CODE.refin,
    refout: CRC_8_I_CODE.refout,
    xorout: CRC_8_I_CODE.xorout as u64,
    check: CRC_8_I_CODE.check as u64,
    keys: crate::CrcKeysStorage::from_keys_fold_256(KEYS_1D_FORWARD),
};

// width=8 poly=0x1d init=0xff refin=false refout=false xorout=0x00 check=0xb4 residue=0x00 name="CRC-8/HITAG"
pub const CRC8_HITAG: CrcParams = CrcParams {
    name: NAME_CRC8_HITAG,
    algorithm: CrcAlgorithm::Crc8Hitag,
    width: 8,
    poly: CRC_8_HITAG.poly as u64,
    init: CRC_8_HITAG.init as u64,
    init_algorithm: CRC_8_HITAG.init as u64,
    refin: CRC_8_HITAG.refin,
    refout: CRC_8_HITAG.refout,
    xorout: CRC_8_HITAG.xorout as u64,
    check: CRC_8_HITAG.check as u64,
    keys: crate::CrcKeysStorage::from_keys_fold_256(KEYS_1D_FORWARD),
};

// width=8 poly=0x1d init=0xff refin=false refout=false xorout=0xff check=0x4b residue=0xc4 name="CRC-8/SAE-J1850"
pub const CRC8_SAE_J1850: CrcParams = CrcParams {
    name: NAME_CRC8_SAE_J1850,
    algorithm: CrcAlgorithm::Crc8SaeJ1850,
    width: 8,
    poly: CRC_8_SAE_J1850.poly as u64,
    init: CRC_8_SAE_J1850.init as u64,
    init_algorithm: CRC_8_SAE_J1850.init as u64,
    refin: CRC_8_SAE_J1850.refin,
    refout: CRC_8_SAE_J1850.refout,
    xorout: CRC_8_SAE_J1850.xorout as u64,
    check: CRC_8_SAE_J1850.check as u64,
    keys: crate::CrcKeysStorage::from_keys_fold_256(KEYS_1D_FORWARD),
};

// width=8 poly=0x1d init=0xff refin=true refout=true xorout=0x00 check=0x97 residue=0x00 name="CRC-8/TECH-3250"
pub const CRC8_TECH_3250: CrcParams = CrcParams {
    name: NAME_CRC8_TECH_3250,
    algorithm: CrcAlgorithm::Crc8Tech3250,
    width: 8,
    poly: CRC_8_TECH_3250.poly as u64,
    init: CRC_8_TECH_3250.init as u64,
    init_algorithm: CRC_8_TECH_3250.init as u64,
    refin: CRC_8_TECH_3250.refin,
    refout: CRC_8_TECH_3250.refout,
    xorout: CRC_8_TECH_3250.xorout as u64,
    check: CRC_8_TECH_3250.check as u64,
    keys: crate::CrcKeysStorage::from_keys_fold_256(KEYS_1D_REFLECTED),
};

// width=8 poly=0x2f init=0x00 refin=false refout=false xorout=0x00 check=0x3e residue=0x00 name="CRC-8/OPENSAFETY"
pub const CRC8_OPENSAFETY: CrcParams = CrcParams {
    name: NAME_CRC8_OPENSAFETY,
    algorithm: CrcAlgorithm::Crc8Opensafety,
    width: 8,
    poly: CRC_8_OPENSAFETY.poly as u64,
    init: CRC_8_OPENSAFETY.init as u64,
    init_algorithm: CRC_8_OPENSAFETY.init as u64,
    refin: CRC_8_OPENSAFETY.refin,
    refout: CRC_8_OPENSAFETY.refout,
    xorout: CRC_8_OPENSAFETY.xorout as u64,
    check: CRC_8_OPENSAFETY.check as u64,
    keys: crate::CrcKeysStorage::from_keys_fold_256(KEYS_2F_FORWARD),
};

// width=8 poly=0x2f init=0xff refin=false refout=false xorout=0xff check=0xdf residue=0x42 name="CRC-8/AUTOSAR"
pub const CRC8_AUTOSAR: CrcParams = CrcParams {
    name: NAME_CRC8_AUTOSAR,
    algorithm: CrcAlgorithm::Crc8Autosar,
    width: 8,
    poly: CRC_8_AUTOSAR.poly as u64,
    init: CRC_8_AUTOSAR.init as u64,
    init_algorithm: CRC_8_AUTOSAR.init as u64,
    refin: CRC_8_AUTOSAR.refin,
    refout: CRC_8_AUTOSAR.refout,
    xorout: CRC_8_AUTOSAR.xorout as u64,
    check: CRC_8_AUTOSAR.check as u64,
    keys: crate::CrcKeysStorage::from_keys_fold_256(KEYS_2F_FORWARD),
};

// width=8 poly=0x31 init=0x00 refin=true refout=true xorout=0x00 check=0xa1 residue=0x00 name="CRC-8/MAXIM-DOW"
pub const CRC8_MAXIM_DOW: CrcParams = CrcParams {
    name: NAME_CRC8_MAXIM_DOW,
    algorithm: CrcAlgorithm::Crc8MaximDow,
    width: 8,
    poly: CRC_8_MAXIM_DOW.poly as u64,
    init: CRC_8_MAXIM_DOW.init as u64,
    init_algorithm: CRC_8_MAXIM_DOW.init as u64,
    refin: CRC_8_MAXIM_DOW.refin,
    refout: CRC_8_MAXIM_DOW.refout,
    xorout: CRC_8_MAXIM_DOW.xorout as u64,
    check: CRC_8_MAXIM_DOW.check as u64,
    keys: crate::CrcKeysStorage::from_keys_fold_256(KEYS_31_REFLECTED),
};

// width=8 poly=0x31 init=0xff refin=false refout=false xorout=0x00 check=0xf7 residue=0x00 name="CRC-8/NRSC-5"
pub const CRC8_NRSC_5: CrcParams = CrcParams {
    name: NAME_CRC8_NRSC_5,
    algorithm: CrcAlgorithm::Crc8Nrsc5,
    width: 8,
    poly: CRC_8_NRSC_5.poly as u64,
    init: CRC_8_NRSC_5.init as u64,
    init_algorithm: CRC_8_NRSC_5.init as u64,
    refin: CRC_8_NRSC_5.refin,
    refout: CRC_8_NRSC_5.refout,
    xorout: CRC_8_NRSC_5.xorout as u64,
    check: CRC_8_NRSC_5.check as u64,
    keys: crate::CrcKeysStorage::from_keys_fold_256(KEYS_31_FORWARD),
};

// width=8 poly=0x39 init=0x00 refin=true refout=true xorout=0x00 check=0x15 residue=0x00 name="CRC-8/DARC"
pub const CRC8_DARC: CrcParams = CrcParams {
    name: NAME_CRC8_DARC,
    algorithm: CrcAlgorithm::Crc8Darc,
    width: 8,
    poly: CRC_8_DARC.poly as u64,
    init: CRC_8_DARC.init as u64,
    init_algorithm: CRC_8_DARC.init as u64,
    refin: CRC_8_DARC.refin,
    refout: CRC_8_DARC.refout,
    xorout: CRC_8_DARC.xorout as u64,
    check: CRC_8_DARC.check as u64,
    keys: crate::CrcKeysStorage::from_keys_fold_256(KEYS_39_REFLECTED),
};

// width=8 poly=0x49 init=0x00 refin=false refout=false xorout=0xff check=0x94 residue=0x53 name="CRC-8/GSM-B"
pub const CRC8_GSM_B: CrcParams = CrcParams {
    name: NAME_CRC8_GSM_B,
    algorithm: CrcAlgorithm::Crc8GsmB,
    width: 8,
    poly: CRC_8_GSM_B.poly as u64,
    init: CRC_8_GSM_B.init as u64,
    init_algorithm: CRC_8_GSM_B.init as u64,
    refin: CRC_8_GSM_B.refin,
    refout: CRC_8_GSM_B.refout,
    xorout: CRC_8_GSM_B.xorout as u64,
    check: CRC_8_GSM_B.check as u64,
    keys: crate::CrcKeysStorage::from_keys_fold_256(KEYS_49_FORWARD),
};

// width=8 poly=0x9b init=0x00 refin=false refout=false xorout=0x00 check=0xea residue=0x00 name="CRC-8/LTE"
pub const CRC8_LTE: CrcParams = CrcParams {
    name: NAME_CRC8_LTE,
    algorithm: CrcAlgorithm::Crc8Lte,
    width: 8,
    poly: CRC_8_LTE.poly as u64,
    init: CRC_8_LTE.init as u64,
    init_algorithm: CRC_8_LTE.init as u64,
    refin: CRC_8_LTE.refin,
    refout: CRC_8_LTE.refout,
    xorout: CRC_8_LTE.xorout as u64,
    check: CRC_8_LTE.check as u64,
    keys: crate::CrcKeysStorage::from_keys_fold_256(KEYS_9B_FORWARD),
};

// width=8 poly=0x9b init=0x00 refin=true refout=true xorout=0x00 check=0x25 residue=0x00 name="CRC-8/WCDMA"
pub const CRC8_WCDMA: CrcParams = CrcParams {
    name: NAME_CRC8_WCDMA,
    algorithm: CrcAlgorithm::Crc8Wcdma,
    width: 8,
    poly: CRC_8_WCDMA.poly as u64,
    init: CRC_8_WCDMA.init as u64,
    init_algorithm: CRC_8_WCDMA.init as u64,
    refin: CRC_8_WCDMA.refin,
    refout: CRC_8_WCDMA.refout,
    xorout: CRC_8_WCDMA.xorout as u64,
    check: CRC_8_WCDMA.check as u64,
    keys: crate::CrcKeysStorage::from_keys_fold_256(KEYS_9B_REFLECTED),
};

// width=8 poly=0x9b init=0xff refin=false refout=false xorout=0x00 check=0xda residue=0x00 name="CRC-8/CDMA2000"
pub const CRC8_CDMA2000: CrcParams = CrcParams {
    name: NAME_CRC8_CDMA2000,
    algorithm: CrcAlgorithm::Crc8Cdma2000,
    width: 8,
    poly: CRC_8_CDMA2000.poly as u64,
    init: CRC_8_CDMA2000.init as u64,
    init_algorithm: CRC_8_CDMA2000.init as u64,
    refin: CRC_8_CDMA2000.refin,
    refout: CRC_8_CDMA2000.refout,
    xorout: CRC_8_CDMA2000.xorout as u64,
    check: CRC_8_CDMA2000.check as u64,
    keys: crate::CrcKeysStorage::from_keys_fold_256(KEYS_9B_FORWARD),
};

// width=8 poly=0xa7 init=0x00 refin=true refout=true xorout=0x00 check=0x26 residue=0x00 name="CRC-8/BLUETOOTH"
pub const CRC8_BLUETOOTH: CrcParams = CrcParams {
    name: NAME_CRC8_BLUETOOTH,
    algorithm: CrcAlgorithm::Crc8Bluetooth,
    width: 8,
    poly: CRC_8_BLUETOOTH.poly as u64,
    init: CRC_8_BLUETOOTH.init as u64,
    init_algorithm: CRC_8_BLUETOOTH.init as u64,
    refin: CRC_8_BLUETOOTH.refin,
    refout: CRC_8_BLUETOOTH.refout,
    xorout: CRC_8_BLUETOOTH.xorout as u64,
    check: CRC_8_BLUETOOTH.check as u64,
    keys: crate::CrcKeysStorage::from_keys_fold_256(KEYS_A7_REFLECTED),
};

// width=8 poly=0xd5 init=0x00 refin=false refout=false xorout=0x00 check=0xbc residue=0x00 name="CRC-8/DVB-S2"
pub const CRC8_DVB_S2: CrcParams = CrcParams {
    name: NAME_CRC8_DVB_S2,
    algorithm: CrcAlgorithm::Crc8DvbS2,
    width: 8,
    poly: CRC_8_DVB_S2.poly as u64,
    init: CRC_8_DVB_S2.init as u64,
    init_algorithm: CRC_8_DVB_S2.init as u64,
    refin: CRC_8_DVB_S2.refin,
    refout: CRC_8_DVB_S2.refout,
    xorout: CRC_8_DVB_S2.xorout as u64,
    check: CRC_8_DVB_S2.check as u64,
    keys: crate::CrcKeysStorage::from_keys_fold_256(KEYS_D5_FORWARD),
};

// Shared folding keys per (poly, reflected) — generated via `get-custom-params`
pub const KEYS_07_FORWARD: [u64; 23] = [
    0x0000000000000000,
    0x7900000000000000,
    0x0e00000000000000,
    0x3400000000000000,
    0x1500000000000000,
    0x7900000000000000,
    0x6200000000000000,
    0x0000000107156a16,
    0x0000000107000000,
    0x1a00000000000000,
    0x8900000000000000,
    0x0d00000000000000,
    0xc700000000000000,
    0x8500000000000000,
    0xe000000000000000,
    0xc100000000000000,
    0x7000000000000000,
    0xe300000000000000,
    0x3800000000000000,
    0xf200000000000000,
    0x1c00000000000000,
    0x8c00000000000000,
    0x6b00000000000000,
];

pub const KEYS_07_REFLECTED: [u64; 23] = [
    0x0000000000000000,
    0x000000000000013c,
    0x00000000000000e0,
    0x0000000000000058,
    0x0000000000000150,
    0x000000000000013c,
    0x000000000000008c,
    0x00000000d0ad51c1,
    0x00000000000001c1,
    0x00000000000000b0,
    0x0000000000000122,
    0x0000000000000160,
    0x00000000000001c6,
    0x0000000000000142,
    0x000000000000000e,
    0x0000000000000106,
    0x000000000000001c,
    0x000000000000018e,
    0x0000000000000038,
    0x000000000000009e,
    0x0000000000000070,
    0x0000000000000062,
    0x00000000000001ac,
];

pub const KEYS_1D_FORWARD: [u64; 23] = [
    0x0000000000000000,
    0x6500000000000000,
    0x4f00000000000000,
    0xe000000000000000,
    0xcd00000000000000,
    0x6500000000000000,
    0x6a00000000000000,
    0x000000011c4b8192,
    0x000000011d000000,
    0x0f00000000000000,
    0x4200000000000000,
    0x7000000000000000,
    0xe800000000000000,
    0x8900000000000000,
    0x2100000000000000,
    0x3800000000000000,
    0x7400000000000000,
    0xca00000000000000,
    0x9e00000000000000,
    0x1c00000000000000,
    0x3a00000000000000,
    0xa600000000000000,
    0x4c00000000000000,
];

pub const KEYS_1D_REFLECTED: [u64; 23] = [
    0x0000000000000000,
    0x000000000000014c,
    0x00000000000001e4,
    0x000000000000000e,
    0x0000000000000166,
    0x000000000000014c,
    0x00000000000000ac,
    0x000000009303a471,
    0x0000000000000171,
    0x00000000000001e0,
    0x0000000000000084,
    0x000000000000001c,
    0x000000000000002e,
    0x0000000000000122,
    0x0000000000000108,
    0x0000000000000038,
    0x000000000000005c,
    0x00000000000000a6,
    0x00000000000000f2,
    0x0000000000000070,
    0x00000000000000b8,
    0x00000000000000ca,
    0x0000000000000064,
];

pub const KEYS_2F_FORWARD: [u64; 23] = [
    0x0000000000000000,
    0x2900000000000000,
    0x5e00000000000000,
    0x9200000000000000,
    0xe900000000000000,
    0x2900000000000000,
    0xad00000000000000,
    0x000000012bf20fa7,
    0x000000012f000000,
    0x4900000000000000,
    0xe300000000000000,
    0xb300000000000000,
    0xe600000000000000,
    0xce00000000000000,
    0x7300000000000000,
    0x6700000000000000,
    0xae00000000000000,
    0xa400000000000000,
    0x5700000000000000,
    0x5200000000000000,
    0xbc00000000000000,
    0x1300000000000000,
    0x0e00000000000000,
];

pub const KEYS_31_REFLECTED: [u64; 23] = [
    0x0000000000000000,
    0x000000000000015a,
    0x000000000000008c,
    0x0000000000000148,
    0x000000000000005e,
    0x000000000000015a,
    0x0000000000000196,
    0x0000000183a4ce59,
    0x0000000000000119,
    0x00000000000000a2,
    0x00000000000000bc,
    0x0000000000000144,
    0x0000000000000178,
    0x00000000000000ba,
    0x00000000000000c2,
    0x0000000000000174,
    0x0000000000000184,
    0x00000000000000da,
    0x000000000000013a,
    0x00000000000001b4,
    0x0000000000000046,
    0x000000000000019c,
    0x00000000000000c4,
];

pub const KEYS_31_FORWARD: [u64; 23] = [
    0x0000000000000000,
    0xb500000000000000,
    0x6200000000000000,
    0x2500000000000000,
    0xf400000000000000,
    0xb500000000000000,
    0xd300000000000000,
    0x0000000134e64b83,
    0x0000000131000000,
    0x8a00000000000000,
    0x7a00000000000000,
    0x4500000000000000,
    0x3d00000000000000,
    0xba00000000000000,
    0x8600000000000000,
    0x5d00000000000000,
    0x4300000000000000,
    0xb600000000000000,
    0xb900000000000000,
    0x5b00000000000000,
    0xc400000000000000,
    0x7300000000000000,
    0x4600000000000000,
];

pub const KEYS_39_REFLECTED: [u64; 23] = [
    0x0000000000000000,
    0x0000000000000010,
    0x0000000000000100,
    0x0000000000000072,
    0x00000000000001b6,
    0x0000000000000010,
    0x0000000000000004,
    0x0000000004f20279,
    0x0000000000000139,
    0x0000000000000002,
    0x0000000000000020,
    0x00000000000000e4,
    0x000000000000011e,
    0x0000000000000004,
    0x0000000000000040,
    0x00000000000001c8,
    0x000000000000004e,
    0x0000000000000008,
    0x0000000000000080,
    0x00000000000001e2,
    0x000000000000009c,
    0x0000000000000020,
    0x0000000000000072,
];

pub const KEYS_49_FORWARD: [u64; 23] = [
    0x0000000000000000,
    0x7600000000000000,
    0x8900000000000000,
    0xf700000000000000,
    0x4300000000000000,
    0x7600000000000000,
    0xd300000000000000,
    0x000000015c9f834b,
    0x0000000149000000,
    0x0100000000000000,
    0xf100000000000000,
    0xea00000000000000,
    0xef00000000000000,
    0xfb00000000000000,
    0x6200000000000000,
    0x9400000000000000,
    0x5800000000000000,
    0xb300000000000000,
    0xe500000000000000,
    0xf400000000000000,
    0x1900000000000000,
    0x2900000000000000,
    0x3700000000000000,
];

pub const KEYS_9B_FORWARD: [u64; 23] = [
    0x0000000000000000,
    0x7a00000000000000,
    0xad00000000000000,
    0x1500000000000000,
    0x1600000000000000,
    0x7a00000000000000,
    0xe500000000000000,
    0x00000001e21a6dfb,
    0x000000019b000000,
    0xc700000000000000,
    0x0b00000000000000,
    0xae00000000000000,
    0xc800000000000000,
    0x5700000000000000,
    0x6400000000000000,
    0xe600000000000000,
    0x3200000000000000,
    0x7300000000000000,
    0x1900000000000000,
    0xf400000000000000,
    0xc100000000000000,
    0x6800000000000000,
    0x5e00000000000000,
];

pub const KEYS_9B_REFLECTED: [u64; 23] = [
    0x0000000000000000,
    0x00000000000000bc,
    0x000000000000016a,
    0x0000000000000150,
    0x00000000000000d0,
    0x00000000000000bc,
    0x000000000000014e,
    0x00000001bf6cb08f,
    0x00000000000001b3,
    0x00000000000001c6,
    0x00000000000001a0,
    0x00000000000000ea,
    0x0000000000000026,
    0x00000000000001d4,
    0x000000000000004c,
    0x00000000000000ce,
    0x0000000000000098,
    0x000000000000019c,
    0x0000000000000130,
    0x000000000000005e,
    0x0000000000000106,
    0x000000000000002c,
    0x00000000000000f4,
];

pub const KEYS_A7_REFLECTED: [u64; 23] = [
    0x0000000000000000,
    0x000000000000004c,
    0x000000000000012e,
    0x000000000000007c,
    0x00000000000001a0,
    0x000000000000004c,
    0x000000000000013a,
    0x000000019dd66157,
    0x00000000000001cb,
    0x00000000000000f8,
    0x00000000000000d6,
    0x00000000000001f0,
    0x00000000000001ac,
    0x0000000000000076,
    0x00000000000000ce,
    0x00000000000000ec,
    0x000000000000019c,
    0x00000000000001d8,
    0x00000000000000ae,
    0x0000000000000026,
    0x000000000000015c,
    0x000000000000010c,
    0x000000000000004a,
];

pub const KEYS_D5_FORWARD: [u64; 23] = [
    0x0000000000000000,
    0x3700000000000000,
    0x9b00000000000000,
    0xc700000000000000,
    0x7f00000000000000,
    0x3700000000000000,
    0xb600000000000000,
    0x00000001a70fd16e,
    0x00000001d5000000,
    0x0800000000000000,
    0x6700000000000000,
    0xad00000000000000,
    0x4500000000000000,
    0x7300000000000000,
    0xd000000000000000,
    0x7600000000000000,
    0x4a00000000000000,
    0xe500000000000000,
    0xb000000000000000,
    0x9d00000000000000,
    0xc200000000000000,
    0x5b00000000000000,
    0xfe00000000000000,
];
