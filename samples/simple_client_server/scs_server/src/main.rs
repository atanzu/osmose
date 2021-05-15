use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
use std::process::exit;

use osmose_identifier::Identifier;


fn main() {
    let args: std::vec::Vec<String> = std::env::args().collect();
    let listener = TcpListener::bind(&args[1]).unwrap();
    println!("SERVER: Listening on address {}", &args[1]);
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let self_addr = args[1].clone();
                println!("SERVER: new connection: {}", stream.peer_addr().unwrap());
                thread::spawn(move|| {
                    handle_client(stream, &self_addr)
                });
            }
            Err(error) => {
                println!("SERVER: error: {}", error);
            }
        }
    }
    drop(listener);
    exit(0);
}


fn handle_client(mut stream: TcpStream, self_addr: &str) {
    println!("SERVER: start handle_client");
    let mut data = [0 as u8; 50];
    match stream.read(&mut data) {
        Ok(size) => {
            // Use source address as a name
            let source_identifier = Identifier::from_given(
                &stream.peer_addr().unwrap().to_string(), 111);
            println!("SERVER: received request from source {:?}", source_identifier);
            let payload = &data[0..size];

            let mut osmose_client = osmose_client::OsmoseClient::new();
            osmose_client.set_self_id(Identifier::from_given(self_addr, 5678));

            println!("SERVER: asking OSMOSE");
            let res = osmose_client.ask_for_verdict(
                &source_identifier, &payload);
            println!("SERVER: OSMOSE decided: {}", res);
            if res {
                stream.write(&data[0..size]).unwrap();
            }

        },
        Err(error) => {
            println!(
                "SERVER: an error occurred: {}, terminating connection with {}",
                error, stream.peer_addr().unwrap());
            stream.shutdown(Shutdown::Both).unwrap();
        }
    };
    println!("SERVER: finish handle_client");
}
