use rust_pcie_tlp::{RequestHeader, TlpHeader, TlpType};

fn main() {
    let foo = RequestHeader {
        hdr: TlpHeader {
            tlp_type: TlpType::IORdT,
            ..TlpHeader::default()
        },
        ..RequestHeader::default()
    };
    let bytes = foo.to_bytes();
    let new = RequestHeader::from_bytes(bytes).unwrap();
    dbg!(foo, new);
}
