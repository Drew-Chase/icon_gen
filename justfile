set windows-shell := ["powershell.exe", "-NoLogo", "-NoProfile", "-Command"]
set shell := ["bash", "-c"]

default:
    @just --list

# Runs the desktop or cli app
run target="desktop" args="":
    cargo run --bin {{ if target == "desktop" { "simply_igen" } else if target == "cli" { "icon_gen" } else { error("Unknown app: " + target + ". Use 'desktop' or 'cli'.") } }} --release -- {{ args }}

# Build the entire workspace
build:
    cargo build --workspace --release

# Clean the entire workspace
clean:
    cargo clean

test:
    cargo test --release