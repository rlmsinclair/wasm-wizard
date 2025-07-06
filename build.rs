use std::process::Command;

fn main() {
    // Add build-time information
    println!("cargo:rerun-if-changed=build.rs");
    
    // Get git commit hash
    if let Ok(output) = Command::new("git")
        .args(&["rev-parse", "--short", "HEAD"])
        .output()
    {
        let git_hash = String::from_utf8(output.stdout).unwrap_or_default();
        println!("cargo:rustc-env=GIT_HASH={}", git_hash.trim());
    }
    
    // Get build timestamp
    let timestamp = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC");
    println!("cargo:rustc-env=BUILD_TIMESTAMP={}", timestamp);
    
    // Get target triple
    let target = std::env::var("TARGET").unwrap_or_default();
    println!("cargo:rustc-env=TARGET_TRIPLE={}", target);
}