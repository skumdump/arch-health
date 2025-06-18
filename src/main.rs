mod checks;
mod output;
mod config;

use clap::Parser;
use config::load_config;
use output::formatter;
use output::progress::start_spinner;

#[derive(Parser, Debug)]
#[command(name = "arch-health_")]
struct Cli {
    #[arg(short, long, default_value = "text")]
    format: String,
}

fn main() {
    env_logger::init();

    let cli = Cli::parse();
    let config = load_config();
    let format = config
        .as_ref()
        .map(|c| c.output.format.clone())
        .unwrap_or(cli.format);

    let use_library = config.as_ref().map_or(true, |c| c.checks.library);
    let use_pacman = config.as_ref().map_or(true, |c| c.checks.pacman);
    let use_audit = config.as_ref().map_or(false, |c| c.checks.audit);

    formatter::print_header("Running Arch system health check...", &format);

    if use_library {
        let pb = start_spinner("Checking shared libraries with ldd...");
        pb.finish_and_clear(); // Clear the spinner first
        checks::library::run_check(&format);
    }

    if use_pacman {
        let pb = start_spinner("Checking pacman package consistency...");
        checks::pacman::run_check(&format);
        pb.finish_with_message("✅ Done");
    }

    if use_audit {
        let pb = start_spinner("Running arch-audit...");
        checks::audit::run_check(&format);
        pb.finish_with_message("✅ Done");
    }

    formatter::print_header("Health check complete.", &format);
}
