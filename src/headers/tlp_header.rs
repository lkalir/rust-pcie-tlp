use crate::{
    headers::{AddressType, TlpError, TlpType, TrafficClass},
    DWORD_LEN, MAX_DATA_LEN,
};
use num_traits::FromPrimitive;

#[cfg(test)]
use proptest_derive::Arbitrary;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct TlpHeader {
    /// Format and type
    pub tlp_type: TlpType,
    /// Traffic class
    pub tc: TrafficClass,
    /// LN read or write
    pub ln: bool,
    /// TLP processing hints presence
    pub th: bool,
    /// TLP digest presence
    pub td: bool,
    /// TLP poison indicator
    pub ep: bool,
    /// No-snoop
    pub ns: bool,
    /// Relaxed ordering
    pub ro: bool,
    /// Id-based ordering
    pub ibo: bool,
    /// Address type
    pub at: AddressType,
    /// Length of payload in dwords
    #[cfg_attr(test, proptest(strategy = "0u16..1024"))]
    pub length: u16,
}

impl TlpHeader {
    pub const LENGTH: usize = 4;

    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_type(mut self, tlp_type: TlpType) -> Self {
        self.tlp_type = tlp_type;
        self
    }

    pub fn with_tc(mut self, tc: TrafficClass) -> Self {
        self.tc = tc;
        self
    }

    pub fn with_ln(mut self, ln: bool) -> Self {
        self.ln = ln;
        self
    }

    pub fn with_th(mut self, th: bool) -> Self {
        self.th = th;
        self
    }

    pub fn with_td(mut self, td: bool) -> Self {
        self.td = td;
        self
    }

    pub fn with_ep(mut self, ep: bool) -> Self {
        self.ep = ep;
        self
    }

    pub fn with_ns(mut self, ns: bool) -> Self {
        self.ns = ns;
        self
    }

    pub fn with_ro(mut self, ro: bool) -> Self {
        self.ro = ro;
        self
    }

    pub fn with_ibo(mut self, ibo: bool) -> Self {
        self.ibo = ibo;
        self
    }

    pub fn with_at(mut self, at: AddressType) -> Self {
        self.at = at;
        self
    }

    pub fn with_length(mut self, len: u16) -> Result<Self, TlpError> {
        if usize::from(len) > MAX_DATA_LEN {
            Err(TlpError::TooLong)
        } else if len & 0x3 > 0 {
            Err(TlpError::NotAligned)
        } else {
            self.length = match len >> 2 {
                1024 => 0,
                l => l,
            };
            Ok(self)
        }
    }

    pub fn to_bytes(&self) -> [u8; Self::LENGTH] {
        let tlp_type = self.tlp_type as u8;
        let attrs1 =
            (self.tc as u8) << 4 | (self.ibo as u8) << 2 | (self.ln as u8) << 1 | (self.th as u8);
        let attrs2 = (self.td as u8) << 7
            | (self.ep as u8) << 6
            | (self.ro as u8) << 5
            | (self.ns as u8) << 4
            | (self.at as u8) << 2
            | ((self.length >> 8) & 0x3) as u8;
        let lower_len = (self.length & 0xFF) as u8;
        [tlp_type, attrs1, attrs2, lower_len]
    }

    pub fn from_bytes(bytes: [u8; Self::LENGTH]) -> Result<Self, TlpError> {
        let tlp_type = TlpType::from_u8(bytes[0]).ok_or(TlpError::InvalidType)?;
        // SAFETY: All combinations of bits in the TC field are valid
        let tc = TrafficClass::from_u8((bytes[1] & 0x70) >> 4).unwrap();
        let ibo = (bytes[1] & 0x4) > 0;
        let ln = (bytes[1] & 0x2) > 0;
        let th = (bytes[1] & 0x1) > 0;
        let td = (bytes[2] & 0x80) > 0;
        let ep = (bytes[2] & 0x40) > 0;
        let ro = (bytes[2] & 0x20) > 0;
        let ns = (bytes[2] & 0x10) > 0;
        // SAFETY: All combinations of bits in the AT field are valid
        let at = AddressType::from_u8((bytes[2] & 0xC) >> 2).unwrap();
        let length = u16::from_be_bytes([bytes[2] & 0x3, bytes[3]]);

        Ok(TlpHeader {
            tlp_type,
            tc,
            ibo,
            ln,
            th,
            td,
            ep,
            ro,
            ns,
            at,
            length,
        })
    }

    pub fn data_len(&self) -> u16 {
        let x = match self.length {
            0 => 1024,
            l => l,
        };

        x * (DWORD_LEN as u16)
    }
}

impl TryFrom<[u8; Self::LENGTH]> for TlpHeader {
    type Error = TlpError;

    fn try_from(value: [u8; Self::LENGTH]) -> Result<Self, Self::Error> {
        Self::from_bytes(value)
    }
}

impl TryFrom<&[u8]> for TlpHeader {
    type Error = TlpError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        use core::cmp::Ordering;

        match value.len().cmp(&Self::LENGTH) {
            Ordering::Less => Err(TlpError::TooShort),
            Ordering::Greater => Err(TlpError::TooLong),
            // SAFETY: Slice is already confirmed to be correct length
            Ordering::Equal => TryInto::<[u8; Self::LENGTH]>::try_into(value).unwrap().try_into(),
        }
    }
}

impl From<TlpHeader> for [u8; TlpHeader::LENGTH] {
    fn from(hdr: TlpHeader) -> Self {
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

    #[test]
    fn tlp_hdr_from_slice_too_short() {
        let v = vec![1, 2, 3];
        let e = TlpHeader::try_from(v.as_slice());
        assert!(e.is_err());
        assert_eq!(TlpError::TooShort, e.unwrap_err());
    }

    #[test]
    fn tlp_hdr_from_slice_too_long() {
        let v = vec![1, 2, 3, 4, 5];
        let e = TlpHeader::try_from(v.as_slice());
        assert!(e.is_err());
        assert_eq!(TlpError::TooLong, e.unwrap_err());
    }
}
