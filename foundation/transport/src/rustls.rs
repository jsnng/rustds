use crate::prelude::*;
use crate::tls::TlsHandshaker;
use rustls::pki_types::ServerName;
use rustls::{ClientConfig, ClientConnection, RootCertStore};
use rustls_pki_types::DnsName;
use std::convert::TryFrom;
use std::io::{Read, Write};
use std::net::IpAddr;
use std::string::String;
use std::sync::Arc;
use webpki_roots;

#[derive(Debug)]
pub struct RustlsConfig {
    config: Arc<ClientConfig>,
}

impl Default for RustlsConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl RustlsConfig {
    pub fn new() -> Self {
        let root_store = RootCertStore::from_iter(webpki_roots::TLS_SERVER_ROOTS.iter().cloned());
        let config = Arc::new(
            ClientConfig::builder_with_protocol_versions(&[
                #[cfg(feature = "tls1.3")]
                &rustls::version::TLS13,
                #[cfg(feature = "tls1.2")]
                &rustls::version::TLS12,
            ])
            .with_root_certificates(root_store)
            .with_no_client_auth(),
        );
        Self { config }
    }

    /// Trust the server certificate without verification.
    pub fn new_trust_server_certificate() -> Self {
        let config = Arc::new(
            ClientConfig::builder_with_protocol_versions(&[
                #[cfg(feature = "tls1.3")]
                &rustls::version::TLS13,
                #[cfg(feature = "tls1.2")]
                &rustls::version::TLS12,
            ])
            .dangerous()
            .with_custom_certificate_verifier(Arc::new(NoCertificateVerification))
            .with_no_client_auth(),
        );
        Self { config }
    }
}

impl TlsHandshaker for RustlsConfig {
    type Connection = ClientConnection;
    type HandshakeError = rustls::Error;

    fn handshake<S: Read + Write>(&self, server_name: &str, stream: &mut S) -> Result<ClientConnection, rustls::Error> {
        let name = if let Ok(dns_name) = DnsName::try_from(String::from(server_name)) {
            ServerName::DnsName(dns_name)
        } else if let Ok(ip_addr) = server_name.parse::<IpAddr>() {
            ServerName::IpAddress(ip_addr.into())
        } else {
            return Err(rustls::Error::UnsupportedNameType);
        };

        let mut connection = ClientConnection::new(Arc::clone(&self.config), name)?;

        connection
            .complete_io(stream)
            .map_err(|e| rustls::Error::General(format!("TLS handshake IO error: {}", e)))?;

        if connection.is_handshaking() {
            return Err(rustls::Error::General("TLS handshake incomplete".into()));
        }

        Ok(connection)
    }
}

// ---------------------------------------------------------------------------
// RustlsProvider — wraps a transport + an established TLS connection
// ---------------------------------------------------------------------------

pub struct RustlsProvider<T: Transport> {
    transport: T,
    connection: ClientConnection,
}

impl<T: Transport> RustlsProvider<T> {
    pub fn from_parts(transport: T, connection: ClientConnection) -> Self {
        Self { transport, connection }
    }
}

struct StreamAdapter<'a, T: Transport>(&'a mut T);

impl<T: Transport> Read for StreamAdapter<'_, T>
where
    T::Error: core::fmt::Debug,
{
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.0
            .read(buf)
            .map_err(|e| std::io::Error::other(format!("{:?}", e)))
    }
}

impl<T: Transport> Write for StreamAdapter<'_, T>
where
    T::Error: core::fmt::Debug,
{
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.0
            .write(buf)
            .map_err(|e| std::io::Error::other(format!("{:?}", e)))
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

impl<T: Transport> Transport for RustlsProvider<T>
where
    T::Error: core::fmt::Debug,
{
    type Error = std::io::Error;

    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        loop {
            match self.connection.reader().read(buf) {
                Ok(0) => return Ok(0),
                Ok(n) => return Ok(n),
                Err(e) if e.kind() != std::io::ErrorKind::WouldBlock => return Err(e),
                _ => {}
            }

            let mut adaptor = StreamAdapter(&mut self.transport);
            self.connection.read_tls(&mut adaptor)?;
            self.connection.process_new_packets().map_err(|e| {
                std::io::Error::new(std::io::ErrorKind::InvalidData, format!("{}", e))
            })?;
        }
    }

    fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        let written = self.connection.writer().write(buf)?;
        let mut adaptor = StreamAdapter(&mut self.transport);
        self.connection.write_tls(&mut adaptor)?;
        Ok(written)
    }

    fn set_read_timeout(&mut self, timeout: Option<std::time::Duration>) -> Result<(), Self::Error> {
        self.transport
            .set_read_timeout(timeout)
            .map_err(|e| std::io::Error::other(format!("{:?}", e)))
    }

    fn set_write_timeout(&mut self, timeout: Option<std::time::Duration>) -> Result<(), Self::Error> {
        self.transport
            .set_write_timeout(timeout)
            .map_err(|e| std::io::Error::other(format!("{:?}", e)))
    }
}

// ---------------------------------------------------------------------------
// NoCertificateVerification
// ---------------------------------------------------------------------------

#[derive(Debug)]
pub struct NoCertificateVerification;
use rustls::client::danger::{ServerCertVerified, ServerCertVerifier};

impl ServerCertVerifier for NoCertificateVerification {
    fn verify_server_cert(
        &self,
        _end_entity: &rustls_pki_types::CertificateDer<'_>,
        _intermediates: &[rustls_pki_types::CertificateDer<'_>],
        _server_name: &ServerName<'_>,
        _ocsp_response: &[u8],
        _now: rustls_pki_types::UnixTime,
    ) -> Result<rustls::client::danger::ServerCertVerified, rustls::Error> {
        Ok(ServerCertVerified::assertion())
    }

    fn verify_tls12_signature(
        &self,
        message: &[u8],
        cert: &rustls_pki_types::CertificateDer<'_>,
        dss: &rustls::DigitallySignedStruct,
    ) -> Result<rustls::client::danger::HandshakeSignatureValid, rustls::Error> {
        rustls::crypto::verify_tls12_signature(
            message,
            cert,
            dss,
            &rustls::crypto::aws_lc_rs::default_provider().signature_verification_algorithms,
        )
    }

    fn verify_tls13_signature(
        &self,
        message: &[u8],
        cert: &rustls_pki_types::CertificateDer<'_>,
        dss: &rustls::DigitallySignedStruct,
    ) -> Result<rustls::client::danger::HandshakeSignatureValid, rustls::Error> {
        rustls::crypto::verify_tls13_signature(
            message,
            cert,
            dss,
            &rustls::crypto::aws_lc_rs::default_provider().signature_verification_algorithms,
        )
    }

    fn supported_verify_schemes(&self) -> Vec<rustls::SignatureScheme> {
        rustls::crypto::aws_lc_rs::default_provider()
            .signature_verification_algorithms
            .supported_schemes()
    }
}
