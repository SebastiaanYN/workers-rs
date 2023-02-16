#[derive(Debug)]
pub struct CloudflareProperties {
    pub colo: String,
    pub asn: u32,
    pub country: Option<String>,
    pub http_protocol: String,
    pub request_priority: Option<String>,
    pub tls_client_auth: Option<TlsClientAuth>,
    pub tls_cipher: String,
    pub tls_version: String,
    pub city: Option<String>,
    pub continent: Option<String>,
    pub latitude: Option<String>,
    pub longitude: Option<String>,
    pub postal_code: Option<String>,
    pub metro_code: Option<String>,
    pub region: Option<String>,
    pub region_code: Option<String>,
    pub timezone: String,
    pub is_eu_country: Option<String>,
}

impl From<worker_sys::cf::Cf> for CloudflareProperties {
    fn from(cf: worker_sys::cf::Cf) -> Self {
        Self {
            colo: cf.colo(),
            asn: cf.asn(),
            country: cf.country(),
            http_protocol: cf.http_protocol(),
            request_priority: cf.request_priority(),
            tls_client_auth: cf.tls_client_auth().map(Into::into),
            tls_cipher: cf.tls_cipher(),
            tls_version: cf.tls_version(),
            city: cf.city(),
            continent: cf.continent(),
            latitude: cf.latitude(),
            longitude: cf.longitude(),
            postal_code: cf.postal_code(),
            metro_code: cf.metro_code(),
            region: cf.region(),
            region_code: cf.region_code(),
            timezone: cf.timezone(),
            is_eu_country: cf.is_eu_country(),
        }
    }
}

#[derive(Debug)]
pub struct TlsClientAuth {
    pub cert_issuer_dn_legacy: String,
    pub cert_issuer_dn: String,
    pub cert_issuer_dn_rfc2253: String,
    pub cert_subject_dn_legacy: String,
    pub cert_verified: String,
    pub cert_not_after: String,
    pub cert_subject_dn: String,
    pub cert_fingerprint_sha1: String,
    pub cert_not_before: String,
    pub cert_serial: String,
    pub cert_presented: String,
    pub cert_subject_dn_rfc2253: String,
}

impl From<worker_sys::cf::TlsClientAuth> for TlsClientAuth {
    fn from(tls: worker_sys::cf::TlsClientAuth) -> Self {
        Self {
            cert_issuer_dn_legacy: tls.cert_issuer_dn_legacy(),
            cert_issuer_dn: tls.cert_issuer_dn(),
            cert_issuer_dn_rfc2253: tls.cert_issuer_dn_rfc2253(),
            cert_subject_dn_legacy: tls.cert_subject_dn_legacy(),
            cert_verified: tls.cert_verified(),
            cert_not_after: tls.cert_not_after(),
            cert_subject_dn: tls.cert_subject_dn(),
            cert_fingerprint_sha1: tls.cert_fingerprint_sha1(),
            cert_not_before: tls.cert_not_before(),
            cert_serial: tls.cert_serial(),
            cert_presented: tls.cert_presented(),
            cert_subject_dn_rfc2253: tls.cert_subject_dn_rfc2253(),
        }
    }
}
