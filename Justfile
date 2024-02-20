default:
    @echo Nothing to do

build-release:
    cargo build --release
    @just build-examples

build-release-verbose:
    cargo build --release -vvv
    @just build-examples-verbose

build-release-toolchain:
    cargo build --release --features libdragon-sys/buildtoolchain -vvv
    @just build-examples-verbose

build-examples:
    @for example in `ls libdragon-examples`; do \
        cd "libdragon-examples/$example" && just build-release; cd ../..;  \
    done

build-examples-verbose:
    @for example in `ls libdragon-examples`; do \
        cd  "libdragon-examples/$example" && just build-release-verbose; cd ../..; \
    done

clean:
    cargo clean --release
