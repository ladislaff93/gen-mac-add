use clap::Parser;
use libc::{AF_INET, IPPROTO_UDP, SOCK_DGRAM};
use netdevice::{get_hardware, set_hardware};
use rand::RngCore;
use std::fmt::{Debug, Display};
use std::io::Error;
use std::process::Command;

#[derive(Parser, Debug)]
#[command(author = "Ladislav Baculak")]
#[command(version = "0.0.1")]
#[command(long_about = "Set a random MAC Address to A specific device interface")]
struct Args {
    /// Name of the hadrware you want to change MAC Address on.
    name_of_interface: String,
    /// Make the MAC Address Unicast or Multicast. Default Unicast
    #[arg(short)]
    multicast: bool,
    /// Make the MAC Address Local default Universal. Default Universal
    #[arg(short)]
    universal: bool,
}

#[derive(Debug, Clone)]
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

impl Into<[i8; 14]> for MacAddress {
    fn into(self) -> [i8; 14] {
        let mut out = [0i8; 14];

        for (n, b) in self.0.iter().enumerate() {
            out[n] = *b as i8;
        }

        out
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
        octets[0] &= 0xfc; // default MAC set to unicast 0b111_111_00
        octets[0] |= 0x2; // default MAC set to local binary 0b000_000_10
        MacAddress(octets)
    }
    fn set_universal(&mut self) {
        self.0[0] ^= 0b000_000_10;
    }
    fn set_multicast(&mut self) {
        self.0[0] |= 0b000_000_01;
    }
}

fn main() {
    let cli = Args::parse();
    let name_of_iterface = cli.name_of_interface;
    let multicast = cli.multicast;
    let universal = cli.universal;
    let mut mac_add = MacAddress::new();

    if multicast == true {
        mac_add.set_multicast();
    }
    if universal == true {
        mac_add.set_universal();
    }
    // create socket for communicating between process and system network interface
    let res = unsafe { libc::socket(AF_INET, SOCK_DGRAM, IPPROTO_UDP) };
    if res == -1 {
        panic!("{}", Error::last_os_error())
    }
    let mut old_addr =
        get_hardware(res, &name_of_iterface).expect("Unable to get requested interface!");

    old_addr.sa_data = mac_add.clone().into();
    set_hardware(res, &name_of_iterface, old_addr)
        .expect("Unable to set the requested MAC address");

    println!("New MAC address assign: {}", mac_add);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_mac_add() {
        let add = MacAddress::new();
        let binary_representation = format!("{:08b}", add.0[0]);
        // check if its unicast
        assert_eq!(binary_representation.chars().nth(7).unwrap(), '0');
        // check if its local
        assert_eq!(binary_representation.chars().nth(6).unwrap(), '1');
    }
    #[test]
    fn new_mac_make_universal() {
        let mut add = MacAddress::new();
        add.set_universal();
        let binary_representation = format!("{:08b}", add.0[0]);
        // check if its universal
        assert_eq!(binary_representation.chars().nth(6).unwrap(), '0');
    }

    #[test]
    fn new_mac_make_multicast() {
        let mut add = MacAddress::new();
        add.set_multicast();
        let binary_representation = format!("{:08b}", add.0[0]);
        // check if its multicast
        assert_eq!(binary_representation.chars().nth(7).unwrap(), '1');
    }
}
