use super::Command;
use crate::scaffolder::Scaffolder;
use anyhow::Result;
use async_trait::async_trait;
use colored::*;
use std::path::PathBuf;

#[async_trait]
impl Command for super::NewCommand {
    async fn execute(&self) -> Result<()> {
        let target_path = self
            .path
            .clone()
            .unwrap_or_else(|| PathBuf::from(&self.name));

        println!(
            "{} {}",
            "🏗️  Creating new project".bright_green().bold(),
            self.name.bright_cyan()
        );
        println!("{} {}", "   Template:".bright_blue(), self.template);
        println!("{} {}", "   Language:".bright_blue(), self.language);
        println!("{} {}", "   Path:".bright_blue(), target_path.display());
        println!();

        let scaffolder = Scaffolder::new();
        scaffolder
            .create_project(
                &self.name,
                &self.template,
                &self.language,
                &target_path,
                !self.no_git,
                !self.no_install,
            )
            .await?;

        println!();
        println!(
            "{}",
            "🎉 Project created successfully!".bright_green().bold()
        );
        println!();
        println!("{}", "Next steps:".bright_cyan().bold());
        println!("  cd {}", target_path.display());
        println!("  wasm-wizard build");
        println!("  wasm-wizard dev");
        println!();
        println!("{}", "Happy coding! 🚀".bright_magenta());

        Ok(())
    }
}
