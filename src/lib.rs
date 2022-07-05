#![cfg_attr(not(test), no_std)]

mod address;
mod device_id;

pub use address::Address;
pub use device_id::DeviceID;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

#[cfg(test)]
use proptest_derive::Arbitrary;

/// Data word size in bytes
pub const DWORD_LEN: usize = 4;

/// Maximum TLP payload size in bytes
pub const MAX_DATA_LEN: usize = 1024 * DWORD_LEN;

/// Maximum size of a TLP with data in bytes
pub const MAX_TLP_BUFFER: usize = 4 * DWORD_LEN + MAX_DATA_LEN * DWORD_LEN;

/// TLP header types
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum TlpFormat {
    /// 3 data word header with no payload
    NoData3DW = 0b000,
    /// 4 data word header with no payload
    NoData4DW = 0b001,
    /// 3 data word header with payload
    Data3DW = 0b010,
    /// 4 data word header with payload
    Data4DW = 0b011,
    TlpPrefix = 0b100,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, FromPrimitive)]
#[cfg_attr(test, derive(Arbitrary))]
#[repr(u8)]
pub enum AddressType {
    DefaultUntranslated = 0b00,
    TranslationRequest = 0b01,
    Translated = 0b10,
    AddressTypeReserved = 0b11,
}

impl Default for AddressType {
    fn default() -> Self {
        Self::DefaultUntranslated
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, FromPrimitive)]
#[cfg_attr(test, derive(Arbitrary))]
#[repr(u8)]
pub enum TrafficClass {
    TC0 = 0,
    TC1 = 1,
    TC2 = 2,
    TC3 = 3,
    TC4 = 4,
    TC5 = 5,
    TC6 = 6,
    TC7 = 7,
}

impl Default for TrafficClass {
    fn default() -> Self {
        Self::TC0
    }
}

#[derive(Debug)]
#[repr(u8)]
pub enum CompletionStatus {
    SuccessfulCompletion = 0b000,
    UnsupportedRequest = 0b001,
    ConfigurationRequestRetry = 0b010,
    CompleterAbort = 0b100,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, FromPrimitive)]
#[cfg_attr(test, derive(Arbitrary))]
#[repr(u8)]
pub enum TlpType {
    /// Memory read request, 3 data words
    MRd3 = (TlpFormat::NoData3DW as u8) << 5,
    /// Memory read request, 4 data words
    MRd4 = (TlpFormat::NoData4DW as u8) << 5,
    /// Memory read request-locked, 3 data words
    MRdLk3 = (TlpFormat::NoData3DW as u8) << 5 | 1,
    /// Memory read request-locked, 4 data words
    MRdLk4 = (TlpFormat::NoData4DW as u8) << 5 | 1,
    /// Memory write request, 3 data words
    MWr3 = (TlpFormat::Data3DW as u8) << 5,
    /// Memory write request, 4 data words
    MWr4 = (TlpFormat::Data4DW as u8) << 5,
    /// I/O read request
    IORdT = (TlpFormat::NoData3DW as u8) << 5 | 0b10,
    /// I/O write request
    IOWrtT = (TlpFormat::Data3DW as u8) << 5 | 0b10,
    /// Configuration read type 0
    CfgRd0 = (TlpFormat::NoData3DW as u8) << 5 | 0b100,
    /// Configuration write type 0
    CfgWr0 = (TlpFormat::Data3DW as u8) << 5 | 0b100,
    /// Configuration read type 1
    CfgRd1 = (TlpFormat::NoData3DW as u8) << 5 | 0b101,
    /// Configuration write type 1
    CfgWr1 = (TlpFormat::Data3DW as u8) << 5 | 0b101,
    /// Completion without data
    CplE = (TlpFormat::NoData3DW as u8) << 5 | 0b1010,
    /// Completion with data
    CplD = (TlpFormat::Data3DW as u8) << 5 | 0b1010,
    /// Completion without data for locked memory read
    CplLk = (TlpFormat::NoData3DW as u8) << 5 | 0b1011,
    /// Completion with data for locked memory read
    CplLkD = (TlpFormat::Data3DW as u8) << 5 | 0b1011,
    /// Multi-root I/O virtualization and sharing
    MRIOV = (TlpFormat::TlpPrefix as u8) << 5,
    /// Local TLP prefix with vendor subfield
    LocalVendPrefix = (TlpFormat::TlpPrefix as u8) << 5 | 0b1110,
    /// Extended TLP
    ExtTPH = (TlpFormat::TlpPrefix as u8) << 5 | 0b10000,
    /// Process address space id
    PASID = (TlpFormat::TlpPrefix as u8) << 5 | 0b10001,
    /// End-to-end TLP with vendor subfield
    EndEndVendPrefix = (TlpFormat::TlpPrefix as u8) << 5 | 0b11110,
}

impl Default for TlpType {
    fn default() -> Self {
        Self::MRd3
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct TlpHeader {
    /// Format and type
    tlp_type: TlpType,
    /// Traffic class
    tc: TrafficClass,
    /// LN read or write
    ln: bool,
    /// TLP processing hints presence
    th: bool,
    /// TLP digest presence
    td: bool,
    /// TLP poison indicator
    ep: bool,
    /// No-snoop
    ns: bool,
    /// Relaxed ordering
    ro: bool,
    /// Id-based ordering
    ibo: bool,
    /// Address type
    at: AddressType,
    /// Length of payload in dwords
    #[cfg_attr(test, proptest(strategy = "0u16..1024"))]
    length: u16,
}

#[derive(Debug, Eq, PartialEq)]
pub enum TlpError {
    InvalidType,
    NotAligned,
    TooLong,
}

impl TlpHeader {
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

    pub fn to_bytes(&self) -> [u8; 4] {
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

    pub fn from_bytes(bytes: [u8; 4]) -> Result<Self, TlpError> {
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

#[cfg(test)]
mod tests;
