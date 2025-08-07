# redacta

[![crates.io](https://img.shields.io/crates/v/redacta)](https://crates.io/crates/redacta)
[![GitHub Release](https://img.shields.io/github/v/release/lhalf/redacta)](https://github.com/lhalf/redacta/releases)
[![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/lhalf/redacta/on_commit.yml)](https://github.com/lhalf/redacta/actions/workflows/on_commit.yml)
[![MIT](https://img.shields.io/badge/license-MIT-blue)](./LICENSE)

Command line tool for redacting information from text.

> [!WARNING]
> This is an early stage implementation, it might not redact accurately.

## Install

```bash
curl -sS https://raw.githubusercontent.com/lhalf/redacta/main/install.sh | sh
```

Or install via cargo:
```bash
cargo install redacta
```

## Usage

Takes text via stdin and forwards redacted text to stdout.

```
$ redacta --help
Usage: <STDIN> | redacta [OPTIONS]

Options:
      --ipv4           Enable IPv4 redaction
      --ipv6           Enable IPv6 redaction
      --fqdn           Enable FQDN redaction
  -r, --regex <REGEX>  Regex redaction
  -h, --help           Print help
```

## Example

```bash
$ echo "Look at my 192.168.0.1 IP!" | redacta --ipv4
Look at my *********** IP!
```

```bash
$ echo "No really, look at my 2001:db8:3333:4444:5555:6666:7777:8888 IP!" | redacta --ipv6
No really, look at my ************************************** IP!
```

```bash
$ echo "Okay, but you won't example.server.com like it..." | redacta --fqdn
Okay, but you won't ****************** like it...
```
