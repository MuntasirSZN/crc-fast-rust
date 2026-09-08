// Copyright 2025 Don MacAskill. Licensed under MIT or Apache-2.0 and Zlib.

use crate::consts::*;
use crate::CrcAlgorithm;
use core::fmt::{Display, Formatter};
use core::str::FromStr;

impl FromStr for CrcAlgorithm {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            NAME_CRC16_ARC => Ok(CrcAlgorithm::Crc16Arc),
            NAME_CRC16_CDMA2000 => Ok(CrcAlgorithm::Crc16Cdma2000),
            NAME_CRC16_CMS => Ok(CrcAlgorithm::Crc16Cms),
            NAME_CRC16_DDS_110 => Ok(CrcAlgorithm::Crc16Dds110),
            NAME_CRC16_DECT_R => Ok(CrcAlgorithm::Crc16DectR),
            NAME_CRC16_DECT_X => Ok(CrcAlgorithm::Crc16DectX),
            NAME_CRC16_DNP => Ok(CrcAlgorithm::Crc16Dnp),
            NAME_CRC16_EN_13757 => Ok(CrcAlgorithm::Crc16En13757),
            NAME_CRC16_GENIBUS => Ok(CrcAlgorithm::Crc16Genibus),
            NAME_CRC16_GSM => Ok(CrcAlgorithm::Crc16Gsm),
            NAME_CRC16_IBM_3740 => Ok(CrcAlgorithm::Crc16Ibm3740),
            NAME_CRC16_IBM_SDLC => Ok(CrcAlgorithm::Crc16IbmSdlc),
            NAME_CRC16_ISO_IEC_14443_3_A => Ok(CrcAlgorithm::Crc16IsoIec144433A),
            NAME_CRC16_KERMIT => Ok(CrcAlgorithm::Crc16Kermit),
            NAME_CRC16_LJ1200 => Ok(CrcAlgorithm::Crc16Lj1200),
            NAME_CRC16_M17 => Ok(CrcAlgorithm::Crc16M17),
            NAME_CRC16_MAXIM_DOW => Ok(CrcAlgorithm::Crc16MaximDow),
            NAME_CRC16_MCRF4XX => Ok(CrcAlgorithm::Crc16Mcrf4xx),
            NAME_CRC16_MODBUS => Ok(CrcAlgorithm::Crc16Modbus),
            NAME_CRC16_NRSC_5 => Ok(CrcAlgorithm::Crc16Nrsc5),
            NAME_CRC16_OPENSAFETY_A => Ok(CrcAlgorithm::Crc16OpensafetyA),
            NAME_CRC16_OPENSAFETY_B => Ok(CrcAlgorithm::Crc16OpensafetyB),
            NAME_CRC16_PROFIBUS => Ok(CrcAlgorithm::Crc16Profibus),
            NAME_CRC16_RIELLO => Ok(CrcAlgorithm::Crc16Riello),
            NAME_CRC16_SPI_FUJITSU => Ok(CrcAlgorithm::Crc16SpiFujitsu),
            NAME_CRC16_T10_DIF => Ok(CrcAlgorithm::Crc16T10Dif),
            NAME_CRC16_TELEDISK => Ok(CrcAlgorithm::Crc16Teledisk),
            NAME_CRC16_TMS37157 => Ok(CrcAlgorithm::Crc16Tms37157),
            NAME_CRC16_UMTS => Ok(CrcAlgorithm::Crc16Umts),
            NAME_CRC16_USB => Ok(CrcAlgorithm::Crc16Usb),
            NAME_CRC16_XMODEM => Ok(CrcAlgorithm::Crc16Xmodem),
            // CRC-16/X-25 is a catalogue alias of IBM-SDLC (same params)
            NAME_CRC16_X25 => Ok(CrcAlgorithm::Crc16IbmSdlc),
            // Common aliases
            "CRC-16/CCITT-FALSE" => Ok(CrcAlgorithm::Crc16Ibm3740),
            "CRC-16/CCITT-TRUE" => Ok(CrcAlgorithm::Crc16Kermit),
            "CRC-32C" => Ok(CrcAlgorithm::Crc32Iscsi),
            "CRC-32/CASTAGNOLI" => Ok(CrcAlgorithm::Crc32Iscsi),
            "CRC-32" => Ok(CrcAlgorithm::Crc32IsoHdlc),
            NAME_CRC5_USB => Ok(CrcAlgorithm::Crc5Usb),
            NAME_CRC5_EPC_C1G2 => Ok(CrcAlgorithm::Crc5EpcC1G2),
            NAME_CRC5_G_704 => Ok(CrcAlgorithm::Crc5G704),
            NAME_CRC8_SMBUS => Ok(CrcAlgorithm::Crc8Smbus),
            NAME_CRC8_I_432_1 => Ok(CrcAlgorithm::Crc8I4321),
            NAME_CRC8_ROHC => Ok(CrcAlgorithm::Crc8Rohc),
            NAME_CRC8_GSM_A => Ok(CrcAlgorithm::Crc8GsmA),
            NAME_CRC8_MIFARE_MAD => Ok(CrcAlgorithm::Crc8MifareMad),
            NAME_CRC8_I_CODE => Ok(CrcAlgorithm::Crc8ICode),
            NAME_CRC8_HITAG => Ok(CrcAlgorithm::Crc8Hitag),
            NAME_CRC8_SAE_J1850 => Ok(CrcAlgorithm::Crc8SaeJ1850),
            NAME_CRC8_TECH_3250 => Ok(CrcAlgorithm::Crc8Tech3250),
            NAME_CRC8_OPENSAFETY => Ok(CrcAlgorithm::Crc8Opensafety),
            NAME_CRC8_AUTOSAR => Ok(CrcAlgorithm::Crc8Autosar),
            NAME_CRC8_MAXIM_DOW => Ok(CrcAlgorithm::Crc8MaximDow),
            NAME_CRC8_NRSC_5 => Ok(CrcAlgorithm::Crc8Nrsc5),
            NAME_CRC8_DARC => Ok(CrcAlgorithm::Crc8Darc),
            NAME_CRC8_GSM_B => Ok(CrcAlgorithm::Crc8GsmB),
            NAME_CRC8_LTE => Ok(CrcAlgorithm::Crc8Lte),
            NAME_CRC8_WCDMA => Ok(CrcAlgorithm::Crc8Wcdma),
            NAME_CRC8_CDMA2000 => Ok(CrcAlgorithm::Crc8Cdma2000),
            NAME_CRC8_BLUETOOTH => Ok(CrcAlgorithm::Crc8Bluetooth),
            NAME_CRC8_DVB_S2 => Ok(CrcAlgorithm::Crc8DvbS2),
            NAME_CRC31_PHILIPS => Ok(CrcAlgorithm::Crc31Philips),
            NAME_CRC32_AIXM => Ok(CrcAlgorithm::Crc32Aixm),
            NAME_CRC32_AUTOSAR => Ok(CrcAlgorithm::Crc32Autosar),
            NAME_CRC32_BASE91_D => Ok(CrcAlgorithm::Crc32Base91D),
            NAME_CRC32_BZIP2 => Ok(CrcAlgorithm::Crc32Bzip2),
            NAME_CRC32_CD_ROM_EDC => Ok(CrcAlgorithm::Crc32CdRomEdc),
            NAME_CRC32_CKSUM => Ok(CrcAlgorithm::Crc32Cksum),
            NAME_CRC32_ISCSI => Ok(CrcAlgorithm::Crc32Iscsi),
            NAME_CRC32_ISO_HDLC => Ok(CrcAlgorithm::Crc32IsoHdlc),
            NAME_CRC32_JAMCRC => Ok(CrcAlgorithm::Crc32Jamcrc),
            NAME_CRC32_MEF => Ok(CrcAlgorithm::Crc32Mef),
            NAME_CRC32_MPEG_2 => Ok(CrcAlgorithm::Crc32Mpeg2),
            NAME_CRC32_XFER => Ok(CrcAlgorithm::Crc32Xfer),
            NAME_CRC64_GO_ISO => Ok(CrcAlgorithm::Crc64GoIso),
            NAME_CRC64_MS => Ok(CrcAlgorithm::Crc64Ms),
            NAME_CRC64_NVME => Ok(CrcAlgorithm::Crc64Nvme),
            NAME_CRC64_REDIS => Ok(CrcAlgorithm::Crc64Redis),
            NAME_CRC64_XZ => Ok(CrcAlgorithm::Crc64Xz),
            NAME_CRC64_ECMA_182 => Ok(CrcAlgorithm::Crc64Ecma182),
            NAME_CRC64_WE => Ok(CrcAlgorithm::Crc64We),
            _ => Err(()),
        }
    }
}

