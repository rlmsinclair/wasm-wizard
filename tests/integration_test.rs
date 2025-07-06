use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

#[test]
fn test_help_command() {
    let mut cmd = Command::cargo_bin("wasm-wizard").unwrap();
    cmd.arg("--help");
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("wasm-wizard"))
        .stdout(predicate::str::contains("WebAssembly Component Model"));
}

#[test]
fn test_version_command() {
    let mut cmd = Command::cargo_bin("wasm-wizard").unwrap();
    cmd.arg("--version");
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("wasm-wizard"));
}

#[test]
fn test_new_command() {
    let temp_dir = TempDir::new().unwrap();
    let project_path = temp_dir.path().join("test-project");
    
    let mut cmd = Command::cargo_bin("wasm-wizard").unwrap();
    cmd.arg("new")
        .arg("test-project")
        .arg("--path")
        .arg(&project_path)
        .arg("--no-git")
        .arg("--no-install");
    
    cmd.assert().success();
    
    // Check that files were created
    assert!(project_path.join("Cargo.toml").exists());
    assert!(project_path.join("src").exists());
    assert!(project_path.join("wit").exists());
    assert!(project_path.join("wasm-wizard.toml").exists());
    
    // Check Cargo.toml content
    let cargo_toml = fs::read_to_string(project_path.join("Cargo.toml")).unwrap();
    assert!(cargo_toml.contains("test-project"));
    assert!(cargo_toml.contains("cdylib"));
}

#[test]
fn test_new_command_with_different_templates() {
    let temp_dir = TempDir::new().unwrap();
    
    // Test JavaScript template
    let js_project_path = temp_dir.path().join("test-js-project");
    let mut cmd = Command::cargo_bin("wasm-wizard").unwrap();
    cmd.arg("new")
        .arg("test-js-project")
        .arg("--language")
        .arg("javascript")
        .arg("--path")
        .arg(&js_project_path)
        .arg("--no-git")
        .arg("--no-install");
    
    cmd.assert().success();
    
    // Check that JavaScript files were created
    assert!(js_project_path.join("package.json").exists());
    assert!(js_project_path.join("wit").exists());
    
    // Check package.json content
    let package_json = fs::read_to_string(js_project_path.join("package.json")).unwrap();
    assert!(package_json.contains("test-js-project"));
    assert!(package_json.contains("componentize-js"));
}

#[test]
fn test_analyze_command_with_nonexistent_file() {
    let mut cmd = Command::cargo_bin("wasm-wizard").unwrap();
    cmd.arg("analyze")
        .arg("nonexistent.wasm");
    
    cmd.assert().failure();
}

#[test]
fn test_install_command_list() {
    let mut cmd = Command::cargo_bin("wasm-wizard").unwrap();
    cmd.arg("install");
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Available tools"));
}