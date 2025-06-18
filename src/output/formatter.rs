pub fn print_header(msg: &str, format: &str) {
    if format == "text" {
        println!("\n=== {} ===\n", msg);
    }
}
