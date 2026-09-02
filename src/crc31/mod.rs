// Copyright 2025 Don MacAskill. Licensed under MIT or Apache-2.0 and Zlib.

pub(crate) mod algorithm;
pub(crate) mod consts;

#[cfg(test)]
mod property_tests {
    use crate::crc31::consts::CRC31_PHILIPS;
    use crate::test::consts::RUST_CRC31_PHILIPS;
    use crate::test::miri_compatible_proptest_config;
    use crate::{checksum, checksum_combine, checksum_with_params, CrcAlgorithm, CrcParams};
    use proptest::prelude::*;

    proptest! {
        #![proptest_config(miri_compatible_proptest_config())]

        #[test]
        fn prop_crc31_philips_matches_reference(data in proptest::collection::vec(any::<u8>(), 0..1024)) {
            let our_result = checksum(CrcAlgorithm::Crc31Philips, &data);
            let mut crc = RUST_CRC31_PHILIPS.digest();
            crc.update(&data);
            let expected = crc.finalize() as u64;
            prop_assert_eq!(our_result & 0x7fffffff, expected & 0x7fffffff);
        }

        #[test]
        fn prop_crc31_philips_with_params_matches_reference(data in proptest::collection::vec(any::<u8>(), 0..1024)) {
            let params = CRC31_PHILIPS;
            let our_result = checksum_with_params(params, &data);
            let mut crc = RUST_CRC31_PHILIPS.digest();
            crc.update(&data);
            let expected = crc.finalize() as u64;
            prop_assert_eq!(our_result & 0x7fffffff, expected & 0x7fffffff);
        }

        #[test]
        fn prop_crc31_custom_forward_matches_reference(data in proptest::collection::vec(any::<u8>(), 0..1024)) {
            let custom_params = CrcParams::new(
                "CRC-31/CUSTOM",
                31,
                0x04c11db7,
                0x7fffffff,
                false,
                0x7fffffff,
                0x0ce9e46c,
            );
            let our_result = checksum_with_params(custom_params, &data);
            let mut crc = RUST_CRC31_PHILIPS.digest();
            crc.update(&data);
            let expected = crc.finalize() as u64;
            prop_assert_eq!(our_result & 0x7fffffff, expected & 0x7fffffff);
        }

        #[test]
        fn prop_crc31_philips_checksum_combine_roundtrip(
            data1 in proptest::collection::vec(any::<u8>(), 0..512),
            data2 in proptest::collection::vec(any::<u8>(), 0..512)
        ) {
            let mut combined = Vec::new();
            combined.extend_from_slice(&data1);
            combined.extend_from_slice(&data2);

            let crc1 = checksum(CrcAlgorithm::Crc31Philips, &data1);
            let crc2 = checksum(CrcAlgorithm::Crc31Philips, &data2);
            let combined_via_combine = checksum_combine(CrcAlgorithm::Crc31Philips, crc1, crc2, data2.len() as u64);
            let combined_direct = checksum(CrcAlgorithm::Crc31Philips, &combined);

            prop_assert_eq!(combined_via_combine & 0x7fffffff, combined_direct & 0x7fffffff);
        }
    }
}
