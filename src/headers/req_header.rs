use crate::{
    headers::{TlpError, TlpHeader},
    DeviceID,
};
use byteorder::{BigEndian, ByteOrder};

#[cfg(test)]
use proptest_derive::Arbitrary;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct RequestHeader {
    pub hdr: TlpHeader,
    pub req_id: DeviceID,
    pub tag: u8,
    #[cfg_attr(test, proptest(strategy = "0u8..16"))]
    pub first_be: u8,
    #[cfg_attr(test, proptest(strategy = "0u8..16"))]
    pub last_be: u8,
}

impl RequestHeader {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_hdr(mut self, hdr: TlpHeader) -> Self {
        self.hdr = hdr;
        self
    }

    pub fn with_req_id<T>(mut self, req_id: T) -> Self
    where
        T: Into<DeviceID>,
    {
        let did = req_id.into();
        self.req_id = did;
        self
    }

    pub fn with_tag(mut self, tag: u8) -> Self {
        self.tag = tag;
        self
    }

    pub fn with_first_be(mut self, first_be: u8) -> Result<Self, TlpError> {
        if first_be > 0xF {
            Err(TlpError::TooLong)
        } else {
            self.first_be = first_be;
            Ok(self)
        }
    }

    pub fn with_last_be(mut self, last_be: u8) -> Result<Self, TlpError> {
        if last_be > 0xF {
            Err(TlpError::TooLong)
        } else {
            self.last_be = last_be;
            Ok(self)
        }
    }

    pub fn with_byte_enables(mut self) -> Self {
        self.set_byte_enables();
        self
    }

    pub fn to_bytes(&self) -> [u8; 8] {
        let mut ret = [0; 8];
        ret[0..4].clone_from_slice(&self.hdr.to_bytes());
        ret[4..6].clone_from_slice(&self.req_id.to_bytes());
        ret[6] = self.tag;
        ret[7] = (self.last_be << 4) | self.first_be;
        ret
    }

    pub fn from_bytes(bytes: [u8; 8]) -> Result<Self, TlpError> {
        let hdr = TlpHeader::try_from(&bytes[0..4])?;
        let req_id: DeviceID = BigEndian::read_u16(&bytes[4..6]).into();
        let tag = bytes[6];
        let first_be = bytes[7] & 0xF;
        let last_be = (bytes[7] & 0xF0) >> 4;

        Ok(Self {
            hdr,
            req_id,
            tag,
            first_be,
            last_be,
        })
    }

    pub fn set_byte_enables(&mut self) {
        self.first_be = 0xF;
        self.last_be = match self.hdr.length {
            1 => 0,
            _ => 0xF,
        }
    }
}

impl TryFrom<[u8; 8]> for RequestHeader {
    type Error = TlpError;

    fn try_from(value: [u8; 8]) -> Result<Self, Self::Error> {
        Self::from_bytes(value)
    }
}

impl TryFrom<&[u8]> for RequestHeader {
    type Error = TlpError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        use core::cmp::Ordering;

        match value.len().cmp(&8) {
            Ordering::Less => Err(TlpError::TooShort),
            Ordering::Greater => Err(TlpError::TooLong),
            // SAFETY: slice already confirmed to be correct length
            Ordering::Equal => TryInto::<[u8; 8]>::try_into(value).unwrap().try_into(),
        }
    }
}

impl From<RequestHeader> for [u8; 8] {
    fn from(hdr: RequestHeader) -> Self {
        hdr.to_bytes()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        /// Roundtrip testing of header en/decoding
        #[test]
        fn req_hdr_serde_roundtrip(hdr: RequestHeader) {
            let bytes = hdr.to_bytes();
            let new_hdr = RequestHeader::from_bytes(bytes);
            assert!(new_hdr.is_ok());
            assert_eq!(hdr, new_hdr.unwrap());
        }

        #[test]
        fn req_hdr_first_be_too_large(first_be in 0x10u8..) {
            let req = RequestHeader::new().with_first_be(first_be);
            assert!(req.is_err());
            assert_eq!(TlpError::TooLong, req.unwrap_err());
        }

        #[test]
        fn req_hdr_last_be_too_large(last_be in 0x10u8..) {
            let req = RequestHeader::new().with_last_be(last_be);
            assert!(req.is_err());
            assert_eq!(TlpError::TooLong, req.unwrap_err());
        }
    }

    #[test]
    fn req_hdr_from_slice_too_short() {
        let v = vec![1, 2, 3, 4, 5, 6, 7];
        let e = RequestHeader::try_from(v.as_slice());
        assert!(e.is_err());
        assert_eq!(TlpError::TooShort, e.unwrap_err());
    }

    #[test]
    fn req_hdr_from_slice_too_long() {
        let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let e = RequestHeader::try_from(v.as_slice());
        assert!(e.is_err());
        assert_eq!(TlpError::TooLong, e.unwrap_err());
    }
}
