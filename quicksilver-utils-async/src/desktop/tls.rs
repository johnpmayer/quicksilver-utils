use rustls::ClientConfig;
use rustls_native_certs::load_native_certs;

pub fn client_config() -> ClientConfig {
    let mut config = ClientConfig::new();
    let native_certs = load_native_certs().expect("Could not load platform certs");
    config.root_store = native_certs;
    config
}
