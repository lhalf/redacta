#[cfg(test)]
#[macro_export]
macro_rules! assert_redacts {
    ($redactor:expr, $input:expr, $expected:expr) => {{
        let mut buf = $input.to_vec();
        $redactor.redact(&mut buf);
        assert_eq!($expected.as_ref(), buf.as_slice());
    }};
}
