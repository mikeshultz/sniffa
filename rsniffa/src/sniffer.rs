use etherparse::{InternetSlice, SlicedPacket};
use pcap::{Active, Address, Capture, Device, Packet, PacketCodec};
use pyo3::prelude::*;
use std::net::IpAddr;

#[pyclass]
pub struct TextSniffer {
    capture: Capture<Active>,
    device_addresses: Vec<Address>,
}

#[pymethods]
impl TextSniffer {
    #[new]
    fn new() -> Self {
        let cap = Device::lookup().unwrap().unwrap().open().unwrap();
        Self {
            capture: cap,
            device_addresses: Device::lookup().unwrap().unwrap().addresses,
        }
    }

    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(mut slf: PyRefMut<'_, Self>) -> Option<String> {
        loop {
            let packet = slf.capture.next_packet();
            if let Ok(p) = packet {
                match SlicedPacket::from_ethernet(p.data) {
                    Err(value) => println!("Err {:?}", value),
                    Ok(value) => {
                        if let Some(net) = value.net {
                            match net {
                                InternetSlice::Ipv4(ip4) => {
                                    let source = ip4.header().source_addr();
                                    let dest = ip4.header().destination_addr();
                                    if is_me(&slf.device_addresses, IpAddr::V4(source)) {
                                        //println!("{:?} -> {:?}", source, dest);
                                        return Some(format!("{:?} -> {:?}", source, dest));
                                    } else {
                                        //println!("{:?} <- {:?}", source, dest);
                                        return Some(format!("{:?} <- {:?}", source, dest));
                                    }
                                }
                                InternetSlice::Ipv6(ip6) => {
                                    let source = ip6.header().source_addr();
                                    let dest = ip6.header().destination_addr();
                                    if is_me(&slf.device_addresses, IpAddr::V6(source)) {
                                        //println!("{:?} -> {:?}", source, dest);
                                        return Some(format!("{:?} -> {:?}", source, dest));
                                    } else {
                                        //println!("{:?} <- {:?}", source, dest);
                                        return Some(format!("{:?} <- {:?}", source, dest));
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Just to quiet the compiler
            if false {
                break;
            }
        }

        None
    }
}

fn is_me(my_addresses: &Vec<Address>, address: IpAddr) -> bool {
    for my_address in my_addresses {
        if my_address.addr == address {
            return true;
        }
    }
    return false;
}
