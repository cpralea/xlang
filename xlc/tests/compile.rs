use std::fs;
use std::io;
use std::process;

use std::io::Read;


#[test]
fn empty() {
    test("empty", 0);
}
#[test]
fn missing() {
    test("missing", 255);
}


#[test]
fn literals_string_escseqs() {
    test("literals_string_escseqs", 0);
}
#[test]
fn literal_malformed_integer() {
    test("literal_malformed_integer", 255);
}
#[test]
fn literal_malformed_string() {
    test("literal_malformed_string", 255);
}
#[test]
fn literal_string_escseq_invalid() {
    test("literal_string_escseq_invalid", 255);
}


#[test]
fn prints() {
    test("prints", 0);
}
#[test]
fn print_no_exp() {
    test("print_no_exp", 255);
}


#[test]
fn assignments() {
    test("assignments", 0);
}
#[test]
fn assignment_no_exp() {
    test("assignment_no_exp", 255);
}
#[test]
fn assignment_no_op_assign() {
    test("assignment_no_op_assign", 255);
}


#[test]
fn expressions_boolean() {
    test("expressions_boolean", 0);
}
#[test]
fn expressions_integer() {
    test("expressions_integer", 0);
}
#[test]
fn expression_boolint_type_mismatch() {
    test("expression_boolint_type_mismatch", 255);
}
#[test]
fn expression_boolstr_type_mismatch() {
    test("expression_boolstr_type_mismatch", 255);
}
#[test]
fn expression_intstr_type_mismatch() {
    test("expression_intstr_type_mismatch", 255);
}
#[test]
fn expression_lparen_no_exp() {
    test("expression_lparen_no_exp", 255);
}
#[test]
fn expression_lparen_exp_no_rparen() {
    test("expression_lparen_exp_no_rparen", 255);
}
#[test]
fn expression_unknown_variable() {
    test("expression_unknown_variable", 255);
}
#[test]
fn expression_operand_operator_no_operand() {
    test("expression_operand_operator_no_operand", 255);
}


fn test(file: &str, exit: i32) {
    let xl = format!("tests/xl/{}.xl", file);
    let xlc = load_file(&format!("tests/xlc/{}.xlc", file));
    assert!(xlc.is_ok(), "Failed to load reference xlc verbose output for '{}'.", file);
    let xlc = &xlc.unwrap();

    let output = process::Command::new("cargo")
        .args(&["run", "-q", "--", "--verbose", "--no-output", xl.as_str()])
        .stderr(process::Stdio::null())
        .output()
        .expect(format!("Failed to test '{}'", file).as_str());

    let code = output.status.code();
    let stdout = &*String::from_utf8_lossy(&output.stdout);

    check_code(code, exit);
    check_xlc(&stdout, &xlc);
}


fn check_code(actual: Option<i32>, expected: i32) {
    let expected = Some(expected);
    assert!(actual == expected, "Exit code mismatch. Got {:?}, expected {:?}.", actual, expected);
}


fn check_xlc(actual: &str, expected: &str) {
    let (actual, expected) = match cfg!(windows) {
        true => (actual.replace("\r", ""), expected.replace("\r", "")),
        false => (actual.to_string(), expected.to_string()),
    };
    assert!(actual == expected, "Compiler output mismatch.");
}


fn load_file(name: &String) -> Result<String, io::Error> {
    let mut file = try!(fs::File::open(name));
    let mut content = String::new();
    try!(file.read_to_string(&mut content));
    Ok(content)
}
