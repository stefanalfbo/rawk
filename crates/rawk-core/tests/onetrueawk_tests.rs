use rawk_core::awk::Awk;

fn assert_script_output_matches(script: &str, data: &str, expected_data: &str) {
    let input: Vec<String> = data.lines().map(str::to_string).collect();
    let expected: Vec<String> = expected_data.lines().map(str::to_string).collect();

    let awk = Awk::new(script);
    let output = awk.run(input);

    assert_eq!(output, expected);
}

#[test]
fn p1() {
    let script = include_str!("onetrueawk-testdata/p.1");
    let data = include_str!("onetrueawk-testdata/countries");
    let expected_data = include_str!("onetrueawk-testdata/p.1.expected");

    assert_script_output_matches(script, data, expected_data);
}

#[test]
fn p2() {
    let script = include_str!("onetrueawk-testdata/p.2");
    let data = include_str!("onetrueawk-testdata/countries");
    let expected_data = include_str!("onetrueawk-testdata/p.2.expected");

    assert_script_output_matches(script, data, expected_data);
}

#[test]
fn p3() {
    let script = include_str!("onetrueawk-testdata/p.3");
    let data = include_str!("onetrueawk-testdata/countries");
    let expected_data = include_str!("onetrueawk-testdata/p.3.expected");

    assert_script_output_matches(script, data, expected_data);
}

#[test]
fn p4() {
    let script = include_str!("onetrueawk-testdata/p.4");
    let data = include_str!("onetrueawk-testdata/countries");
    let expected_data = include_str!("onetrueawk-testdata/p.4.expected");

    assert_script_output_matches(script, data, expected_data);
}

#[test]
fn p5() {
    let script = include_str!("onetrueawk-testdata/p.5");
    let data = include_str!("onetrueawk-testdata/countries");
    let expected_data = include_str!("onetrueawk-testdata/p.5.expected");

    assert_script_output_matches(script, data, expected_data);
}

#[test]
fn p6() {
    let script = include_str!("onetrueawk-testdata/p.6");
    let data = include_str!("onetrueawk-testdata/countries");
    let expected_data = include_str!("onetrueawk-testdata/p.6.expected");

    assert_script_output_matches(script, data, expected_data);
}
