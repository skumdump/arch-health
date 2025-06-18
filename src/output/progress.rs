use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;

pub fn start_spinner(msg: &str) -> ProgressBar {
    let pb = ProgressBar::new_spinner();
    pb.set_message(msg.to_string());  // clone &str to String
    pb.enable_steady_tick(Duration::from_millis(100));
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_strings(&["⠋","⠙","⠹","⠸","⠼","⠴","⠦","⠧","⠇","⠏"])
            .template("{spinner} {msg}")
            .unwrap(),
    );
    pb
}
