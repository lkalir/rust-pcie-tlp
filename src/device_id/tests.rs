use super::*;
use proptest::prelude::*;
use std::str::FromStr;

proptest! {
    /// Tests that in-bounds parameters produces a DeviceID
    #[test]
    fn device_id_bounds_happy(bus in 0..=255u8, device in 0..=31u8, function in 0..=7u8) {
        let did = DeviceID::new(bus, device, function);
        assert!(did.is_ok());
    }

    /// Tests that out of bounds parameters for a DeviceID are rejected
    #[test]
    fn device_id_bounds_unhappy(bus in 0..=255u8, (device, function) in
        (0u8.., 0u8..).prop_filter("One of device or function must be outside acceptable range",
            |&(dev, fun)| (dev > 31 && fun <= 7) || (dev <= 31 && fun > 7))) {
        let did = DeviceID::new(bus, device, function);
        assert!(did.is_err());
    }

    /// Roundtrip testing of int en/decoding
    #[test]
    fn device_id_int_roundtrip(did: DeviceID) {
        let idid: u16 = did.into();
        let redid: DeviceID = idid.into();
        assert_eq!(did, redid);
    }

    /// Roundtrip testing of str en/decoding
    #[test]
    fn device_id_str_roundtrip(did: DeviceID) {
        let sdid = did.to_string();
        let redid = DeviceID::from_str(&sdid);
        assert!(redid.is_ok());
        assert_eq!(did, redid.unwrap())
    }

    /// Tests that a valid DeviceID string is correctly parsed
    #[test]
    fn str_serde_happy(bus in 0..=255u8, device in 0..=31u8, function in 0..=7u8) {
        let text = format!("{:02X}:{:02X}.{:01X}", bus, device, function);
        let did = DeviceID::from_str(&text);
        assert!(did.is_ok());
        let did = did.unwrap();
        let expect = DeviceID { bus, device, function };
        assert_eq!(expect, did);
    }

    /// Tests that a invalid DeviceID but properly formatted DeviceID string is rejected
    #[test]
    fn str_serde_out_of_range(bus in 0..=255u8, (device, function) in
        (0u8.., 0u8..0xF).prop_filter("One of device or function must be outside acceptable range",
            |&(dev, fun)| (dev > 31 && fun <= 7) || (dev <= 31 && fun > 7))) {
        let text = format!("{:02X}:{:02X}.{:01X}", bus, device, function);
        let did = DeviceID::from_str(&text);
        assert!(did.is_err());
        assert_eq!(did.unwrap_err(), DeviceIDError::OutOfRange);
    }

    /// Tests that a string that is incorrectly sized is rejected
    #[test]
    fn str_serde_wrong_len_too_short(text in ".{0,6}") {
        let did = DeviceID::from_str(&text);
        assert!(did.is_err());
        assert_eq!(did.unwrap_err(), DeviceIDError::IncorrectStrLen);
    }

    /// Tests that a string that is incorrectly sized is rejected
    #[test]
    fn str_serde_wrong_len_too_long(text in ".{8}.*") {
        let did = DeviceID::from_str(&text);
        assert!(did.is_err());
        assert_eq!(did.unwrap_err(), DeviceIDError::IncorrectStrLen);
    }
}

/// Tests that a string of the right length but improperly formatted is rejected
#[test]
fn str_serde_wrong_format() {
    let did = DeviceID::from_str("foobar!");
    assert!(did.is_err());
    assert_eq!(did.unwrap_err(), DeviceIDError::InvalidFormat);
}

/// Tests that a string of the right length and almost the proper format is rejected
#[test]
fn str_serde_bad_hex() {
    let did = DeviceID::from_str("fo:ob.a");
    assert!(did.is_err());

    if let DeviceIDError::IntError(ie) = did.unwrap_err() {
        assert_eq!(*ie.kind(), std::num::IntErrorKind::InvalidDigit);
    } else {
        panic!("Somehow some other error occurred")
    }
}
