use std::fs;
use std::io;
use std::process;

use std::io::Read;


#[test] fn empty() {
    test("empty", 0); }
#[test] fn missing() {
    test("missing", 255); }

#[test] fn str_escseq_valid() {
    test("str_escseq_valid", 0); }
#[test] fn str_escseq_invalid() {
    test("str_escseq_invalid", 255); }

#[test] fn prints() {
    test("prints", 0); }
#[test] fn print_no_expression() {
    test("print_no_expression", 255); }
#[test] fn print_unknown_variable() {
    test("print_unknown_variable", 255); }

#[test] fn assignments() {
    test("assignments", 0); }
#[test] fn assignment_mismatched_types_int2str() {
    test("assignment_mismatched_types_int2str", 255); }
#[test] fn assignment_mismatched_types_bool2int() {
    test("assignment_mismatched_types_bool2int", 255); }
#[test] fn assignment_no_assign() {
    test("assignment_no_assign", 255); }
#[test] fn assignment_no_expression() {
    test("assignment_no_expression", 255); }
#[test] fn assignment_unknown_variable() {
    test("assignment_unknown_variable", 255); }

#[test] fn mixed_assignments_and_prints() {
    test("mixed_assignments_and_prints", 0); }


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
    assert!(actual == expected,
        "Exit code mismatch. Got {:?}, expected {:?}.",
        actual, expected);
}


fn check_xlc(actual: &str, expected: &str) {
    assert!(actual == expected,
        "Compiler output mismatch.");
}


fn load_file(name: &String) -> Result<String, io::Error> {
    let mut file = try!(fs::File::open(name));
    let mut content = String::new();
    try!(file.read_to_string(&mut content));
    Ok(content)
}
