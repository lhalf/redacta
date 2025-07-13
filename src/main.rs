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
    input: impl BufRead,
    output: &mut impl Write,
    redactor: &impl Redact,
) -> std::io::Result<()> {
    for line in input.lines() {
        writeln!(output, "{}", redactor.redact(&line?))?;
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

    // TODO: handle single line without newline

    #[test]
    fn single_clean_line() {
        let input = b"clean\n".to_vec();
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
