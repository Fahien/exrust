use std::{io::Write, net::TcpStream};

use openssl::ssl::{Ssl, SslContext, SslMethod, SslStream};

/// Using OpenSSL and TCP from the standard library tools
fn tcp() -> std::io::Result<()> {
    let ctx_builder =
        SslContext::builder(SslMethod::tls()).expect("Failed to create builder");
    let ctx = ctx_builder.build();

    let mut ssl = Ssl::new(&ctx).expect("Failed to create Ssl");
    ssl.set_connect_state();

    let connection = TcpStream::connect("www.antoniocaggiano.eu:443")?;
    let mut ssl_stream = SslStream::new(ssl, connection).expect("failed to create SslStream");
    ssl_stream.do_handshake().expect("Failed to do handshake");

    ssl_stream.write_all(b"GET / HTTP/1.0")?;
    ssl_stream.write_all(b"\r\n")?;
    ssl_stream.write_all(b"Host: www.antoniocaggiano.eu")?;
    ssl_stream.write_all(b"\r\n\r\n")?;

    std::io::copy(&mut ssl_stream, &mut std::io::stdout())?;

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tcp()?;
    Ok(())
}
