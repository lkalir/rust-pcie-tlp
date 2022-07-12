use super::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn into_addr_32_u8(addr in (0..=u8::MAX).prop_filter("Address must be dword aligned",
            |&x| (x as u64) & !ADDR64_MASK == 0)) {
        let a: Address = addr.try_into().unwrap();
        assert!(matches!(a, Address::Addr32 { .. }))
    }

    #[test]
    fn into_addr_32_u16(addr in (0..=u16::MAX).prop_filter("Address must be dword aligned",
            |&x| (x as u64) & !ADDR64_MASK == 0)) {
        let a: Address = addr.try_into().unwrap();
        assert!(matches!(a, Address::Addr32 { .. }))
    }

    #[test]
    fn into_addr_32_u32(addr in (0..=u32::MAX).prop_filter("Address must be dword aligned",
            |&x| (x as u64) & !ADDR64_MASK == 0)) {
        let a: Address = addr.try_into().unwrap();
        assert!(matches!(a, Address::Addr32 { .. }))
    }

    #[test]
    fn into_addr_32_u64(addr in (0..=(u32::MAX as u64)).prop_filter("Address must be dword aligned",
            |&x| (x as u64) & !ADDR64_MASK == 0)) {
        let a: Address = addr.try_into().unwrap();
        assert!(matches!(a, Address::Addr32 { .. }))
    }

    #[test]
    fn into_addr_64_u64(addr in ((u32::MAX as u64 + 1)..=u64::MAX)
        .prop_filter("Address must be dword aligned",|&x| (x as u64) & !ADDR64_MASK == 0)) {
        let a: Address = addr.try_into().unwrap();
        assert!(matches!(a, Address::Addr64 { .. }))
    }

    #[test]
    fn valid_addr_32(addr in (0..=u32::MAX).prop_map(|a| a & ADDR32_MASK)) {
        assert!(Address::is_valid_addr(addr))
    }

    #[test]
    fn invalid_addr_32((base, wrong) in (0..=u32::MAX).prop_flat_map(|addr| {
        (Just(addr & ADDR32_MASK), 1..=3u32)
    })) {
        let addr = base | wrong;
        assert!(!Address::is_valid_addr(addr));
        let a: Result<Address, TlpError> = addr.try_into();
        assert!(a.is_err());
        assert_eq!(TlpError::NotAligned, a.unwrap_err())
    }

    #[test]
    fn valid_addr_64(addr in (0..=u64::MAX).prop_map(|a| a & ADDR64_MASK)) {
        assert!(Address::is_valid_addr(addr))
    }

    #[test]
    fn invalid_addr_64((base, wrong) in (0..=u64::MAX).prop_flat_map(|addr| {
            (Just(addr & ADDR64_MASK), 1..=3u64)
    })) {
        let addr = base | wrong;
        assert!(!Address::is_valid_addr(addr));
        let a: Result<Address, TlpError> = addr.try_into();
        assert!(a.is_err());
        assert_eq!(TlpError::NotAligned, a.unwrap_err())
    }
}
