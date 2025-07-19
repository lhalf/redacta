use crate::redact::Redact;
use std::borrow::Cow;

#[derive(Debug)]
pub struct RegexRedactor {
    regex: regex::Regex,
}

impl RegexRedactor {
    pub fn try_from(regex: &str) -> Result<Self, anyhow::Error> {
        Ok(Self {
            regex: regex::Regex::new(regex).map_err(|_| anyhow::anyhow!("invalid regex"))?,
        })
    }
}

impl Redact for RegexRedactor {
    fn redact<'a>(&self, input: &'a str) -> Cow<'a, str> {
        self.regex.replace_all(input, |captures: &regex::Captures| {
            "*".repeat(captures[0].len())
        })
    }
}

#[cfg(test)]
mod tests {
    use super::RegexRedactor;
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
        let redactor = RegexRedactor::try_from("^hello").unwrap();
        assert_eq!("clean", redactor.redact("clean"));
    }

    #[test]
    fn matching_regex() {
        let redactor = RegexRedactor::try_from("^hello").unwrap();
        assert_eq!("***** world!", redactor.redact("hello world!"));
    }
}
