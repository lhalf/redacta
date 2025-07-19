#[derive(Debug)]
pub struct RegexRedactor;

impl RegexRedactor {
    pub fn try_from(_regex: &str) -> Result<Self, anyhow::Error> {
        Err(anyhow::anyhow!("invalid regex"))
    }
}

#[cfg(test)]
mod tests {
    use super::RegexRedactor;
    #[test]
    fn invalid_regex() {
        assert_eq!(
            "invalid regex",
            RegexRedactor::try_from("invalid").unwrap_err().to_string()
        );
    }
}
