use crate::ipv4::{IPv4Redactor, Redact};

mod ipv4;

fn main() {
    let redactor = IPv4Redactor::new();
    redactor.redact("example");
}
