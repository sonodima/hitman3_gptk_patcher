# HITMAN3 GPTK Patcher

Experimental Patcher to run HITMAN3 on Apple's Game Porting Toolkit

## Build

1. Install Rust `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
2. Compile `cargo build --release`

## Usage

```bash
./target/release/h3_patcher <HITMAN3_PATH>
```

_(where <HITMAN3_PATH> is the main install directory, containing the Launcher.exe binary)_
