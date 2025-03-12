use anyhow::{Context, Result};
use clap::Parser;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use serde::Deserialize;
use std::fs;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Email title
    #[arg(short, long)]
    title: String,

    /// Email message
    #[arg(short, long)]
    message: String,

    /// Config file path
    #[arg(short, long, default_value = "config.yml")]
    config: String,
}

#[derive(Debug, Deserialize)]
struct Config {
    sender: String,
    receiver: String,
    smtp_server: String,
    smtp_port: u16,
    keychain_service: String,
    keychain_account: String,
}

fn read_config(path: &str) -> Result<Config> {
    let content = fs::read_to_string(path)
        .with_context(|| format!("Failed to read config file: {}", path))?;
    serde_yaml::from_str(&content)
        .with_context(|| format!("Failed to parse config file: {}", path))
}

fn get_smtp_password(service: &str, account: &str) -> Result<String> {
    let entry = keyring::Entry::new(service, account)
        .with_context(|| "Failed to create keychain entry")?;
    entry.get_password()
        .with_context(|| "Failed to get password from keychain")
}

fn send_email(
    title: &str,
    message: &str,
    config: &Config,
    smtp_password: &str,
) -> Result<()> {
    let email = Message::builder()
        .from(config.sender.parse()?)
        .to(config.receiver.parse()?)
        .subject(title)
        .body(String::from(message))?;

    let creds = Credentials::new(config.sender.clone(), smtp_password.to_string());

    let mailer = SmtpTransport::relay(&config.smtp_server)
        .with_context(|| format!("Failed to create SMTP transport for {}", config.smtp_server))?
        .port(config.smtp_port)
        .credentials(creds)
        .build();

    mailer.send(&email)
        .with_context(|| "Failed to send email")?;

    println!("Email sent successfully!");
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let config = read_config(&args.config)?;
    
    let smtp_password = get_smtp_password(&config.keychain_service, &config.keychain_account)?;
    
    send_email(&args.title, &args.message, &config, &smtp_password)?;
    
    Ok(())
}