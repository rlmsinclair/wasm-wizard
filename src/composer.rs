use anyhow::{anyhow, Result};
use std::path::{Path, PathBuf};
use std::process::Command;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CompositionConfig {
    pub components: Vec<ComponentConfig>,
    pub output: OutputConfig,
    pub linking: LinkingConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ComponentConfig {
    pub name: String,
    pub path: PathBuf,
    pub exports: Vec<String>,
    pub imports: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OutputConfig {
    pub name: String,
    pub format: String, // "component" or "module"
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LinkingConfig {
    pub strategy: String, // "auto", "manual"
    pub connections: Vec<Connection>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Connection {
    pub from: String,
    pub to: String,
    pub export: String,
    pub import: String,
}

pub struct Composer {
    wasm_compose_path: Option<String>,
}

impl Composer {
    pub fn new() -> Self {
        Self {
            wasm_compose_path: which::which("wasm-compose").ok().map(|p| p.to_string_lossy().to_string()),
        }
    }
    
    pub async fn compose_components(
        &self,
        components: &[PathBuf],
        output: &Path,
        config: Option<&Path>,
    ) -> Result<()> {
        if let Some(wasm_compose) = &self.wasm_compose_path {
            self.compose_with_wasm_compose(wasm_compose, components, output, config).await
        } else {
            self.compose_manual(components, output, config).await
        }
    }
    
    async fn compose_with_wasm_compose(
        &self,
        wasm_compose: &str,
        components: &[PathBuf],
        output: &Path,
        config: Option<&Path>,
    ) -> Result<()> {
        let mut cmd = Command::new(wasm_compose);
        
        if let Some(config_path) = config {
            cmd.arg("-c").arg(config_path);
        }
        
        cmd.arg("-o").arg(output);
        
        for component in components {
            cmd.arg(component);
        }
        
        let output_result = cmd.output()?;
        
        if !output_result.status.success() {
            let stderr = String::from_utf8_lossy(&output_result.stderr);
            return Err(anyhow!("wasm-compose failed: {}", stderr));
        }
        
        println!("✅ Composed with wasm-compose");
        Ok(())
    }
    
    async fn compose_manual(
        &self,
        components: &[PathBuf],
        output: &Path,
        config: Option<&Path>,
    ) -> Result<()> {
        println!("⚠️  wasm-compose not found, using manual composition");
        
        // Load composition config if provided
        let config = if let Some(config_path) = config {
            let config_content = std::fs::read_to_string(config_path)?;
            toml::from_str::<CompositionConfig>(&config_content)?
        } else {
            self.generate_default_config(components)?
        };
        
        // Analyze components
        let mut analyzed_components = Vec::new();
        for component_path in components {
            let analysis = self.analyze_component(component_path)?;
            analyzed_components.push(analysis);
        }
        
        // Perform composition
        let composed_bytes = self.manual_compose(&analyzed_components, &config)?;
        
        // Write output
        std::fs::write(output, composed_bytes)?;
        
        println!("✅ Manual composition completed");
        Ok(())
    }
    
    fn generate_default_config(&self, components: &[PathBuf]) -> Result<CompositionConfig> {
        let mut config = CompositionConfig {
            components: Vec::new(),
            output: OutputConfig {
                name: "composed".to_string(),
                format: "component".to_string(),
            },
            linking: LinkingConfig {
                strategy: "auto".to_string(),
                connections: Vec::new(),
            },
        };
        
        for component_path in components.iter() {
            let component_name = component_path
                .file_stem()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string();
            
            config.components.push(ComponentConfig {
                name: component_name,
                path: component_path.clone(),
                exports: Vec::new(),
                imports: Vec::new(),
            });
        }
        
        Ok(config)
    }
    
    fn analyze_component(&self, path: &Path) -> Result<ComponentAnalysis> {
        let wasm_bytes = std::fs::read(path)?;
        let parser = wasmparser::Parser::new(0);
        
        let mut analysis = ComponentAnalysis {
            path: path.to_path_buf(),
            imports: Vec::new(),
            exports: Vec::new(),
            functions: Vec::new(),
        };
        
        for payload in parser.parse_all(&wasm_bytes) {
            match payload {
                Ok(wasmparser::Payload::ImportSection(reader)) => {
                    for import in reader {
                        if let Ok(import) = import {
                            analysis.imports.push(ImportInfo {
                                module: import.module.to_string(),
                                name: import.name.to_string(),
                                ty: format!("{:?}", import.ty),
                            });
                        }
                    }
                }
                Ok(wasmparser::Payload::ExportSection(reader)) => {
                    for export in reader {
                        if let Ok(export) = export {
                            analysis.exports.push(ExportInfo {
                                name: export.name.to_string(),
                                ty: format!("{:?}", export.kind),
                                index: export.index,
                            });
                        }
                    }
                }
                _ => {}
            }
        }
        
        Ok(analysis)
    }
    
    fn manual_compose(&self, components: &[ComponentAnalysis], _config: &CompositionConfig) -> Result<Vec<u8>> {
        // This is a simplified manual composition
        // In a real implementation, this would be much more complex
        
        if components.is_empty() {
            return Err(anyhow!("No components to compose"));
        }
        
        // For now, just return the first component as the composed result
        // A full implementation would:
        // 1. Analyze import/export compatibility
        // 2. Resolve dependencies
        // 3. Merge sections
        // 4. Update indices
        // 5. Generate new import/export sections
        
        let first_component = std::fs::read(&components[0].path)?;
        
        println!("📝 Manual composition is simplified - using first component as base");
        println!("   For full composition features, install wasm-compose");
        
        Ok(first_component)
    }
}

#[derive(Debug)]
struct ComponentAnalysis {
    path: PathBuf,
    imports: Vec<ImportInfo>,
    exports: Vec<ExportInfo>,
    functions: Vec<FunctionInfo>,
}

#[derive(Debug)]
struct ImportInfo {
    module: String,
    name: String,
    ty: String,
}

#[derive(Debug)]
struct ExportInfo {
    name: String,
    ty: String,
    index: u32,
}

#[derive(Debug)]
struct FunctionInfo {
    name: String,
    signature: String,
}