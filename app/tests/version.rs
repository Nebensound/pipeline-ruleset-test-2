//! The program prints its version.

#[test]
fn prints_its_version() {
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_app"))
        .arg("--version")
        .output()
        .expect("the program starts");
    let printed = String::from_utf8(output.stdout).expect("text");
    assert!(printed.starts_with("app "), "{printed}");
}
