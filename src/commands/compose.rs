use super::Command;
use crate::composer::Composer;
use crate::optimizer::Optimizer;
use anyhow::Result;
use async_trait::async_trait;
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;

#[async_trait]
impl Command for super::ComposeCommand {
    async fn execute(&self) -> Result<()> {
        println!("{}", "🔗 Composing WASM components".bright_green().bold());

        // Show components being composed
        println!("{}", "Components:".bright_cyan());
        for (i, component) in self.components.iter().enumerate() {
            println!("  {}. {}", i + 1, component.display());
        }
        println!();

        let pb = ProgressBar::new_spinner();
        pb.set_style(
            ProgressStyle::default_spinner()
                .template("{spinner:.green} {msg}")
                .unwrap(),
        );
        pb.enable_steady_tick(Duration::from_millis(100));

        // Compose components
        pb.set_message("Analyzing component interfaces...");
        let composer = Composer::new();

        pb.set_message("Composing components...");
        composer
            .compose_components(
                &self.components,
                &self.output,
                self.config.as_ref().map(|p| p.as_path()),
            )
            .await?;

        // Optimize if requested
        if self.optimize {
            pb.set_message("Optimizing composed component...");
            let optimizer = Optimizer::new();
            let optimized_output = self.output.with_extension("optimized.wasm");
            optimizer
                .optimize_file(&self.output, &optimized_output, 3)
                .await?;

            // Replace original with optimized
            std::fs::rename(&optimized_output, &self.output)?;
        }

        pb.finish_with_message("✅ Composition completed successfully!");

        // Show composition summary
        println!();
        println!("{}", "Composition Summary:".bright_cyan().bold());
        self.show_composition_summary().await?;

        Ok(())
    }
}

impl super::ComposeCommand {
    async fn show_composition_summary(&self) -> Result<()> {
        let metadata = std::fs::metadata(&self.output)?;
        let size = metadata.len();

        println!("  📦 Output: {}", self.output.display());
        println!("  📏 Size: {} bytes ({:.2} KB)", size, size as f64 / 1024.0);
        println!("  🔗 Components: {}", self.components.len());

        // Analyze the composed component
        if let Ok(wasm_bytes) = std::fs::read(&self.output) {
            let parser = wasmparser::Parser::new(0);
            let mut import_count = 0;
            let mut export_count = 0;

            for payload in parser.parse_all(&wasm_bytes) {
                match payload {
                    Ok(wasmparser::Payload::ImportSection(reader)) => {
                        import_count = reader.count();
                    }
                    Ok(wasmparser::Payload::ExportSection(reader)) => {
                        export_count = reader.count();
                    }
                    _ => {}
                }
            }

            println!("  📥 Imports: {}", import_count);
            println!("  📤 Exports: {}", export_count);
        }

        Ok(())
    }
}
