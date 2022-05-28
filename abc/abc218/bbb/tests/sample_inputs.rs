use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26
            "#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "abcdefghijklmnopqrstuvwxyz\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"2 1 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "bacdefghijklmnopqrstuvwxyz\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"5 11 12 16 25 17 18 1 7 10 4 23 20 3 2 24 26 19 14 9 6 22 8 13 15 21
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "eklpyqragjdwtcbxzsnifvhmou\n");
    assert!(output.stderr_str().is_empty());
}
