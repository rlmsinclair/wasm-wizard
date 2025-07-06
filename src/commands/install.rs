use super::Command;
use crate::utils::run_command;
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use colored::*;
use std::collections::HashMap;

#[async_trait]
impl Command for super::InstallCommand {
    async fn execute(&self) -> Result<()> {
        let required_tools = get_required_tools();
        
        if self.all {
            println!("{}", "🛠️  Installing all required tools".bright_green().bold());
            for (tool, info) in &required_tools {
                println!("Installing {}...", tool.bright_cyan());
                install_tool(tool, info).await?;
            }
        } else if let Some(tool) = &self.tool {
            if let Some(info) = required_tools.get(tool) {
                println!("{} {}", "🛠️  Installing".bright_green().bold(), tool.bright_cyan());
                install_tool(tool, info).await?;
            } else {
                return Err(anyhow!("Unknown tool: {}", tool));
            }
        } else {
            println!("{}", "Available tools:".bright_cyan().bold());
            for (tool, info) in &required_tools {
                let status = if is_tool_installed(tool).await {
                    "✅ Installed".bright_green()
                } else {
                    "❌ Not installed".bright_red()
                };
                println!("  {} - {} {}", tool.bright_cyan(), info.description, status);
            }
        }
        
        Ok(())
    }
}

struct ToolInfo {
    description: &'static str,
    install_cmd: &'static str,
    check_cmd: &'static str,
}

fn get_required_tools() -> HashMap<String, ToolInfo> {
    let mut tools = HashMap::new();
    
    tools.insert("wasm-tools".to_string(), ToolInfo {
        description: "Core WebAssembly toolchain",
        install_cmd: "cargo install wasm-tools",
        check_cmd: "wasm-tools --version",
    });
    
    tools.insert("wasm-opt".to_string(), ToolInfo {
        description: "WebAssembly optimizer",
        install_cmd: "cargo install wasm-opt",
        check_cmd: "wasm-opt --version",
    });
    
    tools.insert("wit-bindgen".to_string(), ToolInfo {
        description: "WIT bindings generator",
        install_cmd: "cargo install wit-bindgen-cli",
        check_cmd: "wit-bindgen --version",
    });
    
    tools.insert("wasm-compose".to_string(), ToolInfo {
        description: "WebAssembly component composer",
        install_cmd: "cargo install wasm-compose",
        check_cmd: "wasm-compose --version",
    });
    
    tools
}

async fn install_tool(tool: &str, info: &ToolInfo) -> Result<()> {
    let parts: Vec<&str> = info.install_cmd.split_whitespace().collect();
    if parts.is_empty() {
        return Err(anyhow!("Invalid install command"));
    }
    
    run_command(parts[0], &parts[1..], None).await?;
    println!("✅ {} installed successfully", tool);
    Ok(())
}

async fn is_tool_installed(tool: &str) -> bool {
    which::which(tool).is_ok()
}