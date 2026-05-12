use std::io::BufReader;
use tonic::transport::{Certificate, ClientTlsConfig};

pub(crate) fn build_tonic_tls_config(
    ca_cert_path: Option<&str>,
) -> anyhow::Result<ClientTlsConfig> {
    let mut config = ClientTlsConfig::new().with_native_roots();
    if let Some(path) = ca_cert_path {
        let pem = std::fs::read(path)
            .map_err(|_| anyhow::anyhow!("failed to read CA cert file: {}", path))?;
        config = config
            .ca_certificate(Certificate::from_pem(pem))
            .assume_http2(true);
    }
    Ok(config)
}

pub(crate) fn build_https_connector(
    ca_cert_path: Option<&str>,
) -> anyhow::Result<hyper_rustls::HttpsConnector<hyper_util::client::legacy::connect::HttpConnector>>
{
    if let Some(path) = ca_cert_path {
        let mut roots = rustls::RootCertStore::empty();
        let rustls_native_certs::CertificateResult { certs, errors, .. } =
            rustls_native_certs::load_native_certs();
        if certs.is_empty() {
            anyhow::bail!("could not load platform certs: {errors:?}");
        }
        for cert in certs {
            roots.add(cert).ok();
        }
        let pem = std::fs::read(path)?;
        for cert in rustls_pemfile::certs(&mut BufReader::new(&pem[..])) {
            roots.add(cert?)?;
        }
        let config = rustls::ClientConfig::builder()
            .with_root_certificates(roots)
            .with_no_client_auth();
        Ok(hyper_rustls::HttpsConnectorBuilder::new()
            .with_tls_config(config)
            .https_only()
            .enable_http1()
            .build())
    } else {
        Ok(hyper_rustls::HttpsConnectorBuilder::new()
            .with_native_roots()?
            .https_only()
            .enable_http1()
            .build())
    }
}
