use std::borrow::Cow;
use std::io::{BufRead, Write};

pub trait Redact {
    fn redact<'a>(&self, input: &'a str) -> Cow<'a, str>;
}

pub fn redact_logs(
    mut input: impl BufRead,
    output: &mut impl Write,
    redactors: &[Box<dyn Redact>],
) -> anyhow::Result<()> {
    let mut buffer = Vec::with_capacity(1024);
    let mut owned_buffer = String::with_capacity(1024);

    while input.read_until(b'\n', &mut buffer)? != 0 {
        let mut line = std::str::from_utf8(&buffer)?;

        for redactor in redactors {
            match redactor.redact(line) {
                Cow::Owned(redacted) => {
                    owned_buffer.clear();
                    owned_buffer.push_str(&redacted);
                    line = &owned_buffer;
                }
                _ => continue,
            }
        }

        write!(output, "{line}")?;
        buffer.clear();
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::redact_logs;
    use crate::ipv4::IPv4Redactor;

    #[test]
    fn single_empty_line() {
        let input = vec![];
        let mut output = vec![];
        assert!(
            redact_logs(
                &input[..],
                &mut output,
                &[Box::new(IPv4Redactor::default())]
            )
            .is_ok()
        );
        assert_eq!(input, output);
    }

    #[test]
    fn single_line_without_newline() {
        let input = b"no newline".to_vec();
        let mut output = vec![];
        assert!(
            redact_logs(
                &input[..],
                &mut output,
                &[Box::new(IPv4Redactor::default())]
            )
            .is_ok()
        );
        assert_eq!(input, output);
    }

    #[test]
    fn single_line_with_newline() {
        let input = b"newline\n".to_vec();
        let mut output = vec![];
        assert!(
            redact_logs(
                &input[..],
                &mut output,
                &[Box::new(IPv4Redactor::default())]
            )
            .is_ok()
        );
        assert_eq!(input, output);
    }

    #[test]
    fn multiple_clean_lines() {
        let input = b"clean\nline\nagain\n".to_vec();
        let mut output = vec![];
        assert!(
            redact_logs(
                &input[..],
                &mut output,
                &[Box::new(IPv4Redactor::default())]
            )
            .is_ok()
        );
        assert_eq!(input, output);
    }
}
