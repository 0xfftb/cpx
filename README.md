### Description

Minimal tool to copy file contents to clipboard

### Prerequisites

You need Rust and Cargo installed.
  ```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```

### Installation

1. Clone the repo
   ```sh
   git clone https://github.com/mateomolinari/cpx.git
   ```
2. Build binary
   ```sh
   cargo build --release
   ```
3. Copy binary to PATH
   ```sh
   sudo cp target/release/cpx /usr/local/bin
   ```


## Usage

```sh
   cpx example-file.json
```