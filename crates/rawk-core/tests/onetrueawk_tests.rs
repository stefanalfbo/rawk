use rawk_core::awk::Awk;

#[test]
fn p1() {
    let script = include_str!("onetrueawk-testdata/p.1");
    let data = include_str!("onetrueawk-testdata/countries");
    let expected_data = include_str!("onetrueawk-testdata/p.1.expected");

    let input: Vec<String> = data.lines().map(str::to_string).collect();
    let expected: Vec<String> = expected_data.lines().map(str::to_string).collect();

    let awk = Awk::new(script);
    let output = awk.run(input);

    assert_eq!(output, expected);
}
