use crate::redact::Redact;
use crate::regex::redact;

const IPV6_REGEX: &str = r"(([0-9a-fA-F]{1,4}:){7,7}[0-9a-fA-F]{1,4}|([0-9a-fA-F]{1,4}:){1,7}:|([0-9a-fA-F]{1,4}:){1,6}:[0-9a-fA-F]{1,4}|([0-9a-fA-F]{1,4}:){1,5}(:[0-9a-fA-F]{1,4}){1,2}|([0-9a-fA-F]{1,4}:){1,4}(:[0-9a-fA-F]{1,4}){1,3}|([0-9a-fA-F]{1,4}:){1,3}(:[0-9a-fA-F]{1,4}){1,4}|([0-9a-fA-F]{1,4}:){1,2}(:[0-9a-fA-F]{1,4}){1,5}|[0-9a-fA-F]{1,4}:((:[0-9a-fA-F]{1,4}){1,6})|:((:[0-9a-fA-F]{1,4}){1,7}|:)|fe80:(:[0-9a-fA-F]{0,4}){0,4}%[0-9a-zA-Z]{1,}|::(ffff(:0{1,4}){0,1}:){0,1}((25[0-5]|(2[0-4]|1{0,1}[0-9]){0,1}[0-9])\.){3,3}(25[0-5]|(2[0-4]|1{0,1}[0-9]){0,1}[0-9])|([0-9a-fA-F]{1,4}:){1,4}:((25[0-5]|(2[0-4]|1{0,1}[0-9]){0,1}[0-9])\.){3,3}(25[0-5]|(2[0-4]|1{0,1}[0-9]){0,1}[0-9]))";

pub struct IPv6Redactor {
    regex: regex::bytes::Regex,
}

impl Default for IPv6Redactor {
    fn default() -> Self {
        Self {
            regex: regex::bytes::Regex::new(IPV6_REGEX).expect("this regex is valid"),
        }
    }
}

impl Redact for IPv6Redactor {
    fn redact(&self, input: &mut [u8]) {
        redact(&self.regex, input);
    }
}

#[cfg(test)]
mod tests {
    use super::IPv6Redactor;
    use crate::assert_redacts;
    use crate::redact::Redact;

    #[test]
    fn nothing_to_redact() {
        assert_redacts!(IPv6Redactor::default(), b"clean", b"clean");
        assert_redacts!(IPv6Redactor::default(), b"colon:", b"colon:");
        assert_redacts!(
            IPv6Redactor::default(),
            b"colon and number 0:",
            b"colon and number 0:"
        );
    }

    #[test]
    fn only_ipv6() {
        assert_redacts!(
            IPv6Redactor::default(),
            b"1:2:3:4:5:6:7:8",
            b"***************"
        );
        assert_redacts!(IPv6Redactor::default(), b"1::", b"***");
        assert_redacts!(IPv6Redactor::default(), b"::8", b"***");
        assert_redacts!(IPv6Redactor::default(), b"::7:8", b"*****");
        assert_redacts!(IPv6Redactor::default(), b"::6:7:8", b"*******");
        assert_redacts!(IPv6Redactor::default(), b"::5:6:7:8", b"*********");
        assert_redacts!(IPv6Redactor::default(), b"::4:5:6:7:8", b"***********");
        assert_redacts!(IPv6Redactor::default(), b"::3:4:5:6:7:8", b"*************");
        assert_redacts!(
            IPv6Redactor::default(),
            b"::2:3:4:5:6:7:8",
            b"***************"
        );
        assert_redacts!(
            IPv6Redactor::default(),
            b"2001:db8:3333:4444:5555:6666:7777:8888",
            b"**************************************"
        );
        assert_redacts!(
            IPv6Redactor::default(),
            b"2001:0db8:85a3:0000:0000:8a2e:0370:7334",
            b"***************************************"
        );
    }

    #[test]
    fn single_ipv6_in_sentence() {
        assert_redacts!(
            IPv6Redactor::default(),
            b"Sentence with 2001:db8:3333:4444:5555:6666:7777:8888 here.",
            b"Sentence with ************************************** here."
        );
    }

    #[test]
    fn multiple_ipv6s_in_sentence() {
        assert_redacts!(
            IPv6Redactor::default(),
            b"::1 and 2001:db8:3333:4444:5555:6666:7777:8888 here.",
            b"*** and ************************************** here."
        );
    }

    #[test]
    fn single_nested_ipv6() {
        assert_redacts!(
            IPv6Redactor::default(),
            b"Sentence with2001:db8:3333:4444:5555:6666:7777:8888nested.",
            b"Sentence with**************************************nested."
        );
    }

    #[test]
    fn multiple_nested_ipv6s() {
        assert_redacts!(
            IPv6Redactor::default(),
            b"2001:db8:3333:4444:5555:6666:7777:8888and2001:db8:3333:4444:5555:6666:7777:8888.",
            b"**************************************and**************************************."
        );
    }

    #[test]
    fn multiple_back_to_back_ipv6s() {
        assert_redacts!(
            IPv6Redactor::default(),
            b"2001:db8:3333:4444:5555:6666:7777:88882001:db8:3333:4444:5555:6666:7777:88882001:db8:3333:4444:5555:6666:7777:8888",
            b"******************************************************************************************************************"
        );
    }
}
