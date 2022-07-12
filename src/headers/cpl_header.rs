use crate::{
    headers::{CompletionStatus, TlpHeader},
    DeviceID, TlpError,
};
use byteorder::{BigEndian, ByteOrder};

use num_traits::FromPrimitive;
#[cfg(test)]
use proptest_derive::Arbitrary;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct CplHeader {
    pub hdr: TlpHeader,
    pub cpl_id: DeviceID,
    #[cfg_attr(test, proptest(strategy = "0u16..4096"))]
    pub bc: u16,
    pub status: CompletionStatus,
    pub req_id: DeviceID,
    pub tag: u8,
    #[cfg_attr(test, proptest(strategy = "0u8..128"))]
    pub addr_low: u8,
}

impl CplHeader {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_hdr(mut self, hdr: TlpHeader) -> Self {
        self.hdr = hdr;
        self
    }

    pub fn with_cpl_id<T>(mut self, cpl_id: T) -> Self
    where
        T: Into<DeviceID>,
    {
        let cid = cpl_id.into();
        self.cpl_id = cid;
        self
    }

    pub fn with_bc(mut self, bc: u16) -> Result<Self, TlpError> {
        if bc > 4095 {
            Err(TlpError::TooLong)
        } else {
            self.bc = bc;
            Ok(self)
        }
    }

    pub fn with_status(mut self, status: CompletionStatus) -> Self {
        self.status = status;
        self
    }

    pub fn with_req_id<T>(mut self, req_id: T) -> Self
    where
        T: Into<DeviceID>,
    {
        let req = req_id.into();
        self.req_id = req;
        self
    }

    pub fn with_tag(mut self, tag: u8) -> Self {
        self.tag = tag;
        self
    }

    pub fn with_addr(mut self, addr_low: u8) -> Result<Self, TlpError> {
        if addr_low > 127 {
            Err(TlpError::TooLong)
        } else {
            self.addr_low = addr_low;
            Ok(self)
        }
    }

    pub fn to_bytes(&self) -> [u8; 12] {
        let mut ret = [0; 12];
        ret[0..4].clone_from_slice(&self.hdr.to_bytes());
        ret[4..6].clone_from_slice(&self.cpl_id.to_bytes());

        let bc_status: u16 = ((self.status as u16) << 13) | self.bc;
        ret[6..8].clone_from_slice(&bc_status.to_be_bytes());
        ret[8..10].clone_from_slice(&self.req_id.to_bytes());
        ret[10] = self.tag;
        ret[11] = self.addr_low;
        ret
    }

    pub fn from_bytes(bytes: [u8; 12]) -> Result<Self, TlpError> {
        let hdr = TlpHeader::try_from(&bytes[0..4])?;
        let cpl_id: DeviceID = BigEndian::read_u16(&bytes[4..6]).into();
        let bc_status = BigEndian::read_u16(&bytes[6..8]);
        let bc = bc_status & 0x1FFF;
        let status: CompletionStatus =
            FromPrimitive::from_u16((bc_status & 0xE000) >> 13).ok_or(TlpError::InvalidType)?;
        let req_id: DeviceID = BigEndian::read_u16(&bytes[8..10]).into();
        let tag = bytes[10];
        let addr_low = bytes[11];

        Ok(Self {
            hdr,
            cpl_id,
            bc,
            status,
            req_id,
            tag,
            addr_low,
        })
    }
}

impl TryFrom<[u8; 12]> for CplHeader {
    type Error = TlpError;

    fn try_from(value: [u8; 12]) -> Result<Self, Self::Error> {
        Self::from_bytes(value)
    }
}

impl TryFrom<&[u8]> for CplHeader {
    type Error = TlpError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        use core::cmp::Ordering;

        match value.len().cmp(&12) {
            Ordering::Less => Err(TlpError::TooShort),
            Ordering::Greater => Err(TlpError::TooLong),
            // SAFETY: Slice is already confirmed to be correct length
            Ordering::Equal => TryInto::<[u8; 12]>::try_into(value).unwrap().try_into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
            /// Roundtrip testing of header en/decoding
            #[test]
            fn cpl_hdr_serde_roundtrip(hdr: CplHeader) {
                let bytes = hdr.to_bytes();
                let new_hdr = CplHeader::from_bytes(bytes);
                assert!(new_hdr.is_ok());
                assert_eq!(hdr, new_hdr.unwrap());
            }
    }
}
