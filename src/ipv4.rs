pub fn redact_ipv4(input: &str) -> &str {
    input
}

#[cfg(test)]
mod tests {
    use crate::ipv4::redact_ipv4;

    #[test]
    fn nothing_to_redact() {
        assert_eq!("clean", redact_ipv4("clean"));
    }
}
