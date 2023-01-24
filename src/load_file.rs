use rustls::{server::AllowAnyAuthenticatedClient, RootCertStore};
use std::io::BufReader;
use std::{
    fs::File,
    net::{SocketAddr, ToSocketAddrs},
    sync::Arc,
};

#[derive(Debug, Clone, Default)]
pub struct LoadFile {
    pub ca: String,
    pub client_cert: String,
    pub client_key: String,
    pub server_cert: String,
    pub server_key: String,
}

impl LoadFile {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_str(
        ca: &str,
        client_cert: &str,
        client_key: &str,
        server_cert: &str,
        server_key: &str,
    ) -> Self {
        Self {
            ca: ca.to_string(),
            client_cert: client_cert.to_string(),
            client_key: client_key.to_string(),
            server_cert: server_cert.to_string(),
            server_key: server_key.to_string(),
        }
    }

    pub fn to_str(&self) -> (&str, &str, &str, &str, &str) {
        (
            &self.ca,
            &self.client_cert,
            &self.client_key,
            &self.server_cert,
            &self.server_key,
        )
    }

    pub fn to_vec(&self) -> (&[u8], &[u8], &[u8], &[u8], &[u8]) {
        let ca = self.ca.as_bytes();
        let client_cert = self.client_cert.as_bytes();
        let client_key = self.client_key.as_bytes();
        let server_cert = self.server_cert.as_bytes();
        let server_key = self.server_key.as_bytes();
        (ca, client_cert, client_key, server_cert, server_key)
    }

    /**
     * to SocketAddr from host and port
     */
    pub fn lookup_ipv4(host: &str, port: u16) -> SocketAddr {
        let addrs = (host, port).to_socket_addrs().unwrap();
        for addr in addrs {
            if let SocketAddr::V4(_) = addr {
                return addr;
            }
        }
        unreachable!("Cannot lookup address");
    }

    fn load_certs(&self, filename: &str) -> Vec<rustls::Certificate> {
        let certfile = File::open(filename).expect("cannot open certificate file");
        let mut reader = BufReader::new(certfile);
        rustls_pemfile::certs(&mut reader)
            .unwrap()
            .iter()
            .map(|v| rustls::Certificate(v.clone()))
            .collect()
    }

    fn load_private_key(&self, filename: &str) -> rustls::PrivateKey {
        let keyfile = File::open(filename).expect("cannot open private key file");
        let mut reader = BufReader::new(keyfile);

        loop {
            match rustls_pemfile::read_one(&mut reader).expect("cannot parse private key .pem file")
            {
                Some(rustls_pemfile::Item::RSAKey(key)) => return rustls::PrivateKey(key),
                Some(rustls_pemfile::Item::PKCS8Key(key)) => return rustls::PrivateKey(key),
                None => break,
                _ => {}
            }
        }

        panic!(
            "no keys found in {filename:?} (encrypted keys not supported)"
        );
    }

    /**
     * Convert to rustls::Certificate
     */
    pub fn to_ca(&self) -> Vec<rustls::Certificate> {
        self.load_certs(self.ca.as_str())
    }

    pub fn to_ca_vec(&self) -> Vec<u8> {
        
        self.ca.clone().into_bytes()
    }

    pub fn to_client_cert(&self) -> Vec<rustls::Certificate> {
        self.load_certs(self.client_cert.as_str())
    }

    pub fn to_client_private_key(&self) -> rustls::PrivateKey {
        self.load_private_key(self.client_key.as_str())
    }

    pub fn to_server_cert(&self) -> Vec<rustls::Certificate> {
        self.load_certs(self.server_cert.as_str())
    }

    pub fn to_server_private_key(&self) -> rustls::PrivateKey {
        self.load_private_key(self.server_key.as_str())
    }

    pub fn configure_server(
        &self,
        server_session_memory_cache: Option<usize>,
    ) -> Arc<rustls::ServerConfig> {
        let roots = self.to_server_cert();
        let certs = roots.clone();
        let mut client_auth_roots = RootCertStore::empty();
        for root in roots {
            client_auth_roots.add(&root).unwrap();
        }

        let client_auth = AllowAnyAuthenticatedClient::new(client_auth_roots);
        let privkey = self.to_server_private_key();

        let suites = rustls::ALL_CIPHER_SUITES.to_vec();
        let versions = rustls::ALL_VERSIONS.to_vec();

        let mut config = rustls::ServerConfig::builder()
            .with_cipher_suites(&suites)
            .with_safe_default_kx_groups()
            .with_protocol_versions(&versions)
            .expect("inconsistent cipher-suites/versions specified")
            .with_client_cert_verifier(client_auth)
            .with_single_cert_with_ocsp_and_sct(certs, privkey, vec![], vec![])
            .expect("bad certificates/private key");

        config.key_log = Arc::new(rustls::KeyLogFile::new());
        if server_session_memory_cache.is_none() {
            config.session_storage = rustls::server::ServerSessionMemoryCache::new(256);
        } else {
            config.session_storage =
                rustls::server::ServerSessionMemoryCache::new(server_session_memory_cache.unwrap());
        }
        Arc::new(config)
    }

    pub fn configure_client(&self) -> Arc<rustls::ClientConfig> {
        let cert_file = File::open(self.ca.as_str()).expect("Cannot open CA file");

        let mut reader = BufReader::new(cert_file);

        let mut root_store = RootCertStore::empty();
        root_store.add_parsable_certificates(&rustls_pemfile::certs(&mut reader).unwrap());

        let suites = rustls::DEFAULT_CIPHER_SUITES.to_vec();
        let versions = rustls::DEFAULT_VERSIONS.to_vec();

        let certs = self.to_client_cert();
        let key = self.to_client_private_key();

        let config = rustls::ClientConfig::builder()
            .with_cipher_suites(&suites)
            .with_safe_default_kx_groups()
            .with_protocol_versions(&versions)
            .expect("inconsistent cipher-suite/versions selected")
            .with_root_certificates(root_store)
            .with_single_cert(certs, key)
            .expect("invalid client auth certs/key");
        Arc::new(config)
    }
}
