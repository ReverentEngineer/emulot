use std::process::Command;

#[test]
fn validate_guest_config_toml() {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let bin = env!("CARGO_BIN_EXE_emulot");
    let output = Command::new(format!("{bin}"))
        .arg("run")
        .arg("--validate")
        .arg(format!("{0}/tests/data/test.toml", manifest_dir))
        .output()
        .unwrap();
    assert!(output.status.success())
}
