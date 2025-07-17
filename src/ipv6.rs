use crate::redact::Redact;
use std::borrow::Cow;

#[derive(Default)]
pub struct IPv6Redactor;

impl Redact for IPv6Redactor {
    fn redact<'a>(&self, input: &'a str) -> Cow<'a, str> {
        input.into()
    }
}

#[cfg(test)]
mod tests {
    use super::IPv6Redactor;
    use crate::redact::Redact;

    #[test]
    fn nothing_to_redact() {
        let redactor = IPv6Redactor;
        assert_eq!("clean", redactor.redact("clean"));
        assert_eq!("colons::", redactor.redact("colons::"));
        assert_eq!(
            "colons and numbers 0000::",
            redactor.redact("colons and numbers 0000::")
        );
    }
}
