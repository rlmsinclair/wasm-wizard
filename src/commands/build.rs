use super::Command;
use crate::optimizer::Optimizer;
use crate::utils::{find_project_root, run_command};
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use std::path::{Path, PathBuf};
use std::time::Duration;

#[async_trait]
impl Command for super::BuildCommand {
    async fn execute(&self) -> Result<()> {
        let project_root = find_project_root()?;
        let config_path = project_root.join("wasm-wizard.toml");

        if !config_path.exists() {
            return Err(anyhow!(
                "Not in a wasm-wizard project. Run 'wasm-wizard new' to create one."
            ));
        }

        println!(
            "{} {}",
            "🔨 Building project".bright_green().bold(),
            self.target.bright_cyan()
        );

        let pb = ProgressBar::new_spinner();
        pb.set_style(
            ProgressStyle::default_spinner()
                .template("{spinner:.green} {msg}")
                .unwrap(),
        );
        pb.enable_steady_tick(Duration::from_millis(100));

        // Build the project
        pb.set_message("Building WASM component...");
        self.build_component(&project_root).await?;

        if self.optimize {
            pb.set_message("Optimizing WASM binary...");
            self.optimize_component(&project_root).await?;
        }

        pb.finish_with_message("✅ Build completed successfully!");

        println!();
        println!("{}", "Build Summary:".bright_cyan().bold());
        self.show_build_summary(&project_root).await?;

        Ok(())
    }
}

impl super::BuildCommand {
    async fn build_component(&self, project_root: &Path) -> Result<()> {
        let cargo_toml = project_root.join("Cargo.toml");

        if cargo_toml.exists() {
            // Rust project
            let mut args = vec!["build"];
            if self.target == "release" {
                args.push("--release");
            }
            args.extend(["--target", "wasm32-wasip1"]);

            run_command("cargo", &args, Some(project_root)).await?;
        } else {
            // Check for other language support
            let package_json = project_root.join("package.json");
            if package_json.exists() {
                // JavaScript/TypeScript project
                run_command("npm", &["run", "build:wasm"], Some(project_root)).await?;
            } else {
                return Err(anyhow!("Unsupported project type"));
            }
        }

        Ok(())
    }

    async fn optimize_component(&self, project_root: &Path) -> Result<()> {
        let wasm_file = self.find_wasm_output(project_root)?;
        let optimizer = Optimizer::new();

        let optimized_path = if let Some(output) = &self.output {
            output.clone()
        } else {
            wasm_file.with_extension("optimized.wasm")
        };

        optimizer
            .optimize_file(&wasm_file, &optimized_path, 3)
            .await?;
        Ok(())
    }

    fn find_wasm_output(&self, project_root: &Path) -> Result<PathBuf> {
        let target_dir = project_root.join("target/wasm32-wasip1");
        let build_dir = if self.target == "release" {
            target_dir.join("release")
        } else {
            target_dir.join("debug")
        };

        // Find the .wasm file
        let entries = std::fs::read_dir(&build_dir)?;
        for entry in entries {
            let entry = entry?;
            let path = entry.path();
            if path.extension().is_some_and(|ext| ext == "wasm") {
                return Ok(path);
            }
        }

        Err(anyhow!("No WASM output found in {}", build_dir.display()))
    }

    async fn show_build_summary(&self, project_root: &Path) -> Result<()> {
        if let Ok(wasm_file) = self.find_wasm_output(project_root) {
            let metadata = std::fs::metadata(&wasm_file)?;
            let size = metadata.len();

            println!("  📦 Output: {}", wasm_file.display());
            println!("  📏 Size: {} bytes ({:.2} KB)", size, size as f64 / 1024.0);

            // Show optimization potential
            if !self.optimize {
                println!("  💡 Tip: Use --optimize to reduce size further");
            }
        }

        Ok(())
    }
}
