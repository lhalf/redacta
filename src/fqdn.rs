use crate::redact::Redact;
use crate::regex::redact;

const FQDN_REGEX: &str = r"(?i)(?:[a-z0-9](?:[a-z0-9-]{0,61}[a-z0-9])?\.)+[a-z]{2,63}";

pub struct FqdnRedactor {
    regex: regex::bytes::Regex,
}

impl Default for FqdnRedactor {
    fn default() -> Self {
        Self {
            regex: regex::bytes::Regex::new(FQDN_REGEX).expect("this regex is valid"),
        }
    }
}

impl Redact for FqdnRedactor {
    fn redact(&self, input: &mut [u8]) {
        redact(&self.regex, input);
    }
}

#[cfg(test)]
mod tests {
    use super::FqdnRedactor;
    use crate::assert_redacts;
    use crate::redact::Redact;

    #[test]
    fn nothing_to_redact() {
        assert_redacts!(FqdnRedactor::default(), b"no fqdn here", b"no fqdn here");
        assert_redacts!(FqdnRedactor::default(), b"examplecom", b"examplecom");
    }

    #[test]
    fn only_fqdn() {
        assert_redacts!(
            FqdnRedactor::default(),
            b"server1.example.com",
            b"*******************"
        );
        assert_redacts!(FqdnRedactor::default(), b"a.b.com", b"*******");
    }

    #[test]
    fn single_fqdn_in_sentence() {
        assert_redacts!(
            FqdnRedactor::default(),
            b"visit server1.example.com for more info.",
            b"visit ******************* for more info."
        );
    }

    #[test]
    fn multiple_fqdns_in_sentence() {
        assert_redacts!(
            FqdnRedactor::default(),
            b"first.test.io and second.domain.org are both valid.",
            b"************* and ***************** are both valid."
        );
    }

    #[test]
    fn multiple_back_to_back_fqdns() {
        assert_redacts!(
            FqdnRedactor::default(),
            b"checkhost.localnetwork.local",
            b"****************************"
        );
    }
}
