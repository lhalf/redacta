# redacta

[![crates.io](https://img.shields.io/crates/v/redacta)](https://crates.io/crates/redacta)
[![GitHub Release](https://img.shields.io/github/v/release/lhalf/redacta)](https://github.com/lhalf/redacta/releases)
[![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/lhalf/redacta/on_commit.yml)](https://github.com/lhalf/redacta/actions/workflows/on_commit.yml)
[![MIT](https://img.shields.io/badge/license-MIT-blue)](./LICENSE)

Command line tool for redacting information from text.

> [!WARNING]
> This is an early stage implementation, it might not redact accurately.

## Install

Download from [releases](https://github.com/lhalf/redacta/releases) then:
```bash
tar -xvzf redacta.tar.gz
```

Or install via cargo:
```bash
cargo install redacta
```

## Usage

Takes text via stdin and forwards redacted text to stdout.

```
$ redacta --help
Command line tool for redacting information from text.

Usage: redacta [OPTIONS]

Options:
      --ipv4           Enable IPv4 redaction
      --ipv6           Enable IPv6 redaction
  -r, --regex <REGEX>  Regex redaction
  -h, --help           Print help
  -V, --version        Print versio
```

## Example

```bash
$ echo "Look at my 192.168.0.1 IP!" | redacta --ipv4
Look at my *********** IP!
```