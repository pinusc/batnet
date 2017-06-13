use runner;
use std::io::Read;  // Trait necessary for TcpStream::read
use std::net::TcpStream;
use std::process::Output;
use std::str;
use serde_json;

#[derive(Serialize, Deserialize)]
pub struct Msg {
    pub command: String,
    pub argument: Option<String>
}


pub struct Peer {
    pub stream: Option<TcpStream>,
    pub authenticated: bool,
    pub running: Option<String>,
    pub output: Option<Output>
}

impl Default for Peer {
    fn default () -> Peer {
        Peer {
            stream: None,
            authenticated: false,
            running: None,
            output: None
        }
    }
}

pub fn handle_client(mut peer: &Peer) {
    // if let Some(ref stream) = peer.stream {
    //     if let Ok(addr) = stream.peer_addr() {
    //         println!("Connection from: {}", addr);
    //     } else {
    //         return;
    //     }
    // } else {
    //     return;
    // }
    while let Some(ref mut stream) = peer.stream {
        let mut result_bytes: Vec<u8> = vec![0; 128];
        let res = stream.read(&mut result_bytes); // ignore here too
        if let Ok(0) = res {  // If 0 bytes are sent the socket is assumed to be disconnected
            peer.stream = None;
            return;
        } else if let Ok(n) = res {
            let bytes = &result_bytes[0..n];
            if let Ok(result) = str::from_utf8(&bytes) {
                if let Ok(msg) = serde_json::from_str(&result) as serde_json::Result<Msg>{
                    runner::handle_command(&mut peer, msg);
                } else {
                    println!("Invalid JSON");
                };
            }
        } else {
            peer.stream = None;
            return;
        }
    }
}
