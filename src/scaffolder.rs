use anyhow::{anyhow, Result};
use handlebars::Handlebars;
use serde_json::json;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub struct Scaffolder {
    handlebars: Handlebars<'static>,
    templates: HashMap<String, TemplateInfo>,
}

struct TemplateInfo {
    #[allow(dead_code)]
    description: String,
    files: Vec<TemplateFile>,
}

struct TemplateFile {
    path: String,
    content: String,
    executable: bool,
}

impl Scaffolder {
    pub fn new() -> Self {
        let mut scaffolder = Self {
            handlebars: Handlebars::new(),
            templates: HashMap::new(),
        };

        scaffolder.load_templates();
        scaffolder
    }

    fn load_templates(&mut self) {
        // Basic Rust template
        let mut rust_basic = TemplateInfo {
            description: "Basic Rust WebAssembly component".to_string(),
            files: vec![],
        };

        rust_basic.files.push(TemplateFile {
            path: "Cargo.toml".to_string(),
            content: include_str!("../templates/rust/basic/Cargo.toml").to_string(),
            executable: false,
        });

        rust_basic.files.push(TemplateFile {
            path: "src/lib.rs".to_string(),
            content: include_str!("../templates/rust/basic/src/lib.rs").to_string(),
            executable: false,
        });

        rust_basic.files.push(TemplateFile {
            path: "wit/world.wit".to_string(),
            content: include_str!("../templates/rust/basic/wit/world.wit").to_string(),
            executable: false,
        });

        rust_basic.files.push(TemplateFile {
            path: "wasm-wizard.toml".to_string(),
            content: include_str!("../templates/rust/basic/wasm-wizard.toml").to_string(),
            executable: false,
        });

        rust_basic.files.push(TemplateFile {
            path: ".gitignore".to_string(),
            content: include_str!("../templates/rust/basic/.gitignore").to_string(),
            executable: false,
        });

        self.templates.insert("rust-basic".to_string(), rust_basic);

        // Crypto/Performance template
        let mut rust_crypto = TemplateInfo {
            description: "High-performance cryptographic WebAssembly component".to_string(),
            files: vec![],
        };

        rust_crypto.files.push(TemplateFile {
            path: "Cargo.toml".to_string(),
            content: include_str!("../templates/rust/crypto/Cargo.toml").to_string(),
            executable: false,
        });

        rust_crypto.files.push(TemplateFile {
            path: "src/lib.rs".to_string(),
            content: include_str!("../templates/rust/crypto/src/lib.rs").to_string(),
            executable: false,
        });

        rust_crypto.files.push(TemplateFile {
            path: "build.sh".to_string(),
            content: include_str!("../templates/rust/crypto/build.sh").to_string(),
            executable: true,
        });

        rust_crypto.files.push(TemplateFile {
            path: "demo.html".to_string(),
            content: include_str!("../templates/rust/crypto/demo.html").to_string(),
            executable: false,
        });

        rust_crypto.files.push(TemplateFile {
            path: "README.md".to_string(),
            content: include_str!("../templates/rust/crypto/README.md").to_string(),
            executable: false,
        });

        rust_crypto.files.push(TemplateFile {
            path: "wit/world.wit".to_string(),
            content: include_str!("../templates/rust/crypto/wit/world.wit").to_string(),
            executable: false,
        });

        rust_crypto.files.push(TemplateFile {
            path: "wasm-wizard.toml".to_string(),
            content: include_str!("../templates/rust/crypto/wasm-wizard.toml").to_string(),
            executable: false,
        });

        rust_crypto.files.push(TemplateFile {
            path: ".gitignore".to_string(),
            content: include_str!("../templates/rust/crypto/.gitignore").to_string(),
            executable: false,
        });

        self.templates
            .insert("rust-crypto".to_string(), rust_crypto);

        // Add more templates...
        self.load_javascript_templates();
        self.load_typescript_templates();
    }

    fn load_javascript_templates(&mut self) {
        let mut js_basic = TemplateInfo {
            description: "Basic JavaScript WebAssembly component".to_string(),
            files: vec![],
        };

        js_basic.files.push(TemplateFile {
            path: "package.json".to_string(),
            content: r#"{
  "name": "{{name}}",
  "version": "1.0.0",
  "description": "WebAssembly component created with wasm-wizard",
  "main": "dist/{{name}}.wasm",
  "scripts": {
    "build": "wasm-wizard build",
    "build:wasm": "componentize-js --wit wit --world-name {{name}} --out dist/{{name}}.wasm src/{{name}}.js",
    "dev": "wasm-wizard dev"
  },
  "devDependencies": {
    "@bytecodealliance/componentize-js": "^0.8.0"
  }
}"#.to_string(),
            executable: false,
        });

        js_basic.files.push(TemplateFile {
            path: "src/{{name}}.js".to_string(),
            content: r#"// {{name}} WebAssembly Component
// Generated by wasm-wizard

export const world = {
  greet(name) {
    return `Hello, ${name}! Welcome to WebAssembly Components! 🚀`;
  },
  
  calculate(a, b) {
    return a + b;
  }
};
"#
            .to_string(),
            executable: false,
        });

        js_basic.files.push(TemplateFile {
            path: "wit/world.wit".to_string(),
            content: r#"package {{name}}:component;

world {{name}} {
  export greet: func(name: string) -> string;
  export calculate: func(a: s32, b: s32) -> s32;
}
"#
            .to_string(),
            executable: false,
        });

