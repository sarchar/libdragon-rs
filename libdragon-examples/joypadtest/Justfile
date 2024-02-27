set dotenv-filename := ".libdragon-env"
import? '.libdragon-just'

default:
    @echo Nothing to do

build: 
    cargo build
    @just lb-finish-rom

build-release: 
    cargo build --release
    @just lb-finish-rom

build-release-verbose: 
    cargo build --release --verbose
    @just lb-finish-rom

