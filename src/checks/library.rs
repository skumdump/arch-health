use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use std::fs;
use std::io::Read;
use std::path::Path;
use std::process::Command;
use std::sync::{Arc, Mutex};

fn is_elf(path: &Path) -> bool {
    let mut file = match fs::File::open(path) {
        Ok(f) => f,
        Err(_) => return false,
    };
    let mut magic = [0u8; 4];
    if file.read_exact(&mut magic).is_err() {
        return false;
    }
    magic == [0x7f, b'E', b'L', b'F']
}

pub fn run_check(format: &str) {
    let dirs = ["/usr/bin", "/usr/lib", "/usr/local/bin", "/usr/local/lib"];

    // Collect all ELF files first
    let mut files = Vec::new();
    for dir in &dirs {
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() && is_elf(&path) {
                    files.push(path);
                }
            }
        }
    }

    let pb = ProgressBar::new(files.len() as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner} [{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} {msg}")
            .unwrap()
            .progress_chars("=>-"),
    );

    // Thread-safe error collector
    let errors = Arc::new(Mutex::new(Vec::new()));

    files.par_iter().for_each(|path| {
        let output = Command::new("ldd").arg(path).output();

        if let Ok(output) = output {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);

            if stdout.contains("not found") || stderr.contains("not found") {
                let mut errs = errors.lock().unwrap();
                errs.push(format!("{}: missing libraries\n{}", path.display(), stdout));
            }
        } else {
            let mut errs = errors.lock().unwrap();
            errs.push(format!("Failed to run ldd on {}", path.display()));
        }

        pb.inc(1);
    });

    pb.finish_with_message("ldd scan complete");

    if format == "text" {
        let errs = errors.lock().unwrap();
        if errs.is_empty() {
            println!("✅ [ldd] Checked shared libraries: no missing dependencies found");
        } else {
            println!("❌ [ldd] Missing libraries detected:");
            for e in errs.iter() {
                println!("{}", e);
            }
        }
    }
}
