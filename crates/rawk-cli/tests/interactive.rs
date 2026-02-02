use std::io::Write;
use std::process::{Command, Stdio};

fn run_rawk_interactive(script: &str, input: &[u8]) -> std::process::Output {
    let rawk = env!("CARGO_BIN_EXE_rawk-cli");
    let mut child = Command::new(rawk)
        .arg(script)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("failed to spawn rawk");

    {
        let stdin = child.stdin.as_mut().expect("failed to open stdin");
        stdin.write_all(input).expect("failed to write stdin");
    }

    child.wait_with_output().expect("failed to wait on rawk")
}

fn run_rawk_interactive_from_file(input: &[u8]) -> std::process::Output {
    let rawk = env!("CARGO_BIN_EXE_rawk-cli");
    let script_path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/print.awk");
    let mut child = Command::new(rawk)
        .arg("-f")
        .arg(script_path)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("failed to spawn rawk");

    {
        let stdin = child.stdin.as_mut().expect("failed to open stdin");
        stdin.write_all(input).expect("failed to write stdin");
    }

    child.wait_with_output().expect("failed to wait on rawk")
}

#[test]
fn interactive_mode_script_on_command_line() {
    let script = "{ print $1 }";
    // Send invalid UTF-8 to force interactive mode to exit after the inputs.
    let input = b"Beth 4.00 0\nDan 3.75 0\n\xff";

    let output = run_rawk_interactive(script, input);

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut lines = stdout.lines();

    assert_eq!(lines.next(), Some("Beth"));
    assert_eq!(lines.next(), Some("Dan"));
    assert!(lines.next().is_none());
    assert!(output.stderr.is_empty());
}

#[test]
fn interactive_mode_script_from_file() {
    // Send invalid UTF-8 to force interactive mode to exit after the inputs.
    let input = b"Beth 4.00 0\nDan 3.75 0\n\xff";

    let output = run_rawk_interactive_from_file(input);

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut lines = stdout.lines();

    assert_eq!(lines.next(), Some("Beth 4.00 0"));
    assert_eq!(lines.next(), Some("Dan 3.75 0"));
    assert!(lines.next().is_none());
    assert!(output.stderr.is_empty());
}