impl Display for CrcAlgorithm {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            CrcAlgorithm::Crc16Arc => write!(f, "{NAME_CRC16_ARC}"),
            CrcAlgorithm::Crc16Cdma2000 => write!(f, "{NAME_CRC16_CDMA2000}"),
            CrcAlgorithm::Crc16Cms => write!(f, "{NAME_CRC16_CMS}"),
            CrcAlgorithm::Crc16Dds110 => write!(f, "{NAME_CRC16_DDS_110}"),
            CrcAlgorithm::Crc16DectR => write!(f, "{NAME_CRC16_DECT_R}"),
            CrcAlgorithm::Crc16DectX => write!(f, "{NAME_CRC16_DECT_X}"),
            CrcAlgorithm::Crc16Dnp => write!(f, "{NAME_CRC16_DNP}"),
            CrcAlgorithm::Crc16En13757 => write!(f, "{NAME_CRC16_EN_13757}"),
            CrcAlgorithm::Crc16Genibus => write!(f, "{NAME_CRC16_GENIBUS}"),
            CrcAlgorithm::Crc16Gsm => write!(f, "{NAME_CRC16_GSM}"),
            CrcAlgorithm::Crc16Ibm3740 => write!(f, "{NAME_CRC16_IBM_3740}",),
            CrcAlgorithm::Crc16IbmSdlc => write!(f, "{NAME_CRC16_IBM_SDLC}",),
            CrcAlgorithm::Crc16IsoIec144433A => {
                write!(f, "{NAME_CRC16_ISO_IEC_14443_3_A}",)
            }
            CrcAlgorithm::Crc16Kermit => write!(f, "{NAME_CRC16_KERMIT}"),
            CrcAlgorithm::Crc16Lj1200 => write!(f, "{NAME_CRC16_LJ1200}"),
            CrcAlgorithm::Crc16M17 => write!(f, "{NAME_CRC16_M17}"),
            CrcAlgorithm::Crc16MaximDow => write!(f, "{NAME_CRC16_MAXIM_DOW}"),
            CrcAlgorithm::Crc16Mcrf4xx => write!(f, "{NAME_CRC16_MCRF4XX}"),
            CrcAlgorithm::Crc16Modbus => write!(f, "{NAME_CRC16_MODBUS}"),
            CrcAlgorithm::Crc16Nrsc5 => write!(f, "{NAME_CRC16_NRSC_5}"),
            CrcAlgorithm::Crc16OpensafetyA => write!(f, "{NAME_CRC16_OPENSAFETY_A}"),
            CrcAlgorithm::Crc16OpensafetyB => write!(f, "{NAME_CRC16_OPENSAFETY_B}"),
            CrcAlgorithm::Crc16Profibus => write!(f, "{NAME_CRC16_PROFIBUS}"),
            CrcAlgorithm::Crc16Riello => write!(f, "{NAME_CRC16_RIELLO}"),
            CrcAlgorithm::Crc16SpiFujitsu => write!(f, "{NAME_CRC16_SPI_FUJITSU}"),
            CrcAlgorithm::Crc16T10Dif => write!(f, "{NAME_CRC16_T10_DIF}",),
            CrcAlgorithm::Crc16Teledisk => write!(f, "{NAME_CRC16_TELEDISK}"),
            CrcAlgorithm::Crc16Tms37157 => write!(f, "{NAME_CRC16_TMS37157}"),
            CrcAlgorithm::Crc16Umts => write!(f, "{NAME_CRC16_UMTS}"),
            CrcAlgorithm::Crc16Usb => write!(f, "{NAME_CRC16_USB}"),
            CrcAlgorithm::Crc16Xmodem => write!(f, "{NAME_CRC16_XMODEM}"),
            CrcAlgorithm::Crc5Usb => write!(f, "{NAME_CRC5_USB}"),
            CrcAlgorithm::Crc5EpcC1G2 => write!(f, "{NAME_CRC5_EPC_C1G2}"),
            CrcAlgorithm::Crc5G704 => write!(f, "{NAME_CRC5_G_704}"),
            CrcAlgorithm::Crc8Smbus => write!(f, "{NAME_CRC8_SMBUS}"),
            CrcAlgorithm::Crc8I4321 => write!(f, "{NAME_CRC8_I_432_1}"),
            CrcAlgorithm::Crc8Rohc => write!(f, "{NAME_CRC8_ROHC}"),
            CrcAlgorithm::Crc8GsmA => write!(f, "{NAME_CRC8_GSM_A}"),
            CrcAlgorithm::Crc8MifareMad => write!(f, "{NAME_CRC8_MIFARE_MAD}"),
            CrcAlgorithm::Crc8ICode => write!(f, "{NAME_CRC8_I_CODE}"),
            CrcAlgorithm::Crc8Hitag => write!(f, "{NAME_CRC8_HITAG}"),
            CrcAlgorithm::Crc8SaeJ1850 => write!(f, "{NAME_CRC8_SAE_J1850}"),
            CrcAlgorithm::Crc8Tech3250 => write!(f, "{NAME_CRC8_TECH_3250}"),
            CrcAlgorithm::Crc8Opensafety => write!(f, "{NAME_CRC8_OPENSAFETY}"),
            CrcAlgorithm::Crc8Autosar => write!(f, "{NAME_CRC8_AUTOSAR}"),
            CrcAlgorithm::Crc8MaximDow => write!(f, "{NAME_CRC8_MAXIM_DOW}"),
            CrcAlgorithm::Crc8Nrsc5 => write!(f, "{NAME_CRC8_NRSC_5}"),
            CrcAlgorithm::Crc8Darc => write!(f, "{NAME_CRC8_DARC}"),
            CrcAlgorithm::Crc8GsmB => write!(f, "{NAME_CRC8_GSM_B}"),
            CrcAlgorithm::Crc8Lte => write!(f, "{NAME_CRC8_LTE}"),
            CrcAlgorithm::Crc8Wcdma => write!(f, "{NAME_CRC8_WCDMA}"),
            CrcAlgorithm::Crc8Cdma2000 => write!(f, "{NAME_CRC8_CDMA2000}"),
            CrcAlgorithm::Crc8Bluetooth => write!(f, "{NAME_CRC8_BLUETOOTH}"),
            CrcAlgorithm::Crc8DvbS2 => write!(f, "{NAME_CRC8_DVB_S2}"),
            CrcAlgorithm::Crc31Philips => write!(f, "{NAME_CRC31_PHILIPS}"),
            CrcAlgorithm::Crc32Aixm => write!(f, "{NAME_CRC32_AIXM}",),
            CrcAlgorithm::Crc32Autosar => write!(f, "{NAME_CRC32_AUTOSAR}",),
            CrcAlgorithm::Crc32Base91D => write!(f, "{NAME_CRC32_BASE91_D}",),
            CrcAlgorithm::Crc32Bzip2 => write!(f, "{NAME_CRC32_BZIP2}",),
            CrcAlgorithm::Crc32CdRomEdc => write!(f, "{NAME_CRC32_CD_ROM_EDC}",),
            CrcAlgorithm::Crc32Cksum => write!(f, "{NAME_CRC32_CKSUM}",),
            CrcAlgorithm::Crc32Iscsi => write!(f, "{NAME_CRC32_ISCSI}",),
            CrcAlgorithm::Crc32IsoHdlc => write!(f, "{NAME_CRC32_ISO_HDLC}",),
            CrcAlgorithm::Crc32Jamcrc => write!(f, "{NAME_CRC32_JAMCRC}",),
            CrcAlgorithm::Crc32Mef => write!(f, "{NAME_CRC32_MEF}",),
            CrcAlgorithm::Crc32Mpeg2 => write!(f, "{NAME_CRC32_MPEG_2}",),
            CrcAlgorithm::Crc32Xfer => write!(f, "{NAME_CRC32_XFER}",),
            CrcAlgorithm::CrcCustom => write!(f, "CRC/CUSTOM"),
            CrcAlgorithm::Crc64GoIso => write!(f, "{NAME_CRC64_GO_ISO}",),
            CrcAlgorithm::Crc64Ms => write!(f, "{NAME_CRC64_MS}",),
            CrcAlgorithm::Crc64Nvme => write!(f, "{NAME_CRC64_NVME}",),
            CrcAlgorithm::Crc64Redis => write!(f, "{NAME_CRC64_REDIS}",),
            CrcAlgorithm::Crc64Xz => write!(f, "{NAME_CRC64_XZ}",),
            CrcAlgorithm::Crc64Ecma182 => write!(f, "{NAME_CRC64_ECMA_182}",),
            CrcAlgorithm::Crc64We => write!(f, "{NAME_CRC64_WE}",),
        }
    }
}

