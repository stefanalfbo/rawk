use std::process::Command;

fn run_rawk(script: &str) -> std::process::Output {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/emp.data");
    let rawk = env!("CARGO_BIN_EXE_rawk-cli");

    Command::new(rawk)
        .arg(script)
        .arg(&path)
        .output()
        .expect("failed to run rawk")
}

fn run_rawk_from_file() -> std::process::Output {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/emp.data");
    let script_path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/print.awk");
    let rawk = env!("CARGO_BIN_EXE_rawk-cli");

    Command::new(rawk)
        .arg("-f")
        .arg(&script_path)
        .arg(&path)
        .output()
        .expect("failed to run rawk")
}

fn run_rawk_no_args() -> std::process::Output {
    let rawk = env!("CARGO_BIN_EXE_rawk-cli");

    Command::new(rawk)
        .output()
        .expect("failed to run rawk")
}

#[test]
fn print_identity_outputs_input_lines() {
    let script = "{ print }";

    let output = run_rawk(script);

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(String::from_utf8_lossy(&output.stdout).contains("Beth    4.00    0\n"));
    assert!(output.stderr.is_empty());
}

#[test]
fn print_identity_outputs_input_lines_from_file() {
    let output = run_rawk_from_file();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(String::from_utf8_lossy(&output.stdout).contains("Beth    4.00    0\n"));
    assert!(output.stderr.is_empty());
}

#[test]
fn print_begin_end_blocks_with_expressions() {
    let script = "BEGIN { print 1+1 } END { print 3 + 3}";

    let output = run_rawk(script);

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut lines = stdout.lines();

    assert_eq!(lines.next(), Some("2"));
    assert_eq!(lines.next(), Some("6"));
    assert!(output.stderr.is_empty());
}

#[test]
fn print_begin_rules_and_end_blocks_with_expressions() {
    let script = "BEGIN { print 10 } { print $1 } END { print 20 }";

    let output = run_rawk(script);

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut lines = stdout.lines();

    assert_eq!(lines.next(), Some("10"));
    assert_eq!(lines.next(), Some("Beth"));
    assert_eq!(lines.next(), Some("Dan"));
    assert_eq!(lines.next(), Some("Kathy"));
    assert_eq!(lines.next(), Some("Mark"));
    assert_eq!(lines.next(), Some("Mary"));
    assert_eq!(lines.next(), Some("Susie"));
    assert_eq!(lines.next(), Some("20"));
    assert!(output.stderr.is_empty());
}

#[test]
fn help_shows_when_no_args_given() {
    let output = run_rawk_no_args();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(stdout.contains("Usage:"), "stdout: {stdout}");
    assert!(stdout.contains("ARGS"), "stdout: {stdout}");
    assert!(stdout.contains("file"), "stdout: {stdout}");
    assert!(output.stderr.is_empty());
}
