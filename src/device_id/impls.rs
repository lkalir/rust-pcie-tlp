use super::{DeviceID, DeviceIDError};

// Using core instead of std for no_std support
use core::{fmt, num::ParseIntError, str::FromStr};

impl From<u16> for DeviceID {
    fn from(i: u16) -> Self {
        Self::from_bytes(i.to_be_bytes())
    }
}

impl From<DeviceID> for u16 {
    fn from(did: DeviceID) -> Self {
        Self::from_be_bytes(did.to_bytes())
    }
}

impl fmt::Display for DeviceIDError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DeviceIDError::IncorrectStrLen => f.write_str("inccorect string length"),
            DeviceIDError::InvalidFormat => f.write_str("invalid string format"),
            DeviceIDError::OutOfRange => f.write_str("out of bounds"),
            DeviceIDError::IntError(ie) => f.write_fmt(format_args!("{}", ie)),
        }
    }
}

impl From<ParseIntError> for DeviceIDError {
    fn from(e: ParseIntError) -> Self {
        Self::IntError(e)
    }
}

impl FromStr for DeviceID {
    type Err = DeviceIDError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.chars().count() != 7 {
            return Err(DeviceIDError::IncorrectStrLen);
        }

        let mut chs = s.chars();

        if chs.nth(2) != Some(':') || chs.nth(2) != Some('.') {
            return Err(DeviceIDError::InvalidFormat);
        }

        let bus = u8::from_str_radix(&s[0..=1], 16)?;
        let device = u8::from_str_radix(&s[3..=4], 16)?;
        let function = u8::from_str_radix(&s[6..=6], 16)?;
        Self::new(bus, device, function)
    }
}

impl fmt::Display for DeviceID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!(
            "{:02X}:{:02X}.{:01X}",
            self.bus, self.device, self.function
        ))
    }
}
