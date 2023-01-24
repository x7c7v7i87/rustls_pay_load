use std::io;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::net::TcpStream;
use tokio_rustls::{
    client::TlsStream as ClientTlsStream,
    rustls::{self},
    TlsAcceptor, TlsConnector,
};

pub mod fun;

use rustls_pay_load::load_file::LoadFile as Load;
use fun::{CA_FILE, CLIENT_CERT_FILE, CLIENT_KEY_FILE, SERVER_CERT_FILE, SERVER_KEY_FILE};

pub async fn client(domain: &str) -> ClientTlsStream<TcpStream> {
    let mut load = Load::new();
    load.ca = CA_FILE.to_string();
    load.client_cert = CLIENT_CERT_FILE.to_string();
    load.client_key = CLIENT_KEY_FILE.to_string();

    let addr = Load::lookup_ipv4("127.0.0.1", 3030);

    let config = load.configure_client();
    let connector = TlsConnector::from(config);

    let stream = TcpStream::connect(&addr).await.unwrap();
    let domain = rustls::ServerName::try_from(domain)
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "invalid dnsname"))
        .unwrap();
    
    connector.connect(domain, stream).await.unwrap()
}

pub fn server() -> TlsAcceptor {
    let mut load = Load::new();
    load.server_cert = SERVER_CERT_FILE.to_string();
    load.server_key = SERVER_KEY_FILE.to_string();

    let config = load.configure_server(None);
    
    TlsAcceptor::from(config)
}

async fn start_server() {
    let tls_acceptor = server();
    let listener = TcpListener::bind("0.0.0.0:3030").await.unwrap();

    tokio::spawn(async move {
        let (stream, _peer_addr) = listener.accept().await.unwrap();
        let mut tls_stream = tls_acceptor.accept(stream).await.unwrap();
        println!("server: Accepted client conn with TLS");

        let mut buf = [0; 12];
        tls_stream.read(&mut buf).await.unwrap();
        println!("server: got data: {buf:?}");
        tls_stream.write(&buf).await.unwrap();
        println!("server: flush the data out");
    });
}

async fn start_client(msg: &[u8], buf: &mut [u8]) {
    let mut tls_stream = client("localhost").await;

    tls_stream.write(msg).await.unwrap();
    println!("client: send data");

    tls_stream.read(buf).await.unwrap();
    println!("client: read echoed data");
}


async fn tls() {
    let msg = b"Hello world\n";
    let mut buf = [0; 12];

    start_server().await;

    start_client(msg, &mut buf).await;
    assert_eq!(&buf, msg);
}

#[tokio::main]
async fn main() {
    tls().await;
}
