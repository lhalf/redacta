use std::borrow::Cow;

pub trait Redact {
    fn redact<'a>(&self, input: &'a str) -> Cow<'a, str>;
}
