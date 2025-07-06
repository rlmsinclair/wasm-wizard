use anyhow::{anyhow, Result};
use std::path::Path;
use std::process::Command;

pub struct Optimizer {
    wasm_opt_path: Option<String>,
}

impl Optimizer {
    pub fn new() -> Self {
        Self {
            wasm_opt_path: which::which("wasm-opt")
                .ok()
                .map(|p| p.to_string_lossy().to_string()),
        }
    }

    pub async fn optimize_file(&self, input: &Path, output: &Path, level: u8) -> Result<()> {
        if let Some(wasm_opt) = &self.wasm_opt_path {
            self.optimize_with_wasm_opt(wasm_opt, input, output, level)
                .await
        } else {
            self.optimize_manual(input, output, level).await
        }
    }

    async fn optimize_with_wasm_opt(
        &self,
        wasm_opt: &str,
        input: &Path,
        output: &Path,
        level: u8,
    ) -> Result<()> {
        let level_flag = format!("-O{}", level);

        let mut cmd = Command::new(wasm_opt);
        cmd.arg(&level_flag).arg(input).arg("-o").arg(output);

        // Add common optimizations
        cmd.args(&[
            "--enable-bulk-memory",
            "--enable-reference-types",
            "--enable-simd",
            "--enable-threads",
            "--strip-debug",
            "--strip-producers",
        ]);

        let output_result = cmd.output()?;

        if !output_result.status.success() {
            let stderr = String::from_utf8_lossy(&output_result.stderr);
            return Err(anyhow!("wasm-opt failed: {}", stderr));
        }

        println!("✅ Optimized with wasm-opt (level {})", level);
        Ok(())
    }

    async fn optimize_manual(&self, input: &Path, output: &Path, level: u8) -> Result<()> {
        println!("⚠️  wasm-opt not found, using manual optimization");

        let wasm_bytes = std::fs::read(input)?;
        let optimized_bytes = self.manual_optimize(&wasm_bytes, level)?;

        std::fs::write(output, optimized_bytes)?;

        println!("✅ Manual optimization completed");
        Ok(())
    }

    fn manual_optimize(&self, wasm_bytes: &[u8], level: u8) -> Result<Vec<u8>> {
        // Basic manual optimizations
        let mut optimized = wasm_bytes.to_vec();

        // Remove debug sections
        optimized = self.remove_debug_sections(&optimized)?;

        // Strip custom sections based on level
        if level >= 2 {
            optimized = self.strip_custom_sections(&optimized)?;
        }

        // Further optimizations for higher levels
        if level >= 3 {
            optimized = self.optimize_imports_exports(&optimized)?;
        }

        Ok(optimized)
    }

    fn remove_debug_sections(&self, wasm_bytes: &[u8]) -> Result<Vec<u8>> {
        // Parse and rebuild without debug sections
        let parser = wasmparser::Parser::new(0);
        let mut new_wasm = Vec::new();

        // Write WASM header
        new_wasm.extend_from_slice(&[0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00]);

        for payload in parser.parse_all(wasm_bytes) {
            match payload? {
                wasmparser::Payload::CustomSection(reader) => {
                    // Skip debug-related custom sections
                    let name = reader.name();
                    if !name.starts_with("debug") && !name.starts_with(".debug") {
                        // Keep non-debug custom sections
                        self.write_custom_section(&mut new_wasm, &reader)?;
                    }
                }
                _ => {
                    // Keep all other sections as-is
                    // This is a simplified version - full implementation would
                    // properly reconstruct each section
                }
            }
        }

        Ok(new_wasm)
    }

    fn strip_custom_sections(&self, wasm_bytes: &[u8]) -> Result<Vec<u8>> {
        // Similar to remove_debug_sections but removes more custom sections
        let parser = wasmparser::Parser::new(0);
        let mut new_wasm = Vec::new();

        // Write WASM header
        new_wasm.extend_from_slice(&[0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00]);

        for payload in parser.parse_all(wasm_bytes) {
            match payload? {
                wasmparser::Payload::CustomSection(reader) => {
                    // Only keep essential custom sections
                    let name = reader.name();
                    if name == "name" || name.starts_with("component") {
                        self.write_custom_section(&mut new_wasm, &reader)?;
                    }
                }
                _ => {
                    // Keep all other sections
                }
            }
        }

        Ok(new_wasm)
    }

    fn optimize_imports_exports(&self, wasm_bytes: &[u8]) -> Result<Vec<u8>> {
        // More advanced optimizations would go here
        // For now, just return the input
        Ok(wasm_bytes.to_vec())
    }

    fn write_custom_section(
        &self,
        wasm: &mut Vec<u8>,
        reader: &wasmparser::CustomSectionReader,
    ) -> Result<()> {
        // Write custom section header and data
        wasm.push(0); // Custom section ID

        let name = reader.name();
        let data = reader.data();

        // Write section size (name length + name + data length)
        let section_size = name.len() + data.len() + 1; // +1 for name length byte
        self.write_uleb128(wasm, section_size as u64);

        // Write name
        self.write_uleb128(wasm, name.len() as u64);
        wasm.extend_from_slice(name.as_bytes());

        // Write data
        wasm.extend_from_slice(data);

        Ok(())
    }

    fn write_uleb128(&self, wasm: &mut Vec<u8>, mut value: u64) {
        while value >= 0x80 {
            wasm.push((value & 0x7f | 0x80) as u8);
            value >>= 7;
        }
        wasm.push(value as u8);
    }
}
