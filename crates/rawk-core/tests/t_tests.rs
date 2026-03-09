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
            let script = include_str!(concat!("onetrueawk-testdata/t.", $num, ".awk"));
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
            let script = include_str!(concat!("onetrueawk-testdata/t.", $num, ".awk"));
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
t_test!(t3x, "3.x");
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
t_test!(tarith, "arith");
t_test!(tarray, "array");
t_test!(tarray1, "array1");
t_test!(tarray2, "array2");
t_test!(tassert, "assert");
t_test!(tavg, "avg");
t_test!(tbx, "b.x");
t_test!(tbe, "be");
t_test!(tbeginexit, "beginexit");
t_test!(tbeginnext, "beginnext");
t_test!(tbreak, "break");
t_test!(tbreak1, "break1");
t_test!(tbreak2, "break2");
t_test!(tbreak3, "break3");
t_test!(tbug1, "bug1");
t_test!(tbuiltins, "builtins");
t_test!(tcat, "cat");
t_test!(tcat1, "cat1");
t_test!(tcat2, "cat2");
t_test!(tcmp, "cmp");
t_test!(tcoerce, "coerce");
t_test!(tcoerce2, "coerce2");
t_test!(tcomment, "comment");
t_test!(tcomment1, "comment1");
t_test!(tconcat, "concat");
t_test!(tcond, "cond");
t_test!(tcontin, "contin");
t_test!(tcount, "count");
t_test!(tcrlf, "crlf");
t_test!(tcum, "cum");
t_test!(tdx, "d.x");
t_test!(tdelete0, "delete0");
t_test!(tdelete1, "delete1");
t_test!(tdelete2, "delete2");
t_test!(tdelete3, "delete3");
t_test!(tdo, "do");
t_test!(te, "e");
t_test!(telse, "else");
t_test!(texit, "exit");
t_test!(texit1, "exit1");
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
t_test!(tfor3, "for3");
t_test!(tformat4, "format4");
t_test!(tfun, "fun");
t_test!(tfun0, "fun0");
t_test!(tfun1, "fun1");
t_test!(tfun2, "fun2");
t_test!(tfun3, "fun3");
t_test!(tfun4, "fun4");
t_test!(tfun5, "fun5");
t_test!(tgetline1, "getline1");
t_test!(tgetval, "getval");
t_test!(tgsub, "gsub");
t_test!(tgsub1, "gsub1");
t_test!(tgsub3, "gsub3");
t_test!(tgsub4, "gsub4");
t_test!(tix, "i.x");
t_test!(tif, "if");
t_test!(tin, "in");
t_test!(tin1, "in1");
t_test!(tin2, "in2");
t_test!(tin3, "in3");
t_test!(tincr, "incr");
t_test!(tincr2, "incr2");
t_test!(tincr3, "incr3");
t_test!(tindex, "index");
t_test!(tintest, "intest");
t_test_ignore!(tintest2, "intest2");
t_test!(tjx, "j.x");
t_test!(tlongstr, "longstr");
t_test!(tmakef, "makef");
t_test!(tmatch, "match");
t_test_ignore!(tmatch1, "match1");
t_test!(tmax, "max");
t_test!(tmod, "mod");
t_test!(tmonotone, "monotone");
t_test!(tnameval, "nameval");
t_test!(tnext, "next");
t_test!(tnf, "NF");
t_test_ignore!(tnot, "not");
t_test_ignore!(tnull0, "null0");
t_test!(tofmt, "ofmt");
t_test_ignore!(tofs, "ofs");
t_test_ignore!(tors, "ors");
t_test!(tpat, "pat");
t_test!(tpipe, "pipe");
t_test!(tpp, "pp");
t_test!(tpp1, "pp1");
t_test!(tpp2, "pp2");
t_test_ignore!(tprintf, "printf");
t_test_ignore!(tprintf2, "printf2");
t_test_ignore!(tquote, "quote");
t_test_ignore!(trandk, "randk");
t_test!(tre1, "re1");
t_test_ignore!(tre1a, "re1a");
t_test_ignore!(tre2, "re2");
t_test_ignore!(tre3, "re3");
t_test!(tre4, "re4");
t_test!(tre5, "re5");
t_test!(tre7, "re7");
t_test!(trec, "rec");
t_test!(tredir1, "redir1");
t_test_ignore!(trefs, "reFS");
t_test_ignore!(treg, "reg");
t_test!(troff, "roff");
t_test!(tsep, "sep");
t_test!(tseqno, "seqno");
t_test!(tset0, "set0");
t_test!(tset0a, "set0a");
t_test_ignore!(tset0b, "set0b");
t_test!(tset1, "set1");
t_test!(tset2, "set2");
t_test!(tset3, "set3");
t_test_ignore!(tsplit1, "split1");
t_test!(tsplit2, "split2");
t_test!(tsplit2a, "split2a");
t_test_ignore!(tsplit3, "split3");
t_test_ignore!(tsplit4, "split4");
t_test_ignore!(tsplit8, "split8");
t_test_ignore!(tsplit9, "split9");
t_test_ignore!(tsplit9a, "split9a");
t_test!(tstately, "stately");
t_test!(tstrcmp, "strcmp");
t_test!(tstrcmp1, "strcmp1");
t_test_ignore!(tstrnum, "strnum");
t_test_ignore!(tsub0, "sub0");
t_test!(tsub1, "sub1");
t_test!(tsub2, "sub2");
t_test!(tsub3, "sub3");
t_test!(tsubstr, "substr");
t_test!(tsubstr1, "substr1");
t_test!(ttime, "time");
t_test!(tvf, "vf");
t_test!(tvf1, "vf1");
t_test!(tvf2, "vf2");
t_test!(tvf3, "vf3");
t_test!(tx, "x");
