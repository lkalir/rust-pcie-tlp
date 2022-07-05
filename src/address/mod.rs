#[derive(Debug, Eq, PartialEq)]
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

impl<T> From<T> for Address
where
    T: Into<u64>,
{
    fn from(a: T) -> Self {
        let a64: u64 = a.into();
        Self::new(a64)
    }
}

#[cfg(test)]
mod tests;
