# Battersy

A lightweight, zero-dependency Linux terminal utility written in Rust that fetches and displays your hardware battery status with clean, color-coded ASCII progress bars.

## Features

- **Zero Dependencies**: Powered entirely by the Rust standard library (`std`).
- **Decoupled Module Architecture**: Split cleanly into domain types, functional validation layers, and layout formatting blocks for maximum test isolation.
- **Robust Pipeline Design**: Implements a strict functional data conveyor pattern to safely ingest, validate, and move configuration states through CLI parameter pipelines.
- **Input Normalization**: Inherent path scrubbing that accepts both raw labels (`BAT0`) and multi-slashed paths (`///BAT0`) interchangeably without breaking kernel directory trees.
- **Trait-Driven State Engine**: Uses Rust's standard `FromStr` conversion traits to natively deserialize sysfs string states into structured, strongly-typed memory variants.

## Module Architecture

The codebase relies on a modular layout pattern ensuring strict separation of concerns across files:

```text
battersy/
├── Cargo.toml         # Crate package metadata, keywords, and lint settings
├── README.md          # Project documentation and specifications
└── src/
    ├── main.rs        # Binary entry point feeding CLI token iterators
    ├── lib.rs         # System root aggregating modules and the execution loop
    ├── types.rs       # Core Enums (Colors, BatteryStatus) and string parsers
    ├── parser.rs      # Functional argument-checking pipeline stages
    └── formatter.rs   # Mathematical scaling and ASCII terminal output rendering
```

1. **`types`**: Encapsulates `Colors` (with raw ANSI maps) and the `BatteryStatus` engine which resolves operational emojis (`🡅`, `✔`, `🡇`).
2. **`parser`**: Controls the conveyor belt stages (`verify_flag` and `extract_battery_path`) without introducing I/O side-effects.
3. **`formatter`**: Parses cell metric strings into bounded boundaries safely utilizing `usize::try_from` calculations.

## Output Structure

When executing Battersy, it handles raw virtual filesystem profiles and colorizes output data streams for terminal emulators:

- **Charging (42%)**: `🡅 42% [####------]` (Yellow highlights)
- **Discharging (15%)**: `🡇 15% [#---------]` (Red highlights)
- **Full / Idle (100%)**: `✔ 100% [##########]` (Green highlights)

## Installation

### As a Global CLI Tool
To install the executable binary directly onto your system from crates.io, execute:

```bash
cargo install battersy
```

### From Source
If you prefer to clone the codebase and compile the optimized binary manually:

```bash
# 1. Clone the codebase
git clone https://github.com/SeRzZzJ/battersy
cd battersy

# 2. Build production-optimized binaries
cargo build --release
```

---

## System Integration & $PATH Configuration

When you run `cargo install`, the compiled `battersy` binary is placed inside your home directory at `~/.cargo/bin/`. To run the global `battersy` command from anywhere in your terminal, this directory must be inside your system's `$PATH` variable.

### Activating the Global Command
If your terminal returns a `command not found` error after installation, add the Cargo binary folder to your active shell configuration:

- **For Bash (`~/.bashrc`):**
  ```bash
  echo 'export PATH="HOME/.cargo/bin:PATH"' >> ~/.bashrc
  source ~/.bashrc
  ```
- **For Zsh (`~/.zshrc`):**
  ```bash
  echo 'export PATH="HOME/.cargo/bin:PATH"' >> ~/.zshrc
  source ~/.zshrc
  ```

### Reverting/Removing from $PATH
If you ever want to remove this modification from your system profile:
1. Open your configuration file in a text editor (e.g., `nano ~/.bashrc`).
2. Scroll to the bottom and delete the `export PATH="$HOME/.cargo/bin:$PATH"` line entirely.
3. Save the file and restart your terminal session by running `exec bash` or `exec zsh`.

---

## Usage
### From Cargo
```bash
# Execute pointing to a specific power peripheral target
battersy -d BAT0

# Inputs are automatically scrubbed and normalized
battersy -d /BAT0
```
### From Source
```bash
# Execute pointing to a specific power peripheral target
./target/release/battersy -d BAT0

# Inputs are automatically scrubbed and normalized
./target/release/battersy -d /BAT0
```
## Quality Assurance & Testing

The embedded verification framework contains granular target coverage evaluating flag exceptions, string sanitizers, and edge-case capacity values.

Run the test block using Cargo:
```bash
cargo test
```

To run standard compliance linting matching strict continuous integration configurations:
```bash
cargo clippy --all-targets -- -D warnings
```

## License

Distributed under the MIT License. See `LICENSE` for more information.

