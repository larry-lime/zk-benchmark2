# ZK Benchmark 2.0

## Benchmark Memory Performance of zkVMs

...

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
cargo build --release
time ./target/release/host
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
(cd program/ && cargo prove build) && (cd script/ && cargo build --release)
time ./script/target/release/sp1-bench-script
```

### Jolt - a16z

#### 1. Install the Jolt Toolchain

```bash
cargo +nightly install --git https://github.com/a16z/jolt --force --bins jolt
jolt install-toolchain
```

#### 2. Roughly benchmark Jolt Execution and Proving

```bash
cargo build --release
time ./target/release/jolt-bench
```

## Building Research Paper

### Requirements

To use these templates, we require the following software.

1. [_Pandoc_](https://pandoc.org/) for converting between the Markdown files into other document formats.
2. [_LaTeX_](https://www.latex-project.org/) for creating PDF documents.
3. Shell such as Bash for invoking the build scripts.

### Building

```bash
cd research-paper
source ./build.sh
pdf_print # To build the pdf for printing
pdf_ereader # To build the pdf for ereader
```

## Todos

- [x] Individually benchmark SP1
- [x] Individually benchmark Jolt
- [x] Individually benchmark RISC-0

## Benchmarking Todos

### Algorithm Checking

- [ ] Compare benchmarks with existing code to make sure we aren't making dumb mistakes
- [ ] Double check the algorithm used is writing to memory in the same way
- [ ] Benchmark on memory non-intensive algorithms
- [ ] Benchmark on memory intensive algorithms (copy paste from other projects here)

### Benchmarking Input

- [ ] Run on input of n = 10
- [ ] Run on input of n = 100
- [ ] Run on input of n = 10000
- [ ] Run on input of n = 100000
- [ ] Put everything in a table (json, toml, txt, etc.)

### Draw benchmarking conclusion

- [ ] Did it align with expectations?
  - [ ] If no, why?
- [ ] Analyze the results
  - [ ] Why is SP1 faster than Jolt?
