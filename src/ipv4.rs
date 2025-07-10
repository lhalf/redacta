const IPV4_REGEX: &str =
    r"(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)";

pub struct IPv4Redactor {
    regex: regex::Regex,
}

impl IPv4Redactor {
    pub fn new() -> Self {
        Self {
            regex: regex::Regex::new(IPV4_REGEX).expect("this regex is valid"),
        }
    }
}

pub trait Redact {
    fn redact(&self, input: &str) -> String;
}

impl Redact for IPv4Redactor {
    fn redact(&self, input: &str) -> String {
        self.regex
            .replace_all(input, |captures: &regex::Captures| {
                "*".repeat(captures[0].len())
            })
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::{IPv4Redactor, Redact};

    #[test]
    fn nothing_to_redact() {
        let redactor = IPv4Redactor::new();
        assert_eq!("clean", redactor.redact("clean"));
        assert_eq!("full stop.", redactor.redact("full stop."));
        assert_eq!(
            "full stop and number 192.",
            redactor.redact("full stop and number 192.")
        );
    }

    #[test]
    fn only_ipv4() {
        let redactor = IPv4Redactor::new();
        assert_eq!("***********", redactor.redact("192.168.0.1"));
        assert_eq!("***********", redactor.redact("192.168.0.2"));
    }

    #[test]
    fn single_ipv4_in_sentence() {
        let redactor = IPv4Redactor::new();
        assert_eq!(
            "Sentence with *********** here.",
            redactor.redact("Sentence with 192.168.0.1 here.")
        );
    }

    #[test]
    fn multiple_ipv4s_in_sentence() {
        let redactor = IPv4Redactor::new();
        assert_eq!(
            "*************** and *********** here.",
            redactor.redact("255.255.255.255 and 192.168.0.1 here.")
        );
    }

    #[test]
    fn single_nested_ipv4() {
        let redactor = IPv4Redactor::new();
        assert_eq!(
            "Sentence with***********nested.",
            redactor.redact("Sentence with192.168.0.1nested.")
        );
    }

    #[test]
    fn multiple_nested_ipv4s() {
        let redactor = IPv4Redactor::new();
        assert_eq!(
            "***************and***********.",
            redactor.redact("255.255.255.255and192.168.0.1.")
        );
    }

    #[test]
    fn multiple_back_to_back_ipv4s() {
        let redactor = IPv4Redactor::new();
        assert_eq!(
            "************************************************",
            redactor.redact("255.255.255.255192.168.0.1172.168.1.1255.1.255.1")
        );
    }
}
