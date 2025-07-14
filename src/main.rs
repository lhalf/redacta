use redacta::ipv4::{IPv4Redactor, Redact};
use std::io::{BufRead, BufReader, Write};

fn main() {
    let redactor = IPv4Redactor::default();
    let stdin = std::io::stdin().lock();
    let mut stdout = std::io::stdout().lock();

    redact_logs(BufReader::new(stdin), &mut stdout, &redactor)
        .unwrap_or_else(|_| println!("failed to redact"));
}

fn redact_logs(
    mut input: impl BufRead,
    output: &mut impl Write,
    redactor: &impl Redact,
) -> anyhow::Result<()> {
    let mut buffer = Vec::with_capacity(1024);
    while input.read_until(b'\n', &mut buffer)? != 0 {
        write!(output, "{}", redactor.redact(std::str::from_utf8(&buffer)?))?;
        buffer.clear();
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::redact_logs;
    use redacta::ipv4::IPv4Redactor;

    #[test]
    fn single_empty_line() {
        let input = vec![];
        let mut output = vec![];
        assert!(redact_logs(&input[..], &mut output, &IPv4Redactor::default()).is_ok());
        assert_eq!(input, output);
    }

    #[test]
    fn single_line_without_newline() {
        let input = b"no newline".to_vec();
        let mut output = vec![];
        assert!(redact_logs(&input[..], &mut output, &IPv4Redactor::default()).is_ok());
        assert_eq!(input, output);
    }

    #[test]
    fn single_line_with_newline() {
        let input = b"newline\n".to_vec();
        let mut output = vec![];
        assert!(redact_logs(&input[..], &mut output, &IPv4Redactor::default()).is_ok());
        assert_eq!(input, output);
    }

    #[test]
    fn multiple_clean_lines() {
        let input = b"clean\nline\nagain\n".to_vec();
        let mut output = vec![];
        assert!(redact_logs(&input[..], &mut output, &IPv4Redactor::default()).is_ok());
        assert_eq!(input, output);
    }
}
