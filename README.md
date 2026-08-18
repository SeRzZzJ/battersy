
# Battersy

A lightweight, zero-dependency Linux terminal utility written in Rust that fetches and displays your hardware battery status with clean, color-coded ASCII progress bars.

## Features

- **Zero Dependencies**: Powered entirely by the Rust standard library (`std`).
- **Decoupled Module Architecture**: Split cleanly into domain kinds, functional validation layers, and layout formatting blocks for maximum test isolation.
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
    ├── kinds.rs       # Core Enums (Colors, BatteryStatus) and string parsers
    ├── parser.rs      # Functional argument-checking pipeline stages
    └── formatter.rs   # Mathematical scaling and ASCII terminal output rendering
```

1. **`kinds`**: Encapsulates `Colors` (with raw ANSI maps) and the `BatteryStatus` engine which resolves operational emojis (`🡅`, `✔`, `🡇`).
2. **`parser`**: Controls the conveyor belt stages (`verify_flag` and `extract_battery_path`) without introducing I/O side-effects.
3. **`formatter`**: Parses cell metric strings into bounded boundaries safely utilizing `usize::try_from` calculations.

## Output Structure

When executing Battersy, it handles raw virtual filesystem profiles and colorizes output data streams for terminal emulators:

- **Charging (42%)**: `🡅 42% [####------]` (Yellow highlights)
- **Discharging (15%)**: `🡇 15% [#---------]` (Red highlights)
- **Full / Idle (100%)**: `✔ 100% [##########]` (Green highlights)

## Installation & Compilation

This application targets Linux-based environments exposing live power configurations via the local `/sys/class/power_supply/` subsystems.

```bash
# 1. Clone the codebase
git clone https://github.com
cd battersy

# 2. Build production-optimized binaries
cargo build --release
```

## Usage

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

