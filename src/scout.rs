//! Responsible for the discovery of new peers. 
//! Works by: 
//! - Keeping a list of ranked known peers
//! - Asking connected peers to share their list
//! - Knocking on PORT on every address in a specified range 
use tools;
use peer;

use std::thread;
use std::net::TcpStream;
use serde_json;
use std::str;
use std::io::{Read,Write};
use std::net;

/// Given a Ipv4Addr, return a new Ipv4 incremented by 1.
/// If the address given is = std::net::Ipv4Addr::new(255, 255, 255, 255),
/// it returns std::net::Ipv4Addr::new(0, 0, 0, 0).
/// # Examples
/// ```
/// let addr = std::net::Ipv4Addr::new(234, 45, 12, 255);
/// let addr_incremented = increment_ip(addr);
/// assert_eq!(addr_incremented, std::net::Ipv4Addr::new(234, 45, 13, 0);
/// let addr_incremented_2 = increment_ip(addr_incremented);
/// assert_eq!(addr_incremented_2, std::net::Ipv4Addr::new(234, 45, 13, 1);
/// ```
fn increment_ip(addr: net::Ipv4Addr) -> net::Ipv4Addr {
    let mut prev_octets = addr.octets();
    for i in (0..4).rev() {
        if prev_octets[i] < 255 {
            prev_octets[i] += 1;
            break;
        } else {
            prev_octets[i] = 0;
        }
    }
    let a = prev_octets[0];
    let b = prev_octets[1];
    let c = prev_octets[2];
    let d = prev_octets[3];
    net::Ipv4Addr::new(a,b,c,d)
}

pub fn brute(start: net::Ipv4Addr, end: net::Ipv4Addr) {
    assert_eq!(start, net::Ipv4Addr::new(127,0,0,1));
    let mut connected = vec![];
    let mut cur_addr = start;
    let own_addr = tools::get_own_ip();
    while cur_addr <= end {
        if let Some(own_addr) = own_addr {
            // println!("{}", own_addr);
            if own_addr == cur_addr {
                continue;
            }
        }
        if let Ok(mut stream) = TcpStream::connect((cur_addr, 6667)){
            println!("Connected to {}:6667", cur_addr.to_string());
            let _ = stream.write("{\"command\": \"knock\"}".as_bytes());
            let mut result_bytes = vec![0; 128];
            let _ = stream.read(&mut result_bytes); // ignore here too
            let bytes = tools::trim_bytes(result_bytes);
            if let Ok(result) = str::from_utf8(&bytes) {
                if let Ok(msg) = serde_json::from_str(&result) as serde_json::Result<peer::Msg>{
                    if msg.command == "knock-back" {
                        println!("CONNECTED");
                        let handle = thread::spawn(|| {
                            peer::handle_client(&Default::default());
                        });
                        connected.push(handle);
                    }
                }
            }
        }
        cur_addr = increment_ip(cur_addr);
    }
    println!("{} peers connected", connected.len());
}
