# Build the application
[group("dev")]
build:
    cargo build --workspace

# Run tests (matches CI)
[group("dev")]
test:
    cargo test --workspace

# Serve the topcoat example
[group("dev")]
serve:
    cargo run -p topcoat-docs-example

# Rebuild the kit's precompiled stylesheet (matches the CI freshness check).
# Uses the lockfile-pinned tailwind binary — bunx may silently resolve a newer
# cached version, which changes the output byte-for-byte and fails CI.
[group("dev")]
css:
    bun install --frozen-lockfile
    ./node_modules/.bin/tailwindcss -i crates/topcoat-docs-kit/tailwind.css -o crates/topcoat-docs-kit/assets/docs-kit.css --minify

# Refresh vendor/oxidt-ui from the oxidt-ui tag pinned in the script.
[group("dev")]
oxidt-ui:
    ./scripts/vendor-oxidt-ui.sh

# Format check (matches CI)
[group("lint")]
fmt:
    cargo fmt --all --check

# Clippy (matches CI)
[group("lint")]
clippy:
    cargo clippy --workspace --all-targets --all-features -- -D warnings

# Unused dependency check (matches CI)
[group("lint")]
machete:
    cargo machete

# Run all lints and tests (matches CI)
[group("lint")]
check: fmt clippy machete test
