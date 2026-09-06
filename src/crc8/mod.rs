// Copyright 2025 Don MacAskill. Licensed under MIT or Apache-2.0 and Zlib.

pub(crate) mod algorithm;
pub(crate) mod consts;

#[cfg(test)]
mod property_tests {
    use crate::crc8::consts::CRC8_BLUETOOTH;
    use crate::test::consts::{RUST_CRC8_BLUETOOTH, RUST_CRC8_SMBUS};
    use crate::test::miri_compatible_proptest_config;
    use crate::{checksum, checksum_combine, checksum_with_params, CrcAlgorithm};
    use proptest::prelude::*;

    proptest! {
        #![proptest_config(miri_compatible_proptest_config())]

        #[test]
        fn prop_crc8_smbus_matches_reference(data in proptest::collection::vec(any::<u8>(), 0..1024)) {
            let our_result = checksum(CrcAlgorithm::Crc8Smbus, &data);
            let mut crc = RUST_CRC8_SMBUS.digest();
            crc.update(&data);
            let expected = crc.finalize() as u64;
            prop_assert_eq!(our_result & 0xff, expected & 0xff);
        }

        #[test]
        fn prop_crc8_bluetooth_with_params_matches_reference(data in proptest::collection::vec(any::<u8>(), 0..1024)) {
            let params = CRC8_BLUETOOTH;
            let our_result = checksum_with_params(params, &data);
            let mut crc = RUST_CRC8_BLUETOOTH.digest();
            crc.update(&data);
            let expected = crc.finalize() as u64;
            prop_assert_eq!(our_result & 0xff, expected & 0xff);
        }

        #[test]
        fn prop_crc8_smbus_checksum_combine_roundtrip(
            data1 in proptest::collection::vec(any::<u8>(), 0..512),
            data2 in proptest::collection::vec(any::<u8>(), 0..512)
        ) {
            let mut combined = Vec::new();
            combined.extend_from_slice(&data1);
            combined.extend_from_slice(&data2);

            let crc1 = checksum(CrcAlgorithm::Crc8Smbus, &data1);
            let crc2 = checksum(CrcAlgorithm::Crc8Smbus, &data2);
            let combined_via_combine = checksum_combine(CrcAlgorithm::Crc8Smbus, crc1, crc2, data2.len() as u64);
            let combined_direct = checksum(CrcAlgorithm::Crc8Smbus, &combined);

            prop_assert_eq!(combined_via_combine & 0xff, combined_direct & 0xff);
        }
    }
}
