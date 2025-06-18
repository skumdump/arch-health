use std::process::Command;
use which::which;

pub fn run_check(format: &str) {
    if which("arch-audit").is_err() {
        if format == "text" {
            println!("⚠️  [audit] arch-audit not found in PATH");
        }
        return;
    }

    let output = Command::new("arch-audit")
        .arg("--vulnerable")
        .output();

    match output {
        Ok(result) => {
            if format == "text" {
                if result.stdout.is_empty() {
                    println!("✅ [audit] No known vulnerabilities");
                } else {
                    println!("⚠️  [audit] Vulnerabilities found:\n{}", String::from_utf8_lossy(&result.stdout));
                }
            }
        },
        Err(err) => {
            if format == "text" {
                println!("❌ [audit] Failed to run arch-audit: {}", err);
            }
        }
    }
}
