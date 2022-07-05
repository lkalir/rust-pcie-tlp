use rust_pcie_tlp::TlpHeader;

fn main() {
    let hdr = TlpHeader::new();
    println!("{:#?}", hdr);

    let hdr = TlpHeader::from_bytes([0, 1, 2, 3]);
    assert!(hdr.is_ok());
    println!("{:#?}", hdr.unwrap());
}
