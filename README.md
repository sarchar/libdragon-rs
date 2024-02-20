# Rust bindings for libdragon (libdragon-rs)

### Components
The core of libdragon-rs consists of:

- [`libdragon`](./libdragon): High-level safe API (TODO)
- [`libdragon-sys`](./libdragon-sys): Low-level unsafe API (automatically generated)

### Obtaining and building

Clone this repository:

```bash
git clone --recurse-submodules https://github.com/sarchar/libdragon-rs
```

Install Rust

Install `just`
```bash
cargo install just
```

You'll probably need several dependencies:
```bash
apt-get install clang build-essential
```

Build release
```bash
just build-release
```

This will take a **long** time (30m-1hr depending on your system). If you want to monitor progress instead, run with verbose:

```bash
just build-release-verbose
```

In the end, example ROMs should be present as `./target/mips-nintendo64-none/release/*.z64`
