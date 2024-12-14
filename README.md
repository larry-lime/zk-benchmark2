# ZK Benchmark

## Result

[Paper](./research-paper/capstone/Capstone_Report_Template.pdf)

## Quick Start

### Risc Zero

#### 1. Install the RISC Zero Toolchain

```bash
cargo install cargo-binstall
cargo binstall cargo-risczero
cargo risczero install
```

#### 2. Roughly benchmark RISC-0 Execution and Proving

```bash
cd risc0
cargo build --release
```

### SP1 - Succinct Labs

#### 1. Install the SP1 Toolchain

```bash
curl -L https://sp1.succinct.xyz | bash
sp1up
cargo prove --version
```

#### 2. Roughly benchmark SP1 Execution and Proving

```bash
cd sp1
cargo build --release
```

### Jolt - a16z

#### 1. Install the Jolt Toolchain

```bash
cargo +nightly install --git https://github.com/a16z/jolt --force --bins jolt
jolt install-toolchain
```

#### 2. Roughly benchmark Jolt Execution and Proving

```bash
cd jolt
cargo build --release
```
