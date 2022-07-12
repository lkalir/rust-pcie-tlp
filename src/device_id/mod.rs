//! Module containing DeviceID-related definitions and implementations

mod impls;

use crate::TlpError;
use byteorder::{BigEndian, ByteOrder};
use core::num::ParseIntError; // Using core instead of std for no_std support

#[cfg(test)]
use proptest_derive::Arbitrary;

/// Configuration space address that identifies a device on the PCIe fabric
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct DeviceID {
    /// PCIe bus
    pub bus: u8,
    /// PCIe device
    #[cfg_attr(test, proptest(strategy = "0u8..=31"))]
    pub device: u8,
    /// PCIe function
    #[cfg_attr(test, proptest(strategy = "0u8..=7"))]
    pub function: u8,
}

impl DeviceID {
    /// Returns a DeviceID if the parameters are valid, otherwise `Err`
    ///
    /// # Arguments
    /// * `bus` - PCIe bus
    /// * `device` - PCIe device, must be 0-31
    /// * `function` - PCIe function, must be 0-7
    ///
    /// # Examples
    /// ```
    /// # use rust_pcie_tlp::DeviceID;
    /// // Valid DeviceID
    /// let did = DeviceID::new(0, 1, 2);
    /// assert!(did.is_ok());
    /// // Invalid DeviceID
    /// let bad_did = DeviceID::new(255, 255, 255);
    /// assert!(bad_did.is_err());
    /// ```
    pub fn new(bus: u8, device: u8, function: u8) -> Result<Self, DeviceIDError> {
        if device > 31 || function > 7 {
            Err(DeviceIDError::OutOfRange)
        } else {
            Ok(Self {
                bus,
                device,
                function,
            })
        }
    }

    /// Converts a DeviceID into bytes buffer (big endian)
    ///
    /// You probably actually just want to convert this to a `u16`.
    ///
    /// # Examples
    /// ```
    /// # use rust_pcie_tlp::DeviceID;
    /// let did = DeviceID::new(0, 1, 2).unwrap();
    /// let bytes = did.to_bytes();
    /// assert_eq!(bytes, [0, 0x0A]);
    /// // Using Into<u16>
    /// let as_int: u16 = did.into();
    /// assert_eq!(as_int, 0x0A);
    /// ```
    pub fn to_bytes(&self) -> [u8; 2] {
        let x = (self.bus as u16) << 8 | (self.device as u16) << 3 | (self.function as u16);
        x.to_be_bytes()
    }

    /// Constructs a DeviceID from a big endian bytes buffer
    ///
    /// You probably actually just want to convert a `u16`.
    ///
    /// # Examples
    /// ```
    /// # use rust_pcie_tlp::DeviceID;
    /// let bytes = [0, 0x0A];
    /// let did = DeviceID::from_bytes(bytes);
    /// let expect = DeviceID::new(0, 1, 2).unwrap();
    /// assert_eq!(expect, did);
    /// // Using From<u16>
    /// let int = 0x0A_u16;
    /// let did = DeviceID::from(int);
    /// assert_eq!(expect, did);
    /// ```
    pub fn from_bytes(bytes: [u8; 2]) -> Self {
        let y = u16::from_be_bytes(bytes);
        let bus = ((y & 0xFF00) >> 8) as u8;
        let device = ((y & 0xFF) >> 3) as u8;
        let function = (y & 0x7) as u8;

        Self {
            bus,
            device,
            function,
        }
    }
}

impl TryFrom<&[u8]> for DeviceID {
    type Error = TlpError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        use core::cmp::Ordering;

        match value.len().cmp(&2) {
            Ordering::Less => Err(TlpError::TooShort),
            Ordering::Greater => Err(TlpError::TooLong),
            Ordering::Equal => {
                let did = BigEndian::read_u16(&value[0..2]);
                Ok(did.into())
            }
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DeviceIDError {
    IncorrectStrLen,
    InvalidFormat,
    OutOfRange,
    IntError(ParseIntError),
}

#[cfg(test)]
mod tests;
