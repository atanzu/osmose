use std::net::{TcpStream};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use osmose_generated::generated_proto::osmose::Identifier as InternalIdentifier;
use osmose_generated::generated_proto::osmose::DecisionRequest as Request;
use osmose_generated::generated_proto::osmose::DecisionResponse as Response;
use osmose_generated::generated_proto::osmose::Decision as Decision;
use osmose_identifier::Identifier;
use protobuf::Message;


/// A client for making verdict requests to Osmose server
pub struct OsmoseClient {
    server_address: SocketAddr,
    self_id: Identifier
}


impl OsmoseClient {
    /// Returns a default-initializes OsmoseClient instance
    ///
    /// # Examples
    ///
    /// ```
    /// use osmose_client::OsmoseClient;
    /// let client = OsmoseClient::new();
    /// ```
    pub fn new() -> Self {
        OsmoseClient{
            server_address: SocketAddr::new(IpAddr::V4(
                                    Ipv4Addr::new(127, 0, 0, 1)), 9061),
            self_id: Identifier::new()
        }
    }

    /// Returns an OsmoseClient instance with custom server address and port
    ///
    /// # Arguments
    ///
    /// * `ip` - An IpAddr object with IP address of Osmose server
    /// * `port` - Port for communication with Osmose server
    ///
    /// # Examples
    ///
    /// ```
    /// use std::net::{IpAddr, Ipv4Addr};
    /// use osmose_client::OsmoseClient;
    ///
    /// let server_addr = IpAddr::V4(Ipv4Addr::new(192, 168, 0, 1));
    /// let server_port: u16 = 12345;
    /// let client = OsmoseClient::from_address(server_addr, server_port);
    /// ```
    pub fn from_address(ip: IpAddr, port: u16) -> Self {
        OsmoseClient{ 
            server_address: SocketAddr::new(ip, port),
            self_id: Identifier::new()
        }
    }

    /// Returns an OsmoseClient instance with custom server address and port
    /// given as a SocketAddr object
    ///
    /// # Arguments
    ///
    /// * `address` - A SocketAddr object with IP address and port of Osmose
    /// server
    ///
    /// # Examples
    ///
    /// ```
    /// use std::net::{IpAddr, Ipv4Addr, SocketAddr};
    /// use osmose_client::OsmoseClient;
    ///
    /// let server_addr = IpAddr::V4(Ipv4Addr::new(192, 168, 0, 1));
    /// let server_port: u16 = 12345;
    /// let server_sockaddr = SocketAddr::new(server_addr, server_port);
    /// let client = OsmoseClient::from_socket_address(server_sockaddr);
    /// ```
    pub fn from_socket_address(address: SocketAddr) -> Self {
        OsmoseClient{ 
            server_address: address,
            self_id: Identifier::new()
        }
    }

    /// Sets self ID for this client instance
    ///
    /// # Arguments
    ///
    /// * `id` - Identifier which represents the calling entity
    ///
    /// # Examples
    ///
    /// ```
    /// use osmose_client::OsmoseClient;
    /// use osmose_identifier::Identifier;
    ///
    /// let mut client = OsmoseClient::new();
    /// let custom_id = Identifier::new();
    /// client.set_self_id(custom_id);
    /// ```
    pub fn set_self_id(&mut self, id: Identifier) {
        self.self_id = id;
    }

    /// Returns self ID for this client instance
    ///
    /// # Examples
    ///
    /// ```
    /// use osmose_client::OsmoseClient;
    /// use osmose_identifier::Identifier;
    ///
    /// let client = OsmoseClient::new();
    /// let identifier = client.get_self_id();
    /// assert_eq!(identifier.get_id(), std::process::id() as u64);
    /// ```
    pub fn get_self_id(&self) -> &Identifier {
        &self.self_id
    }

    /// Sends a request to Osmose server to get the verdict
    /// Returns `true` if Osmose server allows to pass the given message from
    /// the calling entity to the current entity, false otherwise
    ///
    /// # Arguments
    ///
    /// * `source` - Identifier which represents the calling entity
    /// * `payload` - Raw message data from the calling entity
    ///
    /// # Examples
    /// ```
    /// use osmose_identifier::Identifier;
    /// use osmose_client::OsmoseClient;
    /// use osmose_generated::generated_proto::osmose::Decision as Decision;
    ///
    /// let client = OsmoseClient::new();
    ///
    /// let identifier = Identifier::new();
    /// let msg = b"test";
    ///
    /// let verdict = client.ask_for_verdict(&identifier, msg);
    /// assert_eq!(verdict, false);
    /// ```
    ///
    // TODO: change return type
    pub fn ask_for_verdict(&self, source: &Identifier, payload: &[u8]) -> bool {
        let request = prepare_request(source, &self.get_self_id(), payload);

        match TcpStream::connect(&self.server_address) {
            Ok(mut stream) => {
                log::debug!(
                    "Connected to OSMOSE server {}", &self.server_address);

                request.write_to_writer(&mut stream).unwrap();
                log::debug!("Sent request: {:?}", &request);

                let response = Response::parse_from_reader(&mut stream).unwrap();
                log::debug!("Received reply: {:?}", &response);
                match response.get_decision() {
                    Decision::ALLOW => true,
                    _ => false
                }
            },
            Err(e) => {
                log::error!("Failed to connect to Osmose server: {}", e);
                false
            }
        }
    }
}


/// Helper function to fill fields of Protobuf request object
///
/// # Arguments
///
/// * `from` - Identifier of the calling entity
/// * `to` - Identifier of the target entity, in particular the current one
/// which actually asks Osmose server for the verdict
/// * `msg` - Message from the calling entity
fn prepare_request(from: &Identifier, to: &Identifier, msg: &[u8]) -> Request {
    let mut req = Request::new();

    req.set_source(InternalIdentifier::from(from));
    req.set_destination(InternalIdentifier::from(to));

    req.set_payload(std::vec::Vec::from(msg));

    req
}

