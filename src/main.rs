extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate crypto_hash;

mod peer;
mod runner;
mod tools;
mod listener;
mod scout;

use std::thread;
use std::net;

fn main() {
    let server_handle = thread::spawn(|| {
        listener::server();
    });
    let client_handle = thread::spawn(|| {
        let start = net::Ipv4Addr::new(127,0,0,1);
        let end = net::Ipv4Addr::new(127,0,0,254);
        scout::brute(start, end);
    });
    let _ = server_handle.join().unwrap();
    let _ = client_handle.join().unwrap();
}
