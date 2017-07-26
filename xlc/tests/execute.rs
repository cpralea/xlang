use std::fs;
use std::io;
use std::process;

use std::io::Read;


#[test]
fn empty() {
    test("empty", 0);
}
#[test]
fn literals_string_escseqs() {
    test("literals_string_escseqs", 0);
}
#[test]
fn prints() {
    test("prints", 0);
}
#[test]
fn assignments() {
    test("assignments", 0);
}
#[test]
fn expressions_boolean() {
    test("expressions_boolean", 0);
}
#[test]
fn expressions_integer() {
    test("expressions_integer", 0);
}


fn test(file: &str, exit: i32) {
    let xl = format!("tests/xl/{}.xl", file);
    let out = load_file(&format!("tests/out/{}.out", file));
    assert!(out.is_ok(), "Failed to load reference program output for '{}'.", file);
    let out = &out.unwrap();

    let python = match cfg!(windows) {
        true => "py.exe",
        false => "python3",
    };
    let output = process::Command::new(python)
        .args(&["tools/xl.py", "--quiet", xl.as_str()])
        .stderr(process::Stdio::null())
        .output()
        .expect(format!("Failed to test '{}'", file).as_str());

    let code = output.status.code();
    let stdout = &*String::from_utf8_lossy(&output.stdout);

    check_code(code, exit);
    check_out(&stdout, &out);
}


fn check_code(actual: Option<i32>, expected: i32) {
    let expected = Some(expected);
    assert!(actual == expected, "Exit code mismatch. Got {:?}, expected {:?}.", actual, expected);
}


fn check_out(actual: &str, expected: &str) {
    let (actual, expected) = match cfg!(windows) {
        true => (actual.replace("\r", ""), expected.replace("\r", "")),
        false => (actual.to_string(), expected.to_string()),
    };
    assert!(actual == expected, "Program output mismatch.");
}


fn load_file(name: &String) -> Result<String, io::Error> {
    let mut file = try!(fs::File::open(name));
    let mut content = String::new();
    try!(file.read_to_string(&mut content));
    Ok(content)
}
