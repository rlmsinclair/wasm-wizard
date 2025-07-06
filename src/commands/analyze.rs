use super::Command;
use anyhow::Result;
use async_trait::async_trait;
use colored::*;

#[async_trait]
impl Command for super::AnalyzeCommand {
    async fn execute(&self) -> Result<()> {
        println!("{}", "📊 Analyzing WASM component".bright_green().bold());
        println!("  Component: {}", self.component.display());

        let metadata = std::fs::metadata(&self.component)?;
        let size = metadata.len();

        println!();
        println!("{}", "Analysis Results:".bright_cyan().bold());
        println!("  📏 Size: {} bytes ({:.2} KB)", size, size as f64 / 1024.0);

        // Analyze WASM structure
        if let Ok(wasm_bytes) = std::fs::read(&self.component) {
            let parser = wasmparser::Parser::new(0);
            let mut sections = Vec::new();

            for payload in parser.parse_all(&wasm_bytes) {
                match payload {
                    Ok(wasmparser::Payload::TypeSection(_)) => sections.push("Type"),
                    Ok(wasmparser::Payload::ImportSection(_)) => sections.push("Import"),
                    Ok(wasmparser::Payload::FunctionSection(_)) => sections.push("Function"),
                    Ok(wasmparser::Payload::TableSection(_)) => sections.push("Table"),
                    Ok(wasmparser::Payload::MemorySection(_)) => sections.push("Memory"),
                    Ok(wasmparser::Payload::GlobalSection(_)) => sections.push("Global"),
                    Ok(wasmparser::Payload::ExportSection(_)) => sections.push("Export"),
                    Ok(wasmparser::Payload::StartSection { .. }) => sections.push("Start"),
                    Ok(wasmparser::Payload::ElementSection(_)) => sections.push("Element"),
                    Ok(wasmparser::Payload::DataSection(_)) => sections.push("Data"),
                    Ok(wasmparser::Payload::CodeSectionStart { .. }) => sections.push("Code"),
                    _ => {}
                }
            }

            println!("  📦 Sections: {}", sections.join(", "));
        }

        Ok(())
    }
}
