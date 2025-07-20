use crate::redact::Redact;

pub fn redact(regex: &regex::bytes::Regex, input: &mut [u8]) {
    let ranges: Vec<(usize, usize)> = regex
        .find_iter(input)
        .map(|m| (m.start(), m.end()))
        .collect();

    ranges
        .into_iter()
        .for_each(|(start, end)| input[start..end].fill(b'*'));
}

#[derive(Debug)]
pub struct RegexRedactor {
    regex: regex::bytes::Regex,
}

impl RegexRedactor {
    pub fn try_from(regex: &str) -> Result<Self, anyhow::Error> {
        Ok(Self {
            regex: regex::bytes::Regex::new(regex).map_err(|_| anyhow::anyhow!("invalid regex"))?,
        })
    }
}

impl Redact for RegexRedactor {
    fn redact(&self, input: &mut [u8]) {
        redact(&self.regex, input);
    }
}

#[cfg(test)]
mod tests {
    use super::RegexRedactor;
    use crate::assert_redacts;
    use crate::redact::Redact;

    #[test]
    fn invalid_regex() {
        assert_eq!(
            "invalid regex",
            RegexRedactor::try_from("invalid(").unwrap_err().to_string()
        );
    }

    #[test]
    fn nothing_to_redact() {
        assert_redacts!(
            RegexRedactor::try_from("^hello").unwrap(),
            b"clean",
            b"clean"
        );
    }

    #[test]
    fn matching_regex() {
        assert_redacts!(
            RegexRedactor::try_from("^hello").unwrap(),
            b"hello world!",
            b"***** world!"
        );
    }
}
