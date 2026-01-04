use std::process::Command;

fn run_rawk(awk_script: &str) -> std::process::Output {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/emp.data");
    let rawk = env!("CARGO_BIN_EXE_rawk");

    Command::new(rawk)
        .arg(awk_script)
        .arg(&path)
        .output()
        .expect("failed to run rawk")
}

#[test]
fn print_identity_outputs_input_lines() {
    let awk_script = "{ print }";

    let output = run_rawk(awk_script);

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        "Parsed Program: { print }\n"
    );
    assert!(output.stderr.is_empty());
}
