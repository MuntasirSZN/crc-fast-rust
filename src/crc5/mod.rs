// Copyright 2025 Don MacAskill. Licensed under MIT or Apache-2.0 and Zlib.

pub(crate) mod algorithm;
pub(crate) mod consts;

#[cfg(test)]
mod property_tests {
    use crate::crc5::consts::CRC5_USB;
    use crate::test::consts::RUST_CRC5_USB;
    use crate::test::miri_compatible_proptest_config;
    use crate::{checksum, checksum_combine, checksum_with_params, CrcAlgorithm, CrcParams};
    use proptest::prelude::*;

    proptest! {
        #![proptest_config(miri_compatible_proptest_config())]

        #[test]
        fn prop_crc5_usb_matches_reference(data in proptest::collection::vec(any::<u8>(), 0..1024)) {
            let our_result = checksum(CrcAlgorithm::Crc5Usb, &data);
            let mut crc = RUST_CRC5_USB.digest();
            crc.update(&data);
            let expected = crc.finalize() as u64;
            prop_assert_eq!(our_result & 0x1f, expected & 0x1f);
        }

        #[test]
        fn prop_crc5_usb_with_params_matches_reference(data in proptest::collection::vec(any::<u8>(), 0..1024)) {
            let params = CRC5_USB;
            let our_result = checksum_with_params(params, &data);
            let mut crc = RUST_CRC5_USB.digest();
            crc.update(&data);
            let expected = crc.finalize() as u64;
            prop_assert_eq!(our_result & 0x1f, expected & 0x1f);
        }

        #[test]
        fn prop_crc5_custom_reflected_matches_reference(data in proptest::collection::vec(any::<u8>(), 0..1024)) {
            let custom_params = CrcParams::new(
                "CRC-5/CUSTOM",
                5,
                0x05,
                0x1f,
                true,
                0x1f,
                0x19,
            );
            let our_result = checksum_with_params(custom_params, &data);
            let mut crc = RUST_CRC5_USB.digest();
            crc.update(&data);
            let expected = crc.finalize() as u64;
            prop_assert_eq!(our_result & 0x1f, expected & 0x1f);
        }

        #[test]
        fn prop_crc5_usb_checksum_combine_roundtrip(
            data1 in proptest::collection::vec(any::<u8>(), 0..512),
            data2 in proptest::collection::vec(any::<u8>(), 0..512)
        ) {
            let mut combined = Vec::new();
            combined.extend_from_slice(&data1);
            combined.extend_from_slice(&data2);

            let crc1 = checksum(CrcAlgorithm::Crc5Usb, &data1);
            let crc2 = checksum(CrcAlgorithm::Crc5Usb, &data2);
            let combined_via_combine = checksum_combine(CrcAlgorithm::Crc5Usb, crc1, crc2, data2.len() as u64);
            let combined_direct = checksum(CrcAlgorithm::Crc5Usb, &combined);

            prop_assert_eq!(combined_via_combine & 0x1f, combined_direct & 0x1f);
        }
    }
}
