use rayon::prelude::*;
use std::fs;
use std::io::{self, Read, Write};
use std::path::Path;
use std::process::Command;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Instant;

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

fn print_header() {
    eprintln!("\n╭─────────────────────────────────────────────╮");
    eprintln!("│           🔍 ELF Dependency Scanner         │");
    eprintln!("╰─────────────────────────────────────────────╯");
}

fn print_scanning_dirs(dirs: &[&str]) {
    eprintln!("\n📂 Scanning directories:");
    for dir in dirs {
        if Path::new(dir).exists() {
            eprintln!("   ✓ {}", dir);
        } else {
            eprintln!("   ✗ {} (not found)", dir);
        }
    }
}

fn create_progress_bar(current: usize, total: usize, width: usize) -> String {
    let percentage = (current as f64 / total as f64 * 100.0) as usize;
    let filled = (current as f64 / total as f64 * width as f64) as usize;
    let empty = width - filled;

    format!(
        "│{}{} │ {:3}% ({:4}/{:4})",
        "█".repeat(filled),
        "░".repeat(empty),
        percentage,
        current,
        total
    )
}

pub fn run_check(format: &str) {
    let start_time = Instant::now();

    // Clear any existing spinner output
    eprint!("\r\x1b[2K");
    io::stderr().flush().unwrap();

    print_header();

    let dirs = ["/usr/bin", "/usr/lib", "/usr/local/bin", "/usr/local/lib"];
    print_scanning_dirs(&dirs);

    // Collect all ELF files first
    eprint!("\n🔎 Discovering ELF files");
    io::stderr().flush().unwrap();

    let mut files = Vec::new();
    for dir in &dirs {
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() && is_elf(&path) {
                    files.push(path);
                    if files.len() % 50 == 0 {
                        eprint!(".");
                        io::stderr().flush().unwrap();
                    }
                }
            }
        }
    }

    let processed = Arc::new(AtomicUsize::new(0));
    let total = files.len();

    eprintln!(" found {} files\n", total);
    eprintln!("⚙️  Running dependency analysis...");
    eprintln!("╭─────────────────────────────────────────────╮");

    // Thread-safe error collector
    let errors = Arc::new(Mutex::new(Vec::new()));

    files.par_iter().for_each(|path| {
        let output = Command::new("ldd").arg(path).output();

        if let Ok(output) = output {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);

            if stdout.contains("not found") || stderr.contains("not found") {
                let mut errs = errors.lock().unwrap();
                errs.push(format!("📄 {}\n   ❌ Missing dependencies:\n{}",
                                  path.display(),
                                  stdout.lines()
                                      .filter(|line| line.contains("not found"))
                                      .map(|line| format!("      {}", line.trim()))
                                      .collect::<Vec<_>>()
                                      .join("\n")
                ));
            }
        } else {
            let mut errs = errors.lock().unwrap();
            errs.push(format!("⚠️  Failed to analyze: {}", path.display()));
        }

        let count = processed.fetch_add(1, Ordering::Relaxed) + 1;
        if count % 25 == 0 || count == total {
            let progress_bar = create_progress_bar(count, total, 35);
            eprint!("\r{}", progress_bar);
            io::stderr().flush().unwrap();
        }
    });

    let elapsed = start_time.elapsed();
    eprintln!("\r│{}│ 100% ({:4}/{:4})", "█".repeat(35), total, total);
    eprintln!("╰─────────────────────────────────────────────╯");
    eprintln!("⏱️  Completed in {:.2}s", elapsed.as_secs_f64());

    if format == "text" {
        let errs = errors.lock().unwrap();
        eprintln!("\n╭─────────────────────────────────────────────╮");
        if errs.is_empty() {
            eprintln!("│                  🎉 RESULTS                 │");
            eprintln!("╰─────────────────────────────────────────────╯");
            println!("✅ All {} ELF files have satisfied dependencies!", total);
            println!("🔗 No missing shared libraries detected");
        } else {
            eprintln!("│                 ⚠️  RESULTS                 │");
            eprintln!("╰─────────────────────────────────────────────╯");
            println!("❌ Found {} files with missing dependencies:\n", errs.len());
            for (i, e) in errs.iter().enumerate() {
                if i > 0 { println!(); }
                println!("{}", e);
            }
            println!("\n📊 Summary: {}/{} files have dependency issues", errs.len(), total);
        }
    }
}