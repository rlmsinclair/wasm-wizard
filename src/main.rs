use anyhow::Result;
use clap::{Parser, Subcommand};
use colored::*;

mod commands;
mod composer;
mod optimizer;
mod scaffolder;
mod utils;

use commands::*;

#[derive(Parser)]
#[command(
    name = "wasm-wizard",
    version,
    about = "A CLI tool that makes WebAssembly Component Model ridiculously easy to use 🧙‍♂️",
    long_about = "WASM Wizard simplifies WebAssembly Component Model development with:\n\
                  • Project scaffolding with battle-tested templates\n\
                  • Smart build optimization and size reduction\n\
                  • Component composition and linking\n\
                  • Developer-friendly tooling and debugging"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new WASM component project
    #[command(alias = "init")]
    New(NewCommand),
    
    /// Build and optimize WASM components
    Build(BuildCommand),
    
    /// Compose multiple components into a single component
    Compose(ComposeCommand),
    
    /// Optimize existing WASM components
    Optimize(OptimizeCommand),
    
    /// Install and manage toolchain dependencies
    Install(InstallCommand),
    
    /// Check component health and dependencies
    Check(CheckCommand),
    
    /// Add components to existing project
    Add(AddCommand),
    
    /// Run development server with hot reload
    Dev(DevCommand),
    
    /// Generate bindings for different languages
    Bindings(BindingsCommand),
    
    /// Analyze component size and performance
    Analyze(AnalyzeCommand),
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Show wizard banner
    println!("{}", "🧙‍♂️ WASM Wizard".bright_magenta().bold());
    println!("{}", "Making WebAssembly Component Model ridiculously easy!".bright_cyan());
    println!();

    match cli.command {
        Commands::New(cmd) => cmd.execute().await,
        Commands::Build(cmd) => cmd.execute().await,
        Commands::Compose(cmd) => cmd.execute().await,
        Commands::Optimize(cmd) => cmd.execute().await,
        Commands::Install(cmd) => cmd.execute().await,
        Commands::Check(cmd) => cmd.execute().await,
        Commands::Add(cmd) => cmd.execute().await,
        Commands::Dev(cmd) => cmd.execute().await,
        Commands::Bindings(cmd) => cmd.execute().await,
        Commands::Analyze(cmd) => cmd.execute().await,
    }
}