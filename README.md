
# Rust bindings for libdragon (libdragon-rs)

### Components
The core of libdragon-rs consists of:

- [`libdragon`](./libdragon): High-level safe API (39/45 modules wrapped)
- [`libdragon-sys`](./libdragon-sys): Low-level unsafe API (automatically generated)
- [`libdragon-build`](./libdragon-build): Build tools to produce N64 binary images
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

4. Install dependencies:
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

**DISCLAIMER** The build process will fetch the prebuilt toolchain at 
[https://github.com/sarchar/libdragon/releases/tag/toolchain-continuous-prerelease](https://github.com/sarchar/libdragon/releases/tag/toolchain-continuous-prerelease).  These are pre-compiled binaries. If you want to compile the toolchain yourself, use `just build-toolchain`.  It will take a long, long time.

In the end, example ROMs should be present as `./target/mips-nintendo64-none/{debug,release}/*.z64`

### Status

Complete modules (with documentation):
    * audio, backtrace, console, cop0, cop1, debug, dir (as part of dfs), display, dfs, dlfcn, dma, eeprom, eepromfs, exception, graphics, 
      interrupt, joybus, joybus_accessory (as part of joybus), joypad, mempak, mixer, model64, mpeg2, n64sys, rdp, rdpq, rsp, rspq, rtc, samplebuffer, sprite, 
      surface, timer, throttle, usb, wav64, xm64, ym64, yuv

Modules without an interface (TODO):
    * ay8910
    * tpak
    * fmath - implement traits on f32, etc
    * pixelfx
    * system - newlib hooks, etc
    * n64types - test unaligned data accesses. can be done through core::ptr::read_unaligned.


