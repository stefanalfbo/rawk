use rawk_core::awk::Awk;

fn assert_script_output_matches(script: &str, data: &str, expected_data: &str) {
    let input: Vec<String> = data.lines().map(str::to_string).collect();
    let expected: Vec<String> = expected_data.lines().map(str::to_string).collect();

    let awk = Awk::new(script);
    let output = awk.run(input);

    assert_eq!(output, expected);
}

macro_rules! p_test {
    ($name:ident, $num:literal) => {
        #[test]
        fn $name() {
            let script = include_str!(concat!("onetrueawk-testdata/p.", $num));
            let data = include_str!("onetrueawk-testdata/countries");
            let expected_data = include_str!(concat!("onetrueawk-testdata/p.", $num, ".expected"));

            assert_script_output_matches(script, data, expected_data);
        }
    };
}

macro_rules! p_test_ignored {
    ($name:ident, $num:literal) => {
        #[test]
        #[ignore = "not supported yet"]
        fn $name() {
            let script = include_str!(concat!("onetrueawk-testdata/p.", $num));
            let data = include_str!("onetrueawk-testdata/countries");
            let expected_data = include_str!(concat!("onetrueawk-testdata/p.", $num, ".expected"));

            assert_script_output_matches(script, data, expected_data);
        }
    };
}

p_test!(p1, "1");
p_test!(p2, "2");
p_test!(p3, "3");
p_test!(p4, "4");
p_test!(p5, "5");
p_test!(p6, "6");
p_test!(p7, "7");
p_test!(p8, "8");
p_test!(p9, "9");
p_test!(p10, "10");
p_test!(p11, "11");
p_test!(p12, "12");
p_test!(p13, "13");
p_test!(p14, "14");
p_test!(p15, "15");
p_test!(p16, "16");
p_test!(p17, "17");
p_test!(p18, "18");
p_test!(p19, "19");
p_test!(p20, "20");
p_test!(p21, "21");
p_test!(p21a, "21a");
p_test!(p22, "22");
p_test!(p23, "23");
p_test!(p24, "24");
p_test!(p25, "25");
p_test!(p26, "26");
p_test!(p26a, "26a");
p_test!(p27, "27");
p_test!(p28, "28");
p_test_ignored!(p29, "29");
p_test_ignored!(p30, "30");
p_test_ignored!(p31, "31");
p_test_ignored!(p32, "32");
p_test_ignored!(p33, "33");
p_test_ignored!(p34, "34");
p_test_ignored!(p35, "35");
p_test_ignored!(p36, "36");
p_test_ignored!(p37, "37");
p_test_ignored!(p38, "38");
p_test_ignored!(p39, "39");
p_test_ignored!(p40, "40");
p_test_ignored!(p41, "41");
p_test_ignored!(p42, "42");
p_test_ignored!(p43, "43");
p_test_ignored!(p44, "44");
p_test_ignored!(p45, "45");
p_test_ignored!(p46, "46");
p_test_ignored!(p47, "47");
p_test_ignored!(p48, "48");
p_test_ignored!(p49, "49");
p_test_ignored!(p50, "50");
p_test_ignored!(p51, "51");
p_test_ignored!(p52, "52");
