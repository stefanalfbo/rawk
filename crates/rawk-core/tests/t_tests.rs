use rawk_core::awk::Awk;

fn assert_script_output_matches(script: &str, data: &str, expected_data: &str) {
    let input: Vec<String> = data.lines().map(str::to_string).collect();
    let expected: Vec<String> = expected_data.lines().map(str::to_string).collect();

    let awk = Awk::new(script);
    let output = awk.run(input, Some("onetrueawk-testdata/data".to_string()));

    assert_eq!(output, expected);
}

macro_rules! t_test {
    ($name:ident, $num:literal) => {
        #[test]
        fn $name() {
            let script = include_str!(concat!("onetrueawk-testdata/t.", $num));
            let data = include_str!("onetrueawk-testdata/data");
            let expected_data = include_str!(concat!("onetrueawk-testdata/t.", $num, ".expected"));

            assert_script_output_matches(script, data, expected_data);
        }
    };
}

macro_rules! t_test_ignore {
    ($name:ident, $num:literal) => {
        #[test]
        #[ignore]
        fn $name() {
            let script = include_str!(concat!("onetrueawk-testdata/t.", $num));
            let data = include_str!("onetrueawk-testdata/data");
            let expected_data = include_str!(concat!("onetrueawk-testdata/t.", $num, ".expected"));

            assert_script_output_matches(script, data, expected_data);
        }
    };
}

