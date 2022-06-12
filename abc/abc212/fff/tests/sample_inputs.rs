use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"3 2 3
1 2 1 3
2 3 3 5
1 1 5
2 2 3
1 3 2
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(
        output.stdout_str(),
        r#"2 3
2
3
"#
    );
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"8 10 10
4 3 329982133 872113932
6 8 101082040 756263297
4 7 515073851 793074419
8 7 899017043 941751547
5 7 295510441 597348810
7 2 688716395 890599546
6 1 414221915 748470452
6 4 810915860 904512496
3 1 497469654 973509612
4 1 307142272 872178157
374358788 4 509276232
243448834 6 585993193
156350864 4 682491610
131643541 8 836902943
152874385 6 495945159
382276121 1 481368090
552433623 2 884584430
580376205 2 639442239
108790644 7 879874292
883275610 1 994982498
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(
        output.stdout_str(),
        r#"4
6 1
4 1
8
6 1
1
2
2
7 2
1
"#
    );
    assert!(output.stderr_str().is_empty());
}
