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
