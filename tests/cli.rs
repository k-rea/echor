use predicates::prelude::predicate;
use assert_cmd::Command;
use std::fs;

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn dies_no_args() -> TestResult {
    Command::cargo_bin("echor")?
        .assert()
        .failure()
        .stderr(predicate::str::contains("Usage"));
    Ok(())
}

fn run(args: &[&str], expected_file: &str) -> TestResult {
    let expected = fs::read_to_string(expected_file).unwrap();
    Command::cargo_bin("echor")?.args(args).assert().success().stdout(expected);
    Ok(())
}

#[test]
fn hello_there1() -> TestResult {
    run(&["Hello there"], "tests/expected/hello-there1.txt")
}

#[test]
fn hello_there2() -> TestResult {
    run(&["Hello", "there"], "tests/expected/hello-there2.txt")
}

#[test]
fn hello_there_1_no_newline() -> TestResult {
    run(&["-n" ,"Hello", "there"], "tests/expected/hello-there2.n.txt")
}

#[test]
fn hello_there_2_no_newline() -> TestResult {
    run(&["Hello", "there", "-n"], "tests/expected/hello-there2.n.txt")
}
