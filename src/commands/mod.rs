use anyhow::Result;
use clap::Args;
use std::path::PathBuf;

pub mod add;
pub mod analyze;
pub mod bindings;
pub mod build;
pub mod check;
pub mod compose;
pub mod dev;
pub mod install;
pub mod new;
pub mod optimize;

// Command implementations are in separate modules

#[async_trait::async_trait]
pub trait Command {
    async fn execute(&self) -> Result<()>;
}

#[derive(Args)]
pub struct NewCommand {
    /// Project name
    #[arg(value_name = "NAME")]
    pub name: String,

    /// Template to use
    #[arg(short, long, default_value = "basic")]
    pub template: String,

    /// Programming language
    #[arg(short, long, default_value = "rust")]
    pub language: String,

    /// Target directory
    #[arg(short, long)]
    pub path: Option<PathBuf>,

    /// Skip git initialization
    #[arg(long)]
    pub no_git: bool,

    /// Skip dependency installation
    #[arg(long)]
    pub no_install: bool,
}

#[derive(Args)]
pub struct BuildCommand {
    /// Build target (debug/release)
    #[arg(short, long, default_value = "release")]
    pub target: String,

    /// Enable optimization
    #[arg(short, long)]
    pub optimize: bool,

    /// Output directory
    #[arg(short = 'O', long)]
    pub output: Option<PathBuf>,

    /// Watch for changes
    #[arg(short, long)]
    pub watch: bool,

    /// Enable verbose output
    #[arg(short, long)]
    pub verbose: bool,
}

#[derive(Args)]
pub struct ComposeCommand {
    /// Component files to compose
    #[arg(value_name = "COMPONENTS", required = true)]
    pub components: Vec<PathBuf>,

    /// Output file
    #[arg(short, long, default_value = "composed.wasm")]
    pub output: PathBuf,

    /// Composition configuration file
    #[arg(short, long)]
    pub config: Option<PathBuf>,

    /// Enable optimization after composition
    #[arg(long)]
    pub optimize: bool,
}

#[derive(Args)]
pub struct OptimizeCommand {
    /// WASM file to optimize
    #[arg(value_name = "FILE")]
    pub file: PathBuf,

    /// Output file
    #[arg(short, long)]
    pub output: Option<PathBuf>,

    /// Optimization level (0-4)
    #[arg(short, long, default_value = "3")]
    pub level: u8,

    /// Target size in bytes
    #[arg(short, long)]
    pub size: Option<usize>,

    /// Enable experimental optimizations
    #[arg(long)]
    pub experimental: bool,
}

#[derive(Args)]
pub struct InstallCommand {
    /// Tool to install
    #[arg(value_name = "TOOL")]
    pub tool: Option<String>,

    /// Install all required tools
    #[arg(long)]
    pub all: bool,

    /// Force reinstallation
    #[arg(long)]
    pub force: bool,
}

#[derive(Args)]
pub struct CheckCommand {
    /// Check specific component
    #[arg(value_name = "COMPONENT")]
    pub component: Option<PathBuf>,

    /// Check all components in project
    #[arg(long)]
    pub all: bool,

    /// Fix issues automatically
    #[arg(long)]
    pub fix: bool,
}

#[derive(Args)]
pub struct AddCommand {
    /// Component type to add
    #[arg(value_name = "TYPE")]
    pub component_type: String,

    /// Component name
    #[arg(value_name = "NAME")]
    pub name: String,

    /// Template to use
    #[arg(short, long)]
    pub template: Option<String>,
}

#[derive(Args)]
pub struct DevCommand {
    /// Port to run on
    #[arg(short, long, default_value = "8080")]
    pub port: u16,

    /// Host to bind to
    #[arg(long, default_value = "localhost")]
    pub host: String,

    /// Watch for changes
    #[arg(short, long)]
    pub watch: bool,

    /// Enable hot reload
    #[arg(long)]
    pub hot_reload: bool,
}

#[derive(Args)]
pub struct BindingsCommand {
    /// Component to generate bindings for
    #[arg(value_name = "COMPONENT")]
    pub component: PathBuf,

    /// Target language
    #[arg(short, long, default_value = "javascript")]
    pub language: String,

    /// Output directory
    #[arg(short, long)]
    pub output: Option<PathBuf>,
}

#[derive(Args)]
pub struct AnalyzeCommand {
    /// Component to analyze
    #[arg(value_name = "COMPONENT")]
    pub component: PathBuf,

    /// Show detailed analysis
    #[arg(short, long)]
    pub detailed: bool,

    /// Output format (text/json)
    #[arg(short, long, default_value = "text")]
    pub format: String,
}
