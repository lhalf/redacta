use std::io::{BufRead, Write};

pub trait Redact {
    fn redact(&self, input: &mut [u8]);
}

pub fn redact_logs(
    mut input: impl BufRead,
    output: &mut impl Write,
    redactors: &[Box<dyn Redact>],
) -> anyhow::Result<()> {
    let mut buffer = Vec::with_capacity(1024);

    while input.read_until(b'\n', &mut buffer)? != 0 {
        for redactor in redactors {
            redactor.redact(&mut buffer);
        }

        output.write_all(&buffer)?;
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
