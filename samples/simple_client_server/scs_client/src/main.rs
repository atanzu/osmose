use std::io::{Read, Write};
use std::str;
use std::str::from_utf8;
use std::process::exit;
use std::net::{SocketAddr, TcpStream};
use socket2::{Socket, Domain, Type};

fn main() {
    let args: std::vec::Vec<String> = std::env::args().collect();

    let socket = Socket::new(Domain::IPV4, Type::STREAM, None).unwrap();
    let address_dest: SocketAddr = args[1].parse().unwrap();
    let address_local: SocketAddr = args[2].parse().unwrap();

    socket.bind(&address_local.into()).unwrap();
    println!("CLIENT: Opened local socket on address {}", args[2]);
    socket.set_reuse_address(true).unwrap();
    socket.connect(&address_dest.into()).unwrap();
    println!("CLIENT: Successfully connected to server {}", &args[1]);

    let mut stream: TcpStream = socket.into();

    let msg = String::from("Hello!");

    println!("CLIENT: Send request");
    stream.write(msg.as_bytes()).unwrap();

    let mut data = [0 as u8; 50];
    match stream.read(&mut data) {
        Ok(length) => {
            let received_msg = &data[0..length];
            if str::from_utf8(&received_msg).unwrap() == msg {
                println!("CLIENT: Reply is ok!");
            } else {
                let text = from_utf8(&data).unwrap();
                println!("CLIENT: Unexpected reply: {}", text);
                exit(1);
            }
        },
        Err(e) => {
            println!("CLIENT: Failed to receive data: {}", e);
            exit(2);
        }
    }
    println!("CLIENT: Terminated.");
    exit(0);
}
