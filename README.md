# 🩺 arch-health

**A modular system health checker for Arch Linux.**  
Easily audit your system's configuration, shared libraries, installed packages, and audit framework — all from a clean, async-powered CLI.

---

## ✨ Features

- ✅ Runs system health checks for:
    - Shared libraries (`ldd`)
    - Package database (`pacman -Qi`)
    - Audit subsystem (`auditctl`)
- 📦 Modular check architecture — add your own in `src/checks/`
- 🖨️ Clean output formatting with colour and log level filtering
- 🔄 CLI spinner for progress feedback
- 🧾 Configurable behaviour via `config.toml`
- 🧪 Built with async/await (`tokio`) and structured logging (`tracing`)

---

## 📦 Installation

```bash
git clone https://github.com/skumdump/arch-health.git
cd arch-health
cargo build --release
./target/release/arch-health
```

---

## ⚙️ Usage

```bash
arch-health [OPTIONS]
```

### Options

| Flag                    | Description                                        |
|-------------------------|----------------------------------------------------|
| `--debug`               | Enable debug-level logging                         |
| `--no-spinner`          | Disable spinner/progress animation                 |
| `--checks <CHECKS>`     | Comma-separated list of checks (e.g. pacman,audit) |
| `--config <FILE>`       | Load configuration from a custom config path       |
| `--output-format <FMT>` | Set output format (e.g. `default`, `json`)         |
| `--log-file <FILE>`     | Write logs to a specified file                     |
| `--help`                | Show help message and exit                         |

---

## 🛠 Configuration

Create a `config.toml` file in one of the following:

- `$XDG_CONFIG_HOME/arch-health/config.toml`
- `./config.toml`

Example:

```toml
[logging]
level = "info"

[output]
color = true
spinner = true
```

---

## 📂 Project Structure

```
src/
├── main.rs             # CLI and entrypoint
├── config.rs           # Loads and parses config
├── checks/             # Modular system check implementations
├── output/             # Logging, progress, formatting
```

---

## 🧪 Development

Run checks and tests:

```bash
cargo check
cargo clippy
cargo test
```

To run with live output:

```bash
RUST_LOG=debug cargo run
```

---

## 📜 License

MIT © skumdump  
Contributions welcome — open a PR or issue!
