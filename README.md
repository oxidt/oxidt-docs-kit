# oxidt-docs-kit

A documentation site framework for Rust: Mintlify-style MDX, sidebar
navigation, full-text search, OpenAPI reference pages and light/dark themes,
all parsed at build time and embedded in the binary. The shell is
[Topcoat](https://github.com/tokio-rs/topcoat) (server-rendered, no runtime
script) styled with [oxidt-ui](https://github.com/oxidt/oxidt-ui).

The content pipeline crates are UI-agnostic and also power
[dioxus-docs-kit](https://github.com/hauju/dioxus-docs-kit), the Dioxus +
DaisyUI shell.

## Crates

| Crate | Description |
|-------|-------------|
| [`topcoat-docs-kit`](crates/topcoat-docs-kit/) | Topcoat docs shell: layout, sidebar, search page, OpenAPI pages (experimental, Rust 1.98) |
| [`oxidt-docs-kit-core`](crates/oxidt-docs-kit-core/) | Bundle loader, docs/blog/skills registries, search, SEO helpers; renders nothing |
| [`oxidt-docs-kit-build`](crates/oxidt-docs-kit-build/) | Build-time helper: parses `_nav.json`, MDX and OpenAPI specs into one JSON bundle |
| [`oxidt-mdx`](crates/oxidt-mdx/) | MDX parser and document types, no UI framework |
| [`hl-lite`](crates/hl-lite/) | Zero-dependency syntax highlighter |
| [`oxidt-docs`](crates/oxidt-docs/) | Static site generator: MDX + `_nav.json` in, plain HTML out (unpublished) |

## Running the example

```sh
cargo run -p topcoat-docs-example
```

It renders the sample content in `docs/` and `blog/`. See the
[topcoat-docs-kit README](crates/topcoat-docs-kit/README.md) to wire the kit
into your own app.

## License

MIT
