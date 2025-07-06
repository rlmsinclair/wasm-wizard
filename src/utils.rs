use anyhow::{anyhow, Result};
use std::path::{Path, PathBuf};
use std::process::Command;

pub async fn run_command(cmd: &str, args: &[&str], cwd: Option<&Path>) -> Result<()> {
    let mut command = Command::new(cmd);
    command.args(args);

    if let Some(dir) = cwd {
        command.current_dir(dir);
    }

    let output = command.output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow!("Command '{}' failed: {}", cmd, stderr));
    }

    Ok(())
}

pub fn find_project_root() -> Result<PathBuf> {
    let current_dir = std::env::current_dir()?;
    let mut path = current_dir.as_path();

    loop {
        let wasm_wizard_config = path.join("wasm-wizard.toml");
        let cargo_toml = path.join("Cargo.toml");
        let package_json = path.join("package.json");

        if wasm_wizard_config.exists() || cargo_toml.exists() || package_json.exists() {
            return Ok(path.to_path_buf());
        }

        match path.parent() {
            Some(parent) => path = parent,
            None => break,
        }
    }

    Err(anyhow!("Could not find project root"))
}

#[allow(dead_code)]
pub fn is_wasm_file(path: &Path) -> bool {
    path.extension().is_some_and(|ext| ext == "wasm")
}

#[allow(dead_code)]
pub fn format_size(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB"];
    let mut size = bytes as f64;
    let mut unit = 0;

    while size >= 1024.0 && unit < UNITS.len() - 1 {
        size /= 1024.0;
        unit += 1;
    }

    if unit == 0 {
        format!("{} {}", bytes, UNITS[unit])
    } else {
        format!("{:.2} {}", size, UNITS[unit])
    }
}

#[allow(dead_code)]
pub fn get_wasm_info(path: &Path) -> Result<WasmInfo> {
    let wasm_bytes = std::fs::read(path)?;
    let parser = wasmparser::Parser::new(0);

    let mut info = WasmInfo {
        size: wasm_bytes.len(),
        imports: 0,
        exports: 0,
        functions: 0,
        sections: Vec::new(),
    };

    for payload in parser.parse_all(&wasm_bytes) {
        match payload {
            Ok(wasmparser::Payload::ImportSection(reader)) => {
                info.imports = reader.count() as usize;
                info.sections.push("Import".to_string());
            }
            Ok(wasmparser::Payload::ExportSection(reader)) => {
                info.exports = reader.count() as usize;
                info.sections.push("Export".to_string());
            }
            Ok(wasmparser::Payload::FunctionSection(reader)) => {
                info.functions = reader.count() as usize;
                info.sections.push("Function".to_string());
            }
            Ok(wasmparser::Payload::TypeSection(_)) => {
                info.sections.push("Type".to_string());
            }
            Ok(wasmparser::Payload::MemorySection(_)) => {
                info.sections.push("Memory".to_string());
            }
            Ok(wasmparser::Payload::GlobalSection(_)) => {
                info.sections.push("Global".to_string());
            }
            Ok(wasmparser::Payload::TableSection(_)) => {
                info.sections.push("Table".to_string());
            }
            Ok(wasmparser::Payload::ElementSection(_)) => {
                info.sections.push("Element".to_string());
            }
            Ok(wasmparser::Payload::DataSection(_)) => {
                info.sections.push("Data".to_string());
            }
            Ok(wasmparser::Payload::CodeSectionStart { .. }) => {
                info.sections.push("Code".to_string());
            }
            Ok(wasmparser::Payload::CustomSection(reader)) => {
                info.sections.push(format!("Custom({})", reader.name()));
            }
            _ => {}
        }
    }

    Ok(info)
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct WasmInfo {
    pub size: usize,
    pub imports: usize,
    pub exports: usize,
    pub functions: usize,
    pub sections: Vec<String>,
}

#[allow(dead_code)]
pub fn create_progress_bar(len: u64, message: &str) -> indicatif::ProgressBar {
    use indicatif::{ProgressBar, ProgressStyle};

    let pb = ProgressBar::new(len);
    pb.set_style(
        ProgressStyle::default_bar()
            .template(
                "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos:>7}/{len:7} {msg}",
            )
            .unwrap()
            .progress_chars("#>-"),
    );
    pb.set_message(message.to_string());
    pb
}

#[allow(dead_code)]
pub fn validate_component_name(name: &str) -> Result<()> {
    if name.is_empty() {
        return Err(anyhow!("Component name cannot be empty"));
    }

    if !name
        .chars()
        .all(|c| c.is_alphanumeric() || c == '-' || c == '_')
    {
        return Err(anyhow!(
            "Component name can only contain alphanumeric characters, hyphens, and underscores"
        ));
    }

    if name.starts_with('-') || name.ends_with('-') {
        return Err(anyhow!("Component name cannot start or end with a hyphen"));
    }

    Ok(())
}
