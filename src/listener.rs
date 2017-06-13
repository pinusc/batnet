use std::thread;
use std::net::TcpListener;
use tools;
use peer;

pub fn server() {
    // let address = env::args().nth(1).unwrap();
    // let port =  env::args().nth(2).unwrap();
    let listener = TcpListener::bind((tools::get_own_ip().unwrap(), 6667)).unwrap();
    let mut client_handles = vec![];
    // accept connections and process them serially
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let handle = thread::spawn(|| {
                    let mut peer = Default::default();
                    peer::handle_client(&peer);
                });
                client_handles.push(handle);
            }
            Err(e) => {
                println!("{}", e);
            }
        }
    }
}
