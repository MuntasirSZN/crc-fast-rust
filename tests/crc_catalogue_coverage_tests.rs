// Comprehensive catalogue check values for newly added variants.
use crc_fast::{checksum, checksum_combine, checksum_with_params, CrcAlgorithm, CrcParams, Digest};

fn check_all() -> Vec<(CrcAlgorithm, u64)> {
    vec![
        (CrcAlgorithm::Crc5EpcC1G2, 0x00),
        (CrcAlgorithm::Crc5G704, 0x07),
        (CrcAlgorithm::Crc8Smbus, 0xF4),
        (CrcAlgorithm::Crc8I4321, 0xA1),
        (CrcAlgorithm::Crc8Rohc, 0xD0),
        (CrcAlgorithm::Crc8GsmA, 0x37),
        (CrcAlgorithm::Crc8MifareMad, 0x99),
        (CrcAlgorithm::Crc8ICode, 0x7E),
        (CrcAlgorithm::Crc8Hitag, 0xB4),
        (CrcAlgorithm::Crc8SaeJ1850, 0x4B),
        (CrcAlgorithm::Crc8Tech3250, 0x97),
        (CrcAlgorithm::Crc8Opensafety, 0x3E),
        (CrcAlgorithm::Crc8Autosar, 0xDF),
        (CrcAlgorithm::Crc8MaximDow, 0xA1),
        (CrcAlgorithm::Crc8Nrsc5, 0xF7),
        (CrcAlgorithm::Crc8Darc, 0x15),
        (CrcAlgorithm::Crc8GsmB, 0x94),
        (CrcAlgorithm::Crc8Lte, 0xEA),
        (CrcAlgorithm::Crc8Wcdma, 0x25),
        (CrcAlgorithm::Crc8Cdma2000, 0xDA),
        (CrcAlgorithm::Crc8Bluetooth, 0x26),
        (CrcAlgorithm::Crc8DvbS2, 0xBC),
    ]
}

#[test]
fn all_new_check_values() {
    for (alg, expected) in check_all() {
        assert_eq!(
            checksum(alg, b"123456789"),
            expected,
            "check failed for {alg:?}"
        );
    }
}

#[test]
fn crc8_smbus_check_value() {
    assert_eq!(checksum(CrcAlgorithm::Crc8Smbus, b"123456789"), 0xF4);
}

#[test]
fn crc8_maxim_check_value() {
    assert_eq!(checksum(CrcAlgorithm::Crc8MaximDow, b"123456789"), 0xA1);
}

#[test]
fn crc5_epc_check_value() {
    assert_eq!(checksum(CrcAlgorithm::Crc5EpcC1G2, b"123456789"), 0x00);
}

#[test]
fn crc5_g704_check_value() {
    assert_eq!(checksum(CrcAlgorithm::Crc5G704, b"123456789"), 0x07);
}

#[test]
fn crc16_x25_alias_matches_ibm_sdlc() {
    let a = checksum(CrcAlgorithm::Crc16IbmSdlc, b"123456789");
    assert_eq!(a, 0x906E);
    let parsed: CrcAlgorithm = "CRC-16/X-25".parse().expect("X-25 alias");
    assert_eq!(checksum(parsed, b"123456789"), a);
}

#[test]
fn new_variants_against_crc_crate_reference() {
    let pairs: Vec<(CrcAlgorithm, &'static crc::Algorithm<u8>)> = vec![
        (CrcAlgorithm::Crc8Smbus, &crc::CRC_8_SMBUS),
        (CrcAlgorithm::Crc8Rohc, &crc::CRC_8_ROHC),
        (CrcAlgorithm::Crc8GsmA, &crc::CRC_8_GSM_A),
        (CrcAlgorithm::Crc8Hitag, &crc::CRC_8_HITAG),
        (CrcAlgorithm::Crc8SaeJ1850, &crc::CRC_8_SAE_J1850),
        (CrcAlgorithm::Crc8Autosar, &crc::CRC_8_AUTOSAR),
        (CrcAlgorithm::Crc8MaximDow, &crc::CRC_8_MAXIM_DOW),
        (CrcAlgorithm::Crc8Bluetooth, &crc::CRC_8_BLUETOOTH),
        (CrcAlgorithm::Crc8Cdma2000, &crc::CRC_8_CDMA2000),
        (CrcAlgorithm::Crc8Darc, &crc::CRC_8_DARC),
        (CrcAlgorithm::Crc8DvbS2, &crc::CRC_8_DVB_S2),
        (CrcAlgorithm::Crc8Lte, &crc::CRC_8_LTE),
        (CrcAlgorithm::Crc8Wcdma, &crc::CRC_8_WCDMA),
        (CrcAlgorithm::Crc5EpcC1G2, &crc::CRC_5_EPC_C1G2),
        (CrcAlgorithm::Crc5G704, &crc::CRC_5_G_704),
    ];
    let data_sets: &[&[u8]] = &[
        b"",
        b"123456789",
        b"hello world, this is a longer test string for CRC paths",
        &[0u8; 256],
        &[0xFFu8; 300],
    ];
    for (alg, ref_alg) in pairs {
        let reference = crc::Crc::<u8, crc::Table<16>>::new(ref_alg);
        for data in data_sets {
            let expected = reference.checksum(data) as u64;
            assert_eq!(
                checksum(alg, data),
                expected,
                "ref mismatch for {alg:?} len={}",
                data.len()
            );
            // Digest incremental path must match one-shot
            let mut digest = Digest::new(alg);
            let mid = data.len() / 2;
            digest.update(&data[..mid]);
            digest.update(&data[mid..]);
            assert_eq!(digest.finalize(), expected, "digest mismatch for {alg:?}");
            // Custom params path must match too (name is inert for checksum)
            let params = CrcParams::new(
                "CRC/CUSTOM",
                ref_alg.width,
                ref_alg.poly as u64,
                ref_alg.init as u64,
                ref_alg.refin,
                ref_alg.xorout as u64,
                ref_alg.check as u64,
            );
            assert_eq!(
                checksum_with_params(params, data),
                expected,
                "custom mismatch for {alg:?}"
            );
        }
    }
}

#[test]
fn new_variants_combine_roundtrip() {
    for (alg, _) in check_all() {
        let a = b"1234";
        let b = b"56789";
        let mut combined = Vec::new();
        combined.extend_from_slice(a);
        combined.extend_from_slice(b);
        let c1 = checksum(alg, a);
        let c2 = checksum(alg, b);
        assert_eq!(
            checksum_combine(alg, c1, c2, b.len() as u64),
            checksum(alg, &combined),
            "combine mismatch for {alg:?}"
        );
    }
}

#[test]
fn new_variants_fromstr_display_roundtrip() {
    for (alg, _) in check_all() {
        let name = alg.to_string();
        let parsed: CrcAlgorithm = name.parse().expect("parse own display");
        assert_eq!(parsed, alg, "roundtrip failed for {name}");
    }
    // Aliases
    assert_eq!(
        "CRC-16/X-25".parse::<CrcAlgorithm>().unwrap(),
        CrcAlgorithm::Crc16IbmSdlc
    );
    assert_eq!(
        "CRC-16/CCITT-FALSE".parse::<CrcAlgorithm>().unwrap(),
        CrcAlgorithm::Crc16Ibm3740
    );
    assert_eq!(
        "CRC-32C".parse::<CrcAlgorithm>().unwrap(),
        CrcAlgorithm::Crc32Iscsi
    );
    assert_eq!(
        "CRC-32".parse::<CrcAlgorithm>().unwrap(),
        CrcAlgorithm::Crc32IsoHdlc
    );
}
