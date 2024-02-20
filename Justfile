default:
    @echo Nothing to do

build-release:
    cargo build --release

    @for example in `ls libdragon-examples`; do \
        cd "libdragon-examples/$example" && just build-release; \
    done

build-release-verbose:
    cargo build --release -vvv

    @for example in `ls libdragon-examples`; do \
        cd "libdragon-examples/$example" && just build-release-verbose; \
    done

build-release-toolchain:
    cargo build --release --features libdragon-sys/buildtoolchain -vvv

    @for example in `ls libdragon-examples`; do \
        cd "libdragon-examples/$example" && just build-release-verbose; \
    done

clean:
    cargo clean --release
