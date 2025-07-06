use super::Command;
use crate::optimizer::Optimizer;
use anyhow::Result;
use async_trait::async_trait;
use colored::*;

#[async_trait]
impl Command for super::OptimizeCommand {
    async fn execute(&self) -> Result<()> {
        let output_file = self.output.clone().unwrap_or_else(|| {
            let mut path = self.file.clone();
            path.set_extension("optimized.wasm");
            path
        });

        println!("{}", "⚡ Optimizing WASM component".bright_green().bold());
        println!("  Input: {}", self.file.display());
        println!("  Output: {}", output_file.display());
        println!("  Level: {}", self.level);
        println!();

        let optimizer = Optimizer::new();
        optimizer
            .optimize_file(&self.file, &output_file, self.level)
            .await?;

        // Show optimization results
        let original_size = std::fs::metadata(&self.file)?.len();
        let optimized_size = std::fs::metadata(&output_file)?.len();
        let reduction = ((original_size - optimized_size) as f64 / original_size as f64) * 100.0;

        println!("{}", "Optimization Results:".bright_cyan().bold());
        println!("  Original: {} bytes", original_size);
        println!("  Optimized: {} bytes", optimized_size);
        println!("  Reduction: {:.1}%", reduction);

        Ok(())
    }
}
