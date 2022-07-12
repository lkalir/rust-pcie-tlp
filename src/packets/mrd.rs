use crate::{Address, DeviceID, RequestHeader, TlpError, TlpHeader, TlpType};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MRd {
    pub hdr: RequestHeader,
    pub addr: Address,
}

impl Default for MRd {
    fn default() -> Self {
        let mut hdr = RequestHeader {
            hdr: TlpHeader {
                tlp_type: TlpType::MRd3,
                ..Default::default()
            },
            ..Default::default()
        };
        hdr.set_byte_enables();

        Self {
            hdr,
            addr: Default::default(),
        }
    }
}

impl MRd {
    pub fn new(req_id: DeviceID, tag: u8, addr: u64, length: u16) -> Result<Self, TlpError> {
        let addr = Address::try_from(addr)?;
        let hdr = TlpHeader::new()
            .with_type(if let Address::Addr32(_) = addr {
                TlpType::MRd3
            } else {
                TlpType::MRd4
            })
            .with_length(length)?;

        Ok(Self {
            hdr: RequestHeader::new()
                .with_hdr(hdr)
                .with_tag(tag)
                .with_byte_enables()
                .with_req_id(req_id),
            addr,
        })
    }
}
