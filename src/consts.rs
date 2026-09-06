// Copyright 2025 Don MacAskill. Licensed under MIT or Apache-2.0 and Zlib.

#![allow(dead_code)]

// Constants for common values with semantic meaning
pub(crate) const CRC_CHUNK_SIZE: usize = 16;
pub(crate) const CRC_HALF_CHUNK_SIZE: usize = 8;
pub(crate) const CRC_LARGE_CHUNK_THRESHOLD: usize = 256;

pub const NAME_CRC16_ARC: &str = "CRC-16/ARC";
pub const NAME_CRC16_CDMA2000: &str = "CRC-16/CDMA2000";
pub const NAME_CRC16_CMS: &str = "CRC-16/CMS";
pub const NAME_CRC16_DDS_110: &str = "CRC-16/DDS-110";
pub const NAME_CRC16_DECT_R: &str = "CRC-16/DECT-R";
pub const NAME_CRC16_DECT_X: &str = "CRC-16/DECT-X";
pub const NAME_CRC16_DNP: &str = "CRC-16/DNP";
pub const NAME_CRC16_EN_13757: &str = "CRC-16/EN-13757";
pub const NAME_CRC16_GENIBUS: &str = "CRC-16/GENIBUS";
pub const NAME_CRC16_GSM: &str = "CRC-16/GSM";
pub const NAME_CRC16_IBM_3740: &str = "CRC-16/IBM-3740";
pub const NAME_CRC16_IBM_SDLC: &str = "CRC-16/IBM-SDLC";
pub const NAME_CRC16_ISO_IEC_14443_3_A: &str = "CRC-16/ISO-IEC-14443-3-A";
pub const NAME_CRC16_KERMIT: &str = "CRC-16/KERMIT";
pub const NAME_CRC16_LJ1200: &str = "CRC-16/LJ1200";
pub const NAME_CRC16_M17: &str = "CRC-16/M17";
pub const NAME_CRC16_MAXIM_DOW: &str = "CRC-16/MAXIM-DOW";
pub const NAME_CRC16_MCRF4XX: &str = "CRC-16/MCRF4XX";
pub const NAME_CRC16_MODBUS: &str = "CRC-16/MODBUS";
pub const NAME_CRC16_NRSC_5: &str = "CRC-16/NRSC-5";
pub const NAME_CRC16_OPENSAFETY_A: &str = "CRC-16/OPENSAFETY-A";
pub const NAME_CRC16_OPENSAFETY_B: &str = "CRC-16/OPENSAFETY-B";
pub const NAME_CRC16_PROFIBUS: &str = "CRC-16/PROFIBUS";
pub const NAME_CRC16_RIELLO: &str = "CRC-16/RIELLO";
pub const NAME_CRC16_SPI_FUJITSU: &str = "CRC-16/SPI-FUJITSU";
pub const NAME_CRC16_T10_DIF: &str = "CRC-16/T10-DIF";
pub const NAME_CRC16_TELEDISK: &str = "CRC-16/TELEDISK";
pub const NAME_CRC16_TMS37157: &str = "CRC-16/TMS37157";
pub const NAME_CRC16_UMTS: &str = "CRC-16/UMTS";
pub const NAME_CRC16_USB: &str = "CRC-16/USB";
pub const NAME_CRC16_X25: &str = "CRC-16/X-25";
pub const NAME_CRC16_XMODEM: &str = "CRC-16/XMODEM";

pub const NAME_CRC5_USB: &str = "CRC-5/USB";
pub const NAME_CRC5_EPC_C1G2: &str = "CRC-5/EPC-C1G2";
pub const NAME_CRC5_G_704: &str = "CRC-5/G-704";

pub const NAME_CRC8_SMBUS: &str = "CRC-8/SMBUS";
pub const NAME_CRC8_I_432_1: &str = "CRC-8/I-432-1";
pub const NAME_CRC8_ROHC: &str = "CRC-8/ROHC";
pub const NAME_CRC8_GSM_A: &str = "CRC-8/GSM-A";
pub const NAME_CRC8_MIFARE_MAD: &str = "CRC-8/MIFARE-MAD";
pub const NAME_CRC8_I_CODE: &str = "CRC-8/I-CODE";
pub const NAME_CRC8_HITAG: &str = "CRC-8/HITAG";
pub const NAME_CRC8_SAE_J1850: &str = "CRC-8/SAE-J1850";
pub const NAME_CRC8_TECH_3250: &str = "CRC-8/TECH-3250";
pub const NAME_CRC8_OPENSAFETY: &str = "CRC-8/OPENSAFETY";
pub const NAME_CRC8_AUTOSAR: &str = "CRC-8/AUTOSAR";
pub const NAME_CRC8_MAXIM_DOW: &str = "CRC-8/MAXIM-DOW";
pub const NAME_CRC8_NRSC_5: &str = "CRC-8/NRSC-5";
pub const NAME_CRC8_DARC: &str = "CRC-8/DARC";
pub const NAME_CRC8_GSM_B: &str = "CRC-8/GSM-B";
pub const NAME_CRC8_LTE: &str = "CRC-8/LTE";
pub const NAME_CRC8_WCDMA: &str = "CRC-8/WCDMA";
pub const NAME_CRC8_CDMA2000: &str = "CRC-8/CDMA2000";
pub const NAME_CRC8_BLUETOOTH: &str = "CRC-8/BLUETOOTH";
pub const NAME_CRC8_DVB_S2: &str = "CRC-8/DVB-S2";

pub const NAME_CRC31_PHILIPS: &str = "CRC-31/PHILIPS";

pub const NAME_CRC32_AIXM: &str = "CRC-32/AIXM";
pub const NAME_CRC32_AUTOSAR: &str = "CRC-32/AUTOSAR";
pub const NAME_CRC32_BASE91_D: &str = "CRC-32/BASE91-D";
pub const NAME_CRC32_BZIP2: &str = "CRC-32/BZIP2";
pub const NAME_CRC32_CD_ROM_EDC: &str = "CRC-32/CD-ROM-EDC";
pub const NAME_CRC32_CKSUM: &str = "CRC-32/CKSUM";
pub const NAME_CRC32_ISCSI: &str = "CRC-32/ISCSI";
pub const NAME_CRC32_ISO_HDLC: &str = "CRC-32/ISO-HDLC";
pub const NAME_CRC32_JAMCRC: &str = "CRC-32/JAMCRC";
pub const NAME_CRC32_MEF: &str = "CRC-32/MEF";
pub const NAME_CRC32_MPEG_2: &str = "CRC-32/MPEG-2";
pub const NAME_CRC32_XFER: &str = "CRC-32/XFER";

pub const NAME_CRC64_ECMA_182: &str = "CRC-64/ECMA-182";
pub const NAME_CRC64_GO_ISO: &str = "CRC-64/GO-ISO";
pub const NAME_CRC64_MS: &str = "CRC-64/MS";
pub const NAME_CRC64_NVME: &str = "CRC-64/NVME";
pub const NAME_CRC64_REDIS: &str = "CRC-64/REDIS";
pub const NAME_CRC64_WE: &str = "CRC-64/WE";
pub const NAME_CRC64_XZ: &str = "CRC-64/XZ";
