extern crate libvirt;
extern crate interfaces;

use libvirt::connection::*;
use interfaces::{Interface};

fn main() {
    let conn: Connection = match Connection::new("qemu:///system".to_string(), ConnectionType::OPEN) {
        Ok(c) => c,
        Err(e) => panic!("Failed to connect to hypervisor: {}", e.message)
    };

    println!("Connected to hypervisor");
    //println!("Version is {}", conn.libvirt_version());
    let ver = match conn.libvirt_version() {
        Ok(c) => c,
        Err(e) => panic!("Failed to disconnect from hypervisor: {}", e.message)
    };
    
    println!("Version {}", ver); 

    match conn.close() {
        Ok(()) => println!("Disconnected from hypervisor"),
        Err(e) => panic!("Failed to disconnect from hypervisor: {}", e.message)
    };
    let ifname = "virbr0";
    let mut i = match Interface::get_by_name(ifname) {
        Ok(Some(i)) => i,
        Ok(None) => {
            println!("Could not find an interface named: {}", ifname);
            return;
        },
        Err(e) => {
            println!("An error occured fetching interfaces: {:?}", e);
            return;
        },
    };

    println!("Interface {} was {}", i.name, if i.is_up() { "up" } else { "down" });
 }
