#[cfg(test)]
mod tests;

use crate::TlpError;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Address {
    Addr32(u32),
    Addr64(u64),
}

const ADDR32_MASK: u32 = 0xFFFFFFFC;
const ADDR64_MASK: u64 = 0xFFFFFFFFFFFFFFFC;

impl Address {
    pub fn new(addr: u64) -> Self {
        if addr > u32::MAX as u64 {
            Address::Addr64(addr & ADDR64_MASK)
        } else {
            let a32 = (addr & (u32::MAX as u64)) as u32;
            Address::Addr32(a32 & ADDR32_MASK)
        }
    }

    pub fn is_valid_addr<T>(addr: T) -> bool
    where
        T: Into<u64>,
    {
        let a64: u64 = addr.into();
        a64 & !ADDR64_MASK == 0
    }
}

macro_rules! impl_addr_try_from {
    ($($ty:ty),+) => {
        $(
        impl TryFrom<$ty> for Address {
            type Error = TlpError;

            fn try_from(value: $ty) -> Result<Self, Self::Error> {
                let v = value as u64;

                if Self::is_valid_addr(v) {
                    Ok(Self::new(v))
                } else {
                    Err(TlpError::NotAligned)
                }
            }
        })+

    };
}

impl_addr_try_from!(u8, u16, u32, u64, usize);

impl Default for Address {
    fn default() -> Self {
        Self::Addr32(0)
    }
}

