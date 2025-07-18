use clap::Parser;
use redacta::ipv4::IPv4Redactor;
use redacta::ipv6::IPv6Redactor;
use redacta::redact::{Redact, redact_logs};
use std::io::BufReader;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// Enable IPv4 redaction
    #[arg(long)]
    ipv4: bool,
    /// Enable IPv6 redaction
    #[arg(long)]
    ipv6: bool,
}

fn main() {
    let stdin = std::io::stdin().lock();
    let mut stdout = std::io::stdout().lock();

    redact_logs(
        BufReader::new(stdin),
        &mut stdout,
        &enabled_redactors(&Args::parse()),
    )
    .unwrap_or_else(|_| println!("failed to redact"));
}

fn enabled_redactors(args: &Args) -> Vec<Box<dyn Redact>> {
    [
        args.ipv4
            .then(|| Box::new(IPv4Redactor::default()) as Box<dyn Redact>),
        args.ipv6
            .then(|| Box::new(IPv6Redactor::default()) as Box<dyn Redact>),
    ]
    .into_iter()
    .flatten()
    .collect()
}
