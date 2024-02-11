default:
    @echo Nothing to do

build-release:
    cargo build --release
    cd temp-test && just finish-rom

build-release-verbose:
    cargo build --release -vvv
    cd temp-test && just finish-rom