t_test!(t0, "0");
t_test!(t0a, "0a");
t_test!(t1, "1");
t_test!(t1x, "1.x");
t_test!(t2, "2");
t_test!(t2x, "2.x");
t_test!(t3, "3");
t_test_ignore!(t3x, "3.x");
t_test!(t4, "4");
t_test!(t4x, "4.x");
t_test!(t5x, "5.x");
t_test!(t6, "6");
t_test!(t6x, "6.x");
t_test!(t6a, "6a");
t_test!(t6b, "6b");
t_test!(t8x, "8.x");
t_test!(t8y, "8.y");
t_test!(ta, "a");
t_test!(taddops, "addops");
t_test!(taeiou, "aeiou");
t_test!(taeiouy, "aeiouy");
t_test_ignore!(tarith, "arith");
t_test!(tarray, "array");
t_test_ignore!(tarray1, "array1");
t_test_ignore!(tarray2, "array2");
t_test_ignore!(tassert, "assert");
t_test_ignore!(tavg, "avg");
t_test!(tbx, "b.x");
t_test_ignore!(tbe, "be");
t_test_ignore!(tbeginexit, "beginexit");
t_test_ignore!(tbeginnext, "beginnext");
t_test_ignore!(tbreak, "break");
t_test_ignore!(tbreak1, "break1");
t_test_ignore!(tbreak2, "break2");
t_test_ignore!(tbreak3, "break3");
t_test_ignore!(tbug1, "bug1");
t_test_ignore!(tbuiltins, "builtins");
t_test_ignore!(tcat, "cat");
t_test_ignore!(tcat1, "cat1");
t_test_ignore!(tcat2, "cat2");
t_test_ignore!(tcmp, "cmp");
t_test_ignore!(tcoerce, "coerce");
t_test_ignore!(tcoerce2, "coerce2");
t_test_ignore!(tcomment, "comment");
t_test_ignore!(tcomment1, "comment1");
t_test_ignore!(tconcat, "concat");
t_test_ignore!(tcond, "cond");
t_test_ignore!(tcontin, "contin");
t_test_ignore!(tcount, "count");
t_test_ignore!(tcrlf, "crlf");
t_test_ignore!(tcum, "cum");
t_test_ignore!(tdx, "d.x");
t_test_ignore!(tdelete0, "delete0");
t_test_ignore!(tdelete1, "delete1");
t_test_ignore!(tdelete2, "delete2");
t_test_ignore!(tdelete3, "delete3");
t_test_ignore!(tdo, "do");
t_test_ignore!(te, "e");
t_test_ignore!(telse, "else");
t_test_ignore!(texit, "exit");
t_test_ignore!(texit1, "exit1");
t_test!(tf, "f");
t_test!(tfx, "f.x");
t_test!(tf0, "f0");
t_test!(tf1, "f1");
t_test!(tf2, "f2");
t_test!(tf3, "f3");
t_test!(tf4, "f4");
t_test!(tfor, "for");
t_test!(tfor1, "for1");
t_test!(tfor2, "for2");
t_test_ignore!(tfor3, "for3");
t_test_ignore!(tformat4, "format4");
t_test_ignore!(tfun, "fun");
t_test_ignore!(tfun0, "fun0");
t_test_ignore!(tfun1, "fun1");
t_test_ignore!(tfun2, "fun2");
t_test_ignore!(tfun3, "fun3");
t_test_ignore!(tfun4, "fun4");
t_test_ignore!(tfun5, "fun5");
t_test_ignore!(tgetline1, "getline1");
t_test_ignore!(tgetval, "getval");
t_test_ignore!(tgsub, "gsub");
t_test_ignore!(tgsub1, "gsub1");
t_test_ignore!(tgsub3, "gsub3");
t_test_ignore!(tgsub4, "gsub4");
t_test_ignore!(tix, "i.x");
t_test_ignore!(tif, "if");
t_test_ignore!(tin, "in");
t_test_ignore!(tin1, "in1");
t_test_ignore!(tin2, "in2");
t_test_ignore!(tin3, "in3");
t_test_ignore!(tincr, "incr");
t_test_ignore!(tincr2, "incr2");
t_test_ignore!(tincr3, "incr3");
t_test_ignore!(tindex, "index");
t_test_ignore!(tintest, "intest");
t_test_ignore!(tintest2, "intest2");
t_test_ignore!(tjx, "j.x");
t_test_ignore!(tlongstr, "longstr");
t_test_ignore!(tmakef, "makef");
t_test_ignore!(tmatch, "match");
t_test_ignore!(tmatch1, "match1");
t_test_ignore!(tmax, "max");
t_test_ignore!(tmod, "mod");
t_test_ignore!(tmonotone, "monotone");
t_test_ignore!(tnameval, "nameval");
t_test_ignore!(tnext, "next");
t_test!(tnf, "NF");
t_test_ignore!(tnot, "not");
t_test_ignore!(tnull0, "null0");
t_test_ignore!(tofmt, "ofmt");
t_test_ignore!(tofs, "ofs");
t_test_ignore!(tors, "ors");
t_test_ignore!(tpat, "pat");
t_test_ignore!(tpipe, "pipe");
t_test_ignore!(tpp, "pp");
t_test_ignore!(tpp1, "pp1");
t_test_ignore!(tpp2, "pp2");
t_test_ignore!(tprintf, "printf");
t_test_ignore!(tprintf2, "printf2");
t_test_ignore!(tquote, "quote");
t_test_ignore!(trandk, "randk");
t_test_ignore!(tre1, "re1");
t_test_ignore!(tre1a, "re1a");
t_test_ignore!(tre2, "re2");
t_test_ignore!(tre3, "re3");
t_test_ignore!(tre4, "re4");
t_test_ignore!(tre5, "re5");
t_test_ignore!(tre7, "re7");
t_test_ignore!(trec, "rec");
t_test_ignore!(tredir1, "redir1");
t_test_ignore!(trefs, "reFS");
t_test_ignore!(treg, "reg");
t_test_ignore!(troff, "roff");
t_test_ignore!(tsep, "sep");
t_test_ignore!(tseqno, "seqno");
t_test_ignore!(tset0, "set0");
t_test_ignore!(tset0a, "set0a");
t_test_ignore!(tset0b, "set0b");
t_test_ignore!(tset1, "set1");
t_test_ignore!(tset2, "set2");
t_test_ignore!(tset3, "set3");
t_test_ignore!(tsplit1, "split1");
t_test_ignore!(tsplit2, "split2");
t_test_ignore!(tsplit2a, "split2a");
t_test_ignore!(tsplit3, "split3");
t_test_ignore!(tsplit4, "split4");
t_test_ignore!(tsplit8, "split8");
t_test_ignore!(tsplit9, "split9");
t_test_ignore!(tsplit9a, "split9a");
t_test_ignore!(tstately, "stately");
t_test_ignore!(tstrcmp, "strcmp");
t_test_ignore!(tstrcmp1, "strcmp1");
t_test_ignore!(tstrnum, "strnum");
t_test_ignore!(tsub0, "sub0");
t_test_ignore!(tsub1, "sub1");
t_test_ignore!(tsub2, "sub2");
t_test_ignore!(tsub3, "sub3");
t_test!(tsubstr, "substr");
t_test!(tsubstr1, "substr1");
t_test!(ttime, "time");
t_test_ignore!(tvf, "vf");
t_test!(tvf1, "vf1");
t_test!(tvf2, "vf2");
t_test!(tvf3, "vf3");
t_test!(tx, "x");
