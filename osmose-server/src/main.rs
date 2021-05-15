mod rules_database;

use std::thread;
use std::sync::Arc;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::Read;

use crate::rules_database::RulesDatabase;

use protobuf::Message;

use log;
use env_logger::Env;
use clap::{App, Arg};

use osmose_generated::generated_proto::osmose::DecisionRequest as Request;
use osmose_generated::generated_proto::osmose::DecisionResponse as Response;
use osmose_generated::generated_proto::osmose::Decision as Decision;

fn main() {
    let args = App::new("Osmose server executable")
        .version("0.1")
        .author("Mark K. <atanzuuu@gmail.com>")
        .about("Manages the rules dataset and processes requests")
        .arg(Arg::new("port")
            .short('p')
            .long("port")
            .default_value("9061")
            .takes_value(true))
        .arg(Arg::new("rules")
            .short('r')
            .long("rules")
            .value_name("rules")
            .about("Sets a rules config file")
            .takes_value(true)
            .required(true))
        .get_matches();

    env_logger::Builder::from_env(
        Env::default().default_filter_or("warn")).init();

    let rules_path = std::path::Path::new(
        args.value_of("rules").expect("No rules file path given")
    );
    let rules_database = Arc::new(RulesDatabase::new(&rules_path));

    let port = args.value_of("port").unwrap().parse::<u16>().unwrap();
    let local_address = format!("0.0.0.0:{}", port);
    let listener = TcpListener::bind(&local_address)
        .expect("Cannot open server socket");

    // accept connections and process them, spawning a new thread for each one
    log::info!("Server listening on {}", local_address);
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                log::debug!("New OSMOSE connection: {}", stream.peer_addr().unwrap());
                //TODO Use scope from rayon to get rid of this cloning
                let db_arc = rules_database.clone();
                thread::spawn(move || {
                    handle_client(stream, db_arc);
                });
            }
            Err(error) => {
                log::warn!("Stream error: {}", error);
            }
        }
    }
    log::info!("Server terminating");
    drop(listener);
}


fn handle_client(mut stream: TcpStream, db: Arc<RulesDatabase>) {
    let mut data = [0 as u8; 4096];
    let mut response = Response::new();
    match stream.read(&mut data) {
        Ok(len) => {
            match Request::parse_from_bytes(&data[0..len]) {
                Ok(request) => {
                    log::debug!("Processing request {:?}", request);
                    let decision = db.is_call_allowed(
                        &osmose_identifier::Identifier::from(request.get_source()),
                        &osmose_identifier::Identifier::from(request.get_destination())
                    );

                    log::debug!("Verdict for request is {:?}", decision);

                    response.set_decision(decision);
                    response.write_to_writer(&mut stream).unwrap();
                },
                Err(parse_error) => {
                    log::error!(
                        "Parse error: {}. Terminating connection with {}",
                        parse_error, stream.peer_addr().unwrap());
                    response.set_decision(Decision::MALFORMED_MESSAGE);
                    response.write_to_writer(&mut stream).unwrap();
                }
            }
        },
        Err(stream_error) => {
            log::error!(
                "Stream error occurred: {}, terminating connection with {}",
                stream_error, stream.peer_addr().unwrap());
        }
    }
    stream.shutdown(Shutdown::Both).unwrap();
}

