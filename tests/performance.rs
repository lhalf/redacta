use colored::Colorize;
use std::io::Cursor;
use std::io::Write;
use std::process::{Command, Stdio};
use std::time::Instant;

#[test]
fn one_million_logs_no_redaction() {
    let num_logs = 1000000;
    let input = random_logs(num_logs);
    let mut reader = Cursor::new(input.clone());

    let mut child = Command::new("./target/release/redacta")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let mut stdin = child.stdin.take().unwrap();

    let start = Instant::now();

    std::thread::spawn(move || {
        std::io::copy(&mut reader, &mut stdin).unwrap();
    });

    let output = child.wait_with_output().unwrap();

    let duration = start.elapsed();

    assert!(output.status.success());
    assert_eq!(input, output.stdout);
    println!(
        "processed {} logs in {} ms",
        num_logs.to_string().green(),
        duration.as_millis().to_string().green()
    );
}

fn random_logs(num_logs: u64) -> Vec<u8> {
    let mut logs = Vec::new();

    for _ in 0..num_logs {
        writeln!(&mut logs, "test log").unwrap();
    }

    logs
}
