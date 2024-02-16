default:
    @echo Nothing to do

build-release:
    cargo build --release
    cd temp-test && just finish-rom

build-release-verbose:
    cargo build --release -vvv
    cd temp-test && just finish-rom

build-release-toolchain:
    cargo build --release --features libdragon-sys/buildtoolchain -vvv
    cd temp-test && just finish-rom

clean:
    cargo clean --release
