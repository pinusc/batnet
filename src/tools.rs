use net;
use std::net::ToSocketAddrs;
use std::env::args;

/// Takes a Vec<u8> and returns a clone of it with all the \0 removed
pub fn trim_bytes(vec: Vec<u8>) -> Vec<u8>{
    let mut res = vec![];
    for i in vec {
        if i != 0 {
            res.push(i);
        }
    }
    res
}

pub fn get_own_ip() -> Option<net::Ipv4Addr> {
    let str_addr = args().nth(1);
    match str_addr {
        Some(str_addr) => {
            let addr = str_addr.to_socket_addrs();
            if let Ok(mut addr) = addr {
                if let Some(foo) = addr.next() {
                    let ip = foo.ip();
                    match ip {
                        net::IpAddr::V4(a) => Some(a),
                        _ => None
                    }
                } else {
                    None
                }
            } else {
                None
            }
        },
        None => None
    }
}
