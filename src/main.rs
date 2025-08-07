use clap::Parser;
use redacta::fqdn::FqdnRedactor;
use redacta::ipv4::IPv4Redactor;
use redacta::ipv6::IPv6Redactor;
use redacta::redact::{Redact, redact_logs};
use redacta::regex::RegexRedactor;
use std::io::BufReader;

#[derive(Parser)]
#[command(disable_help_subcommand = true, override_usage = "<STDIN> | redacta [OPTIONS]")]
struct Args {
    /// Enable IPv4 redaction
    #[arg(long)]
    ipv4: bool,
    /// Enable IPv6 redaction
    #[arg(long)]
    ipv6: bool,
    /// Enable FQDN redaction
    #[arg(long)]
    fqdn: bool,
    /// Regex redaction
    #[arg(short, long)]
    regex: Option<String>,
}

fn main() -> anyhow::Result<()> {
    let stdin = std::io::stdin().lock();
    let mut stdout = std::io::stdout().lock();

    redact_logs(
        BufReader::new(stdin),
        &mut stdout,
        &enabled_redactors(&Args::parse())?,
    )
    .unwrap_or_else(|_| println!("failed to redact"));

    Ok(())
}

fn enabled_redactors(args: &Args) -> anyhow::Result<Vec<Box<dyn Redact>>> {
    let mut redactors: Vec<Box<dyn Redact>> = Vec::new();

    if args.ipv4 {
        redactors.push(Box::new(IPv4Redactor::default()));
    }

    if args.ipv6 {
        redactors.push(Box::new(IPv6Redactor::default()));
    }

    if args.fqdn {
        redactors.push(Box::new(FqdnRedactor::default()));
    }

    if let Some(regex) = &args.regex {
        redactors.push(Box::new(RegexRedactor::try_from(regex)?));
    }

    Ok(redactors)
}
