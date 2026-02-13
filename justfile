name := `echo ${PWD##*/}`
rootdir := `git rev-parse --show-toplevel`
target := rootdir + "/target"

default: build-release

# Build debug
build:
    cargo build

# Build release
build-release:
    cargo build --release

# Run debug
run:
    cargo run

# Install locally
install: build-release
    install -Dm0755 "target/release/cosmic-applet-voicehub" /usr/local/bin/cosmic-applet-voicehub
    install -Dm0644 ".config/com.deivisan.voicehub.desktop" /usr/local/share/applications/com.deivisan.voicehub.desktop

# Uninstall
uninstall:
    rm -f /usr/local/bin/cosmic-applet-voicehub
    rm -f /usr/local/share/applications/com.deivisan.voicehub.desktop

# Clean build artifacts
clean:
    cargo clean

# Check formatting and lints
check:
    cargo fmt --check
    cargo clippy --all-features -- -W clippy::pedantic

# Format code
fmt:
    cargo fmt

# Update dependencies
update:
    cargo update

# Watch for changes and rebuild
dev:
    cargo watch -x build

# Build with vendored dependencies (for packaging)
vendored:
    cargo build --release --features vendored