#[cfg(any(
    target_arch = "x86",
    target_arch = "x86_64",
    target_arch = "aarch64",
    target_arch = "wasm32",
))]
#[derive(Debug, Copy, Clone)]
pub(crate) enum Reflector<T> {
    NoReflector,
    ForwardReflector { smask: T },
}

#[cfg(any(
    target_arch = "x86",
    target_arch = "x86_64",
    target_arch = "aarch64",
    target_arch = "wasm32",
))]
/// Different processing strategies based on data length
pub(crate) enum DataChunkProcessor {
    From0To15,   // 0-15 bytes
    From16,      // exactly 16 bytes
    From17To31,  // 17-31 bytes
    From32To255, // 32-255 bytes
}

#[cfg(any(
    target_arch = "x86",
    target_arch = "x86_64",
    target_arch = "aarch64",
    target_arch = "wasm32",
))]
impl DataChunkProcessor {
    /// Select the appropriate processor based on data length
    pub fn for_length(len: usize) -> Self {
        match len {
            0..=15 => Self::From0To15,
            16 => Self::From16,
            17..=31 => Self::From17To31,
            32..=255 => Self::From32To255,
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::CrcAlgorithm;
    use core::str::FromStr;
    #[cfg(target_arch = "wasm32")]
    use wasm_bindgen_test::wasm_bindgen_test as test;

    fn all_non_custom_algorithms() -> Vec<CrcAlgorithm> {
        vec![
            CrcAlgorithm::Crc16Arc,
            CrcAlgorithm::Crc16Cdma2000,
            CrcAlgorithm::Crc16Cms,
            CrcAlgorithm::Crc16Dds110,
            CrcAlgorithm::Crc16DectR,
            CrcAlgorithm::Crc16DectX,
            CrcAlgorithm::Crc16Dnp,
            CrcAlgorithm::Crc16En13757,
            CrcAlgorithm::Crc16Genibus,
            CrcAlgorithm::Crc16Gsm,
            CrcAlgorithm::Crc16Ibm3740,
            CrcAlgorithm::Crc16IbmSdlc,
            CrcAlgorithm::Crc16IsoIec144433A,
            CrcAlgorithm::Crc16Kermit,
            CrcAlgorithm::Crc16Lj1200,
            CrcAlgorithm::Crc16M17,
            CrcAlgorithm::Crc16MaximDow,
            CrcAlgorithm::Crc16Mcrf4xx,
            CrcAlgorithm::Crc16Modbus,
            CrcAlgorithm::Crc16Nrsc5,
            CrcAlgorithm::Crc16OpensafetyA,
            CrcAlgorithm::Crc16OpensafetyB,
            CrcAlgorithm::Crc16Profibus,
            CrcAlgorithm::Crc16Riello,
            CrcAlgorithm::Crc16SpiFujitsu,
            CrcAlgorithm::Crc16T10Dif,
            CrcAlgorithm::Crc16Teledisk,
            CrcAlgorithm::Crc16Tms37157,
            CrcAlgorithm::Crc16Umts,
            CrcAlgorithm::Crc16Usb,
            CrcAlgorithm::Crc16Xmodem,
            CrcAlgorithm::Crc5Usb,
            CrcAlgorithm::Crc5EpcC1G2,
            CrcAlgorithm::Crc5G704,
            CrcAlgorithm::Crc8Smbus,
            CrcAlgorithm::Crc8I4321,
            CrcAlgorithm::Crc8Rohc,
            CrcAlgorithm::Crc8GsmA,
            CrcAlgorithm::Crc8MifareMad,
            CrcAlgorithm::Crc8ICode,
            CrcAlgorithm::Crc8Hitag,
            CrcAlgorithm::Crc8SaeJ1850,
            CrcAlgorithm::Crc8Tech3250,
            CrcAlgorithm::Crc8Opensafety,
            CrcAlgorithm::Crc8Autosar,
            CrcAlgorithm::Crc8MaximDow,
            CrcAlgorithm::Crc8Nrsc5,
            CrcAlgorithm::Crc8Darc,
            CrcAlgorithm::Crc8GsmB,
            CrcAlgorithm::Crc8Lte,
            CrcAlgorithm::Crc8Wcdma,
            CrcAlgorithm::Crc8Cdma2000,
            CrcAlgorithm::Crc8Bluetooth,
            CrcAlgorithm::Crc8DvbS2,
            CrcAlgorithm::Crc31Philips,
            CrcAlgorithm::Crc32Aixm,
            CrcAlgorithm::Crc32Autosar,
            CrcAlgorithm::Crc32Base91D,
            CrcAlgorithm::Crc32Bzip2,
            CrcAlgorithm::Crc32CdRomEdc,
            CrcAlgorithm::Crc32Cksum,
            CrcAlgorithm::Crc32Iscsi,
            CrcAlgorithm::Crc32IsoHdlc,
            CrcAlgorithm::Crc32Jamcrc,
            CrcAlgorithm::Crc32Mef,
            CrcAlgorithm::Crc32Mpeg2,
            CrcAlgorithm::Crc32Xfer,
            CrcAlgorithm::Crc64Ecma182,
            CrcAlgorithm::Crc64GoIso,
            CrcAlgorithm::Crc64Ms,
            CrcAlgorithm::Crc64Nvme,
            CrcAlgorithm::Crc64Redis,
            CrcAlgorithm::Crc64We,
            CrcAlgorithm::Crc64Xz,
        ]
    }

    #[test]
    fn display_fromstr_roundtrip_all_algorithms() {
        for algorithm in all_non_custom_algorithms() {
            let name = algorithm.to_string();
            assert!(!name.is_empty(), "{algorithm:?} displayed empty");
            assert_eq!(
                CrcAlgorithm::from_str(&name),
                Ok(algorithm),
                "round-trip failed for {name}"
            );
        }
    }

    #[test]
    fn display_spot_checks_canonical_names() {
        assert_eq!(CrcAlgorithm::Crc16Arc.to_string(), "CRC-16/ARC");
        assert_eq!(CrcAlgorithm::Crc5Usb.to_string(), "CRC-5/USB");
        assert_eq!(CrcAlgorithm::Crc8Smbus.to_string(), "CRC-8/SMBUS");
        assert_eq!(CrcAlgorithm::Crc31Philips.to_string(), "CRC-31/PHILIPS");
        assert_eq!(CrcAlgorithm::Crc32IsoHdlc.to_string(), "CRC-32/ISO-HDLC");
        assert_eq!(CrcAlgorithm::Crc64Nvme.to_string(), "CRC-64/NVME");
        assert_eq!(CrcAlgorithm::CrcCustom.to_string(), "CRC/CUSTOM");
    }

    #[test]
    fn fromstr_aliases_resolve_to_canonical() {
        // Catalogue alias: X-25 shares params with IBM-SDLC.
        assert_eq!(
            CrcAlgorithm::from_str("CRC-16/X-25"),
            Ok(CrcAlgorithm::Crc16IbmSdlc)
        );
        assert_eq!(
            CrcAlgorithm::from_str("CRC-16/CCITT-FALSE"),
            Ok(CrcAlgorithm::Crc16Ibm3740)
        );
        assert_eq!(
            CrcAlgorithm::from_str("CRC-16/CCITT-TRUE"),
            Ok(CrcAlgorithm::Crc16Kermit)
        );
        assert_eq!(
            CrcAlgorithm::from_str("CRC-32"),
            Ok(CrcAlgorithm::Crc32IsoHdlc)
        );
        assert_eq!(
            CrcAlgorithm::from_str("CRC-32C"),
            Ok(CrcAlgorithm::Crc32Iscsi)
        );
        assert_eq!(
            CrcAlgorithm::from_str("CRC-32/CASTAGNOLI"),
            Ok(CrcAlgorithm::Crc32Iscsi)
        );
    }

    #[test]
    fn fromstr_rejects_unknown_and_custom() {
        assert_eq!(CrcAlgorithm::from_str(""), Err(()));
        assert_eq!(CrcAlgorithm::from_str("bogus"), Err(()));
        assert_eq!(CrcAlgorithm::from_str("CRC/CUSTOM"), Err(()));
        // Names are case-sensitive.
        assert_eq!(CrcAlgorithm::from_str("crc-32/iso-hdlc"), Err(()));
        assert_eq!(CrcAlgorithm::from_str("CRC-32/ISO-HDLC "), Err(()));
    }
}
