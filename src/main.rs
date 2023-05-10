use std::fs;

use clap::Parser;
use serde::Deserialize;
use serde_json::json;

#[derive(Parser)]
struct Cli {
    domain: String,
    api_key: String,
    secret_key: String,
}

#[derive(Deserialize)]
struct SslCertificateBundle {
    #[serde(rename = "certificatechain")]
    certificate_chain: String,
    #[serde(rename = "privatekey")]
    private_key: String,
}

fn get_ssl_certificate_bundle(
    domain: &str,
    api_key: &str,
    secret_key: &str,
) -> reqwest::Result<SslCertificateBundle> {
    reqwest::blocking::Client::new()
        .post(format!(
            "https://porkbun.com/api/json/v3/ssl/retrieve/{domain}"
        ))
        .json(&json!({"secretapikey": secret_key, "apikey": api_key}))
        .send()?
        .error_for_status()?
        .json()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let ssl_cerificate_bundle =
        get_ssl_certificate_bundle(&cli.domain, &cli.api_key, &cli.secret_key)?;

    fs::write(
        "ssl_certificate.crt",
        &ssl_cerificate_bundle.certificate_chain,
    )?;
    fs::write("ssl_certificate.key", &ssl_cerificate_bundle.private_key)?;

    Ok(())
}
