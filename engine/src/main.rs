extern crate libvirt;
use libvirt::connection::*;

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
 }
