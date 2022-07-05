use rust_pcie_tlp::DeviceID;
use std::str::FromStr;

fn main() {
    let did = DeviceID::new(1, 2, 3).unwrap();
    println!("{}", did);
    println!("{:?}", did);
    println!("{:#?}", did);
    let did = DeviceID::from_str("01:01.1").unwrap();
    println!("{}", did);
}
