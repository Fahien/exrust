use std::{
    net::{SocketAddr, UdpSocket},
    time::Duration,
};

use clap::{Command, Arg};
use trust_dns_resolver::proto::{
    op::{Message, MessageType, OpCode, Query},
    rr::RecordType,
    serialize::binary::{BinEncodable, BinEncoder},
};

fn main() {
    let matches = Command::new("dns")
        .version("0.2")
        .author("Antonio Caggiano <info@antoniocaggiano.eu>")
        .about("DNS resolver")
        .arg(
            Arg::new("dns-server")
                .short('s')
                .default_value("1.1.1.1"),
        )
        .arg(Arg::new("domain-name").required(true))
        .get_matches();

    let dns_server = matches.value_of("dns-server").unwrap();

    let domain_name = matches.value_of("domain-name").unwrap();
    let domain_name = trust_dns_resolver::Name::from_ascii(&domain_name).unwrap();

    // Define message
    let mut msg = Message::new();
    msg.set_id(rand::random::<u16>())
        .set_message_type(MessageType::Query)
        .add_query(Query::query(domain_name, RecordType::A))
        .set_op_code(OpCode::Query)
        .set_recursion_desired(true);

    // Encode message to a buffer of byte
    let mut buffer = Vec::with_capacity(512);
    let mut encoder = BinEncoder::new(&mut buffer);
    msg.emit(&mut encoder).unwrap();

    // Listening socket
    let localhost = UdpSocket::bind("0.0.0.0:0").expect("Failed to bind to local socket");
    let timeout = Duration::from_secs(3);
    localhost.set_read_timeout(Some(timeout)).unwrap();
    localhost.set_nonblocking(false).unwrap();

    let dns_server: SocketAddr = format!("{}:53", dns_server)
        .parse()
        .expect("Failed to parse dns server address");
    let _ = localhost
        .send_to(&buffer, dns_server)
        .expect("Failed to send request");

    let mut response: [u8; 512] = [0; 512];
    let (_, _) = localhost
        .recv_from(&mut response)
        .expect("Failed to recieve response");

    let dns_response = Message::from_vec(&response).expect("Failed to parse response");

    for answer in dns_response.answers() {
        if answer.record_type() == RecordType::A {
            let resource = answer.data().expect("Failed to get data from answer");
            let ip = resource.to_ip_addr().expect("Failed to get IP address");
            println!("{}", ip.to_string());
        }
    }
}
