mod cpl_header;
mod req_header;
mod tlp_header;

use num_derive::FromPrimitive;

#[cfg(test)]
use proptest_derive::Arbitrary;

pub use cpl_header::CplHeader;
pub use req_header::RequestHeader;
pub use tlp_header::TlpHeader;

#[derive(Debug, Eq, PartialEq)]
pub enum TlpError {
    InvalidType,
    NotAligned,
    TooLong,
    TooShort,
}

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

#[derive(Clone, Copy, Debug, Eq, PartialEq, FromPrimitive)]
#[cfg_attr(test, derive(Arbitrary))]
#[repr(u8)]
pub enum CompletionStatus {
    SuccessfulCompletion = 0b000,
    UnsupportedRequest = 0b001,
    ConfigurationRequestRetry = 0b010,
    CompleterAbort = 0b100,
}

impl Default for CompletionStatus {
    fn default() -> Self {
        Self::SuccessfulCompletion
    }
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
