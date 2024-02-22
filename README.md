# Rust bindings for libdragon (libdragon-rs)

### Components
The core of libdragon-rs consists of:

- [`libdragon`](./libdragon): High-level safe API (work in progress)
- [`libdragon-sys`](./libdragon-sys): Low-level unsafe API (automatically generated)
- [`libdragon-examples`](./libdragon-examples): Demo applications that run on the N64

### Obtaining and building

1. Clone this repository:

```bash
git clone --recurse-submodules https://github.com/sarchar/libdragon-rs
```

2. Install Rust

3. Install `just`
```bash
cargo install just
```

4. You'll probably need several dependencies:
```bash
apt-get install clang build-essential
```

5. Build
```bash
just build
```

Or, for release builds,

```bash
just build-release
```

The build process will fetch the prebuilt toolchain at 
[https://github.com/sarchar/libdragon/releases/tag/toolchain-continuous-prerelease](https://github.com/sarchar/libdragon/releases/tag/toolchain-continuous-prerelease). 
If you want to compile it yourself, use `just build-toolchain`.  It will take a long time.

In the end, example ROMs should be present as `./target/mips-nintendo64-none/{debug,release}/*.z64`

### Troubleshooting

Currently, you may see the following errors after your first build:
```
          rust-lld: error: undefined symbol: rsp_crash_text_start
          >>> referenced by rsp.c:26 (src/rsp.c:26)
          >>>               rsp.o:(rsp_crash) in archive /tmp/rustcBflvZN/liblibdragon_sys-36cbfba1e0582cd4.rlib

          rust-lld: error: undefined symbol: rsp_crash_text_end
          >>> referenced by rsp.c:26 (src/rsp.c:26)
          >>>               rsp.o:(rsp_crash) in archive /tmp/rustcBflvZN/liblibdragon_sys-36cbfba1e0582cd4.rlib

          rust-lld: error: undefined symbol: rsp_crash_data_start
          >>> referenced by rsp.c:26 (src/rsp.c:26)
          >>>               rsp.o:(rsp_crash) in archive /tmp/rustcBflvZN/liblibdragon_sys-36cbfba1e0582cd4.rlib

          rust-lld: error: undefined symbol: rsp_crash_data_end
          >>> referenced by rsp.c:26 (src/rsp.c:26)
          >>>               rsp.o:(rsp_crash) in archive /tmp/rustcBflvZN/liblibdragon_sys-36cbfba1e0582cd4.rlib
```

Simply rebuild libdragon-sys once and it should work:

```
touch libdragon-sys/build.rs libdragon-sys/libdragon/src/rsp_crash.S
just build
```

