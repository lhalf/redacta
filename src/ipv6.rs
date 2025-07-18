use crate::redact::Redact;
use std::borrow::Cow;

const IPV6_REGEX: &str = r"(([0-9a-fA-F]{1,4}:){7,7}[0-9a-fA-F]{1,4}|([0-9a-fA-F]{1,4}:){1,7}:|([0-9a-fA-F]{1,4}:){1,6}:[0-9a-fA-F]{1,4}|([0-9a-fA-F]{1,4}:){1,5}(:[0-9a-fA-F]{1,4}){1,2}|([0-9a-fA-F]{1,4}:){1,4}(:[0-9a-fA-F]{1,4}){1,3}|([0-9a-fA-F]{1,4}:){1,3}(:[0-9a-fA-F]{1,4}){1,4}|([0-9a-fA-F]{1,4}:){1,2}(:[0-9a-fA-F]{1,4}){1,5}|[0-9a-fA-F]{1,4}:((:[0-9a-fA-F]{1,4}){1,6})|:((:[0-9a-fA-F]{1,4}){1,7}|:)|fe80:(:[0-9a-fA-F]{0,4}){0,4}%[0-9a-zA-Z]{1,}|::(ffff(:0{1,4}){0,1}:){0,1}((25[0-5]|(2[0-4]|1{0,1}[0-9]){0,1}[0-9])\.){3,3}(25[0-5]|(2[0-4]|1{0,1}[0-9]){0,1}[0-9])|([0-9a-fA-F]{1,4}:){1,4}:((25[0-5]|(2[0-4]|1{0,1}[0-9]){0,1}[0-9])\.){3,3}(25[0-5]|(2[0-4]|1{0,1}[0-9]){0,1}[0-9]))";

pub struct IPv6Redactor {
    regex: regex::Regex,
}

impl Default for IPv6Redactor {
    fn default() -> Self {
        Self {
            regex: regex::Regex::new(IPV6_REGEX).expect("this regex is valid"),
        }
    }
}

impl Redact for IPv6Redactor {
    fn redact<'a>(&self, input: &'a str) -> Cow<'a, str> {
        self.regex.replace_all(input, |captures: &regex::Captures| {
            "*".repeat(captures[0].len())
        })
    }
}

#[cfg(test)]
mod tests {
    use super::IPv6Redactor;
    use crate::redact::Redact;

    #[test]
    fn nothing_to_redact() {
        let redactor = IPv6Redactor::default();
        assert_eq!("clean", redactor.redact("clean"));
        assert_eq!("colon:", redactor.redact("colon:"));
        assert_eq!(
            "colon and number 0:",
            redactor.redact("colon and number 0:")
        );
    }

    #[test]
    fn only_ipv6() {
        let redactor = IPv6Redactor::default();
        assert_eq!("***************", redactor.redact("1:2:3:4:5:6:7:8"));
        assert_eq!("***", redactor.redact("1::"));
        assert_eq!("***", redactor.redact("::8"));
        assert_eq!("*****", redactor.redact("::7:8"));
        assert_eq!("*******", redactor.redact("::6:7:8"));
        assert_eq!("*********", redactor.redact("::5:6:7:8"));
        assert_eq!("***********", redactor.redact("::4:5:6:7:8"));
        assert_eq!("*************", redactor.redact("::3:4:5:6:7:8"));
        assert_eq!("***************", redactor.redact("::2:3:4:5:6:7:8"));
        assert_eq!(
            "**************************************",
            redactor.redact("2001:db8:3333:4444:5555:6666:7777:8888")
        );
        assert_eq!(
            "***************************************",
            redactor.redact("2001:0db8:85a3:0000:0000:8a2e:0370:7334")
        );
    }
}
