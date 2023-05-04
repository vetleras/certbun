#![feature(result_option_inspect)]
use std::{env, fs, io};

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

fn write_to_file(path: &str, contents: &str) -> io::Result<()> {
    fs::write(path, contents).inspect_err(|e| {
        println!(
            "writing to file {}{path} failed: {e}",
            env::current_dir().unwrap().to_string_lossy()
        )
    })
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let ssl_cerificate_bundle =
        get_ssl_certificate_bundle(&cli.domain, &cli.api_key, &cli.secret_key)
            .inspect_err(|e| println!("retrival of ssl certificate bundle failed: {e}"))?;

    write_to_file(
        "ssl_certificate.crt",
        &ssl_cerificate_bundle.certificate_chain,
    )?;
    write_to_file("ssl_certificate.key", &ssl_cerificate_bundle.private_key)?;

    Ok(())
}
