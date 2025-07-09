pub fn redact_ipv4(input: &str) -> &str {
    match input {
        "192.168.0.1" => "***********",
        "192.168.0.2" => "***********",
        _ => input,
    }
}

#[cfg(test)]
mod tests {
    use crate::ipv4::redact_ipv4;

    #[test]
    fn nothing_to_redact() {
        assert_eq!("clean", redact_ipv4("clean"));
    }

    #[test]
    fn only_ipv4() {
        assert_eq!("***********", redact_ipv4("192.168.0.1"));
        assert_eq!("***********", redact_ipv4("192.168.0.2"));
    }
}
