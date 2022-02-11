use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin("56\n")
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "14\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin("32\n")
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "8\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin("0\n")
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "40\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample4() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin("100\n")
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "expert\n");
    assert!(output.stderr_str().is_empty());
}
