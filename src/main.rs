use std::fmt::{Debug, Display};

use rand::RngCore;

#[derive(Debug)]
struct MacAddress([u8; 6]);

impl Display for MacAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
            &self.0[0], &self.0[1], &self.0[2], &self.0[3], &self.0[4], &self.0[5]
        )
    }
}

impl MacAddress {
    ///               Universal Addresses Local Addresses  
    /// [000_000_00] |    Organization   |    Device     |   => First byte transmitter. [7]=>Unicast/Multicast Flag. [6]=>Local/Universal Flag
    /// [000_000_00] |    Organization   |    Device     |   
    /// [000_000_00] |    Organization   |    Device     |   
    /// [000_000_00] |       Device      |    Device     |                                             
    /// [000_000_00] |       Device      |    Device     |   
    /// [000_000_00] |       Device      |    Device     |   
    fn new() -> MacAddress {
        let mut octets = [0; 6];
        rand::thread_rng().fill_bytes(&mut octets);
        octets[0] |= 0b000_000_11;
        MacAddress(octets)
    }
    fn is_local(&self) -> bool {
        self.0[0] & 0b000_000_10 == 0b000_000_10
    }
    fn is_unicast(&self) -> bool {
        self.0[0] & 0b000_000_01 == 0b000_000_01
    }
}

fn main() {
    let mac_add = MacAddress::new();
    println!("Generated MAC: {}", mac_add);
    println!("{:?}", mac_add.is_local());
    println!("{:?}", mac_add.is_unicast());
}
