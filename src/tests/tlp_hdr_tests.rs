use super::super::*;
use proptest::prelude::*;

proptest! {
    /// Roundtrip testing of header en/decoding
    #[test]
    fn tlp_hdr_serde_roundtrip(hdr: TlpHeader) {
        let bytes = hdr.to_bytes();
        let new_hdr = TlpHeader::from_bytes(bytes);
        assert!(new_hdr.is_ok());
        assert_eq!(hdr, new_hdr.unwrap());
    }

    /// Tests that a length that is too long is rejected as invalid
    #[test]
    fn tlp_hdr_len_too_long(i in 4097u16..) {
        let hdr = TlpHeader::default().with_length(i);
        assert!(hdr.is_err());
        assert_eq!(TlpError::TooLong, hdr.unwrap_err());
    }

    /// Tests that a length that is not dword-aligned is rejected as invalid
    #[test]
    fn tlp_hdr_unaligned(i in (0u16..=4096).prop_filter("Length must be misaligned",
            |x| x % 4 != 0)) {
        let hdr = TlpHeader::default().with_length(i);
        assert!(hdr.is_err());
        assert_eq!(TlpError::NotAligned, hdr.unwrap_err());
    }
}
