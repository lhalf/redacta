use crate::redact::Redact;
use crate::regex::redact;

const IPV4_REGEX: &str =
    r"(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)";

pub struct IPv4Redactor {
    regex: regex::bytes::Regex,
}

impl Default for IPv4Redactor {
    fn default() -> Self {
        Self {
            regex: regex::bytes::Regex::new(IPV4_REGEX).expect("this regex is valid"),
        }
    }
}

impl Redact for IPv4Redactor {
    fn redact(&self, input: &mut [u8]) {
        redact(&self.regex, input);
    }
}

#[cfg(test)]
mod tests {
    use super::IPv4Redactor;
    use crate::assert_redacts;
    use crate::redact::Redact;

    #[test]
    fn nothing_to_redact() {
        assert_redacts!(IPv4Redactor::default(), b"clean", b"clean");
        assert_redacts!(IPv4Redactor::default(), b"full stop.", b"full stop.");
        assert_redacts!(
            IPv4Redactor::default(),
            b"full stop and number 192.",
            b"full stop and number 192."
        );
    }

    #[test]
    fn only_ipv4() {
        assert_redacts!(IPv4Redactor::default(), b"192.168.0.1", b"***********");
        assert_redacts!(IPv4Redactor::default(), b"192.168.0.2", b"***********");
    }

    #[test]
    fn single_ipv4_in_sentence() {
        assert_redacts!(
            IPv4Redactor::default(),
            b"Sentence with 192.168.0.1 here.",
            b"Sentence with *********** here."
        );
    }

    #[test]
    fn multiple_ipv4s_in_sentence() {
        assert_redacts!(
            IPv4Redactor::default(),
            b"255.255.255.255 and 192.168.0.1 here.",
            b"*************** and *********** here."
        );
    }

    #[test]
    fn single_nested_ipv4() {
        assert_redacts!(
            IPv4Redactor::default(),
            b"Sentence with192.168.0.1nested.",
            b"Sentence with***********nested."
        );
    }

    #[test]
    fn multiple_nested_ipv4s() {
        assert_redacts!(
            IPv4Redactor::default(),
            b"255.255.255.255and192.168.0.1.",
            b"***************and***********."
        );
    }

    #[test]
    fn multiple_back_to_back_ipv4s() {
        assert_redacts!(
            IPv4Redactor::default(),
            b"255.255.255.255192.168.0.1172.168.1.1255.1.255.1",
            b"************************************************"
        );
    }
}
