default:
    @echo Nothing to do

build:
    cargo build
    @just build-examples

build-release:
    cargo build --release
    @just build-examples-release

build-release-verbose:
    cargo build --release -vvv
    @just build-examples-release-verbose

build-toolchain:
    cargo build --features libdragon-sys/buildtoolchain -vvv
    @just build-examples-release-verbose

build-release-toolchain:
    cargo build --release --features libdragon-sys/buildtoolchain -vvv
    @just build-examples-release-verbose

build-examples:
    @for example in `ls libdragon-examples`; do \
        cd "libdragon-examples/$example" && just build; cd ../..;  \
    done

build-examples-release:
    @for example in `ls libdragon-examples`; do \
        cd "libdragon-examples/$example" && just build-release; cd ../..;  \
    done

build-examples-release-verbose:
    @for example in `ls libdragon-examples`; do \
        cd  "libdragon-examples/$example" && just build-release-verbose; cd ../..; \
    done

clean:
    cargo clean --release