        self.templates
            .insert("javascript-basic".to_string(), js_basic);
    }

    fn load_typescript_templates(&mut self) {
        // Similar to JavaScript but with TypeScript setup
        let mut ts_basic = TemplateInfo {
            description: "Basic TypeScript WebAssembly component".to_string(),
            files: vec![],
        };

        ts_basic.files.push(TemplateFile {
            path: "package.json".to_string(),
            content: r#"{
  "name": "{{name}}",
  "version": "1.0.0",
  "description": "WebAssembly component created with wasm-wizard",
  "main": "dist/{{name}}.wasm",
  "scripts": {
    "build": "tsc && wasm-wizard build",
    "build:wasm": "componentize-js --wit wit --world-name {{name}} --out dist/{{name}}.wasm dist/{{name}}.js",
    "dev": "wasm-wizard dev"
  },
  "devDependencies": {
    "@bytecodealliance/componentize-js": "^0.8.0",
    "typescript": "^5.0.0"
  }
}"#.to_string(),
            executable: false,
        });

        ts_basic.files.push(TemplateFile {
            path: "src/{{name}}.ts".to_string(),
            content: r#"// {{name}} WebAssembly Component
// Generated by wasm-wizard

export const world = {
  greet(name: string): string {
    return `Hello, ${name}! Welcome to WebAssembly Components! 🚀`;
  },
  
  calculate(a: number, b: number): number {
    return a + b;
  }
};
"#
            .to_string(),
            executable: false,
        });

        self.templates
            .insert("typescript-basic".to_string(), ts_basic);
    }

    pub async fn create_project(
        &self,
        name: &str,
        template: &str,
        language: &str,
        target_path: &Path,
        init_git: bool,
        install_deps: bool,
    ) -> Result<()> {
        // Handle special minimal template
        if template == "minimal" {
            return self.create_minimal_project(name, target_path, init_git).await;
        }

        let template_key = format!("{language}-{template}");

        let template_info = self.templates.get(&template_key).ok_or_else(|| {
            anyhow!(
                "Template '{}' not found for language '{}'",
                template,
                language
            )
        })?;

        // Create target directory
        fs::create_dir_all(target_path)?;

        // Create template context
        let context = json!({
            "name": name,
            "template": template,
            "language": language,
            "year": chrono::Utc::now().format("%Y").to_string(),
        });

        // Generate files
        for file in &template_info.files {
            let file_path = self.handlebars.render_template(&file.path, &context)?;
            let file_content = self.handlebars.render_template(&file.content, &context)?;

            let full_path = target_path.join(&file_path);

            // Create parent directories
            if let Some(parent) = full_path.parent() {
                fs::create_dir_all(parent)?;
            }

            fs::write(&full_path, file_content)?;

            if file.executable {
                #[cfg(unix)]
                {
                    use std::os::unix::fs::PermissionsExt;
                    let mut perms = fs::metadata(&full_path)?.permissions();
                    perms.set_mode(0o755);
                    fs::set_permissions(&full_path, perms)?;
                }
            }
        }

        // Initialize git repository
        if init_git {
            self.init_git(target_path).await?;
        }

        // Install dependencies
        if install_deps {
            self.install_dependencies(target_path, language).await?;
        }

        Ok(())
    }

    async fn init_git(&self, path: &Path) -> Result<()> {
        use crate::utils::run_command;

        run_command("git", &["init"], Some(path)).await?;
        run_command("git", &["add", "."], Some(path)).await?;
        run_command(
            "git",
            &["commit", "-m", "Initial commit from wasm-wizard"],
            Some(path),
        )
        .await?;

        Ok(())
    }

    async fn install_dependencies(&self, path: &Path, language: &str) -> Result<()> {
        use crate::utils::run_command;

        match language {
            "rust" => {
                // Add wasm32-wasip1 target if not present
                let _ =
                    run_command("rustup", &["target", "add", "wasm32-wasip1"], Some(path)).await;
            }
            "javascript" | "typescript" => {
                run_command("npm", &["install"], Some(path)).await?;
            }
            _ => {}
        }

        Ok(())
    }

    async fn create_minimal_project(&self, name: &str, target_path: &Path, init_git: bool) -> Result<()> {
        // Create target directory
        fs::create_dir_all(target_path)?;

        // Create the minimal demo HTML file
        let demo_content = include_str!("../templates/minimal/demo.html")
            .replace("{{name}}", name);

        fs::write(target_path.join("demo.html"), demo_content)?;

        // Create build.sh
        let build_content = include_str!("../templates/minimal/build.sh");
        let build_path = target_path.join("build.sh");
        fs::write(&build_path, build_content)?;

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(&build_path)?.permissions();
            perms.set_mode(0o755);
            fs::set_permissions(&build_path, perms)?;
        }

        // Create wasm-wizard.toml
        let toml_content = include_str!("../templates/minimal/wasm-wizard.toml")
            .replace("{{name}}", name);
        fs::write(target_path.join("wasm-wizard.toml"), toml_content)?;

        // Create .gitignore
        fs::write(target_path.join(".gitignore"), "*.pyc\n__pycache__/\n")?;

        // Initialize git repository if requested
        if init_git {
            self.init_git(target_path).await?;
        }

        Ok(())
    }
}
