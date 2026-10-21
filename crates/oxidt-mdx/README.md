# oxidt-mdx

Parser and document types for Mintlify-style MDX — callouts, cards, tabs, code
groups, accordions, steps, API fields and more — plus frontmatter extraction
and OpenAPI spec parsing. No UI framework: the tree is plain `serde` data that
a renderer turns into markup.

Renderers:

- [`topcoat-docs-kit`](https://crates.io/crates/topcoat-docs-kit) renders it
  to server-side HTML.
- [`dioxus-mdx`](https://crates.io/crates/dioxus-mdx) renders it as Dioxus
  components.

Formerly the parser half of `dioxus-mdx`.

## Usage

```toml
[dependencies]
oxidt-mdx = "0.11"
```

```rust
use oxidt_mdx::{parse_document, parse_mdx};

let doc = parse_document(r#"---
title: Getting Started
---

<Tip>This is a helpful tip!</Tip>

## Introduction

Welcome to the documentation.
"#);

assert_eq!(doc.frontmatter.title, "Getting Started");

// Parse content without frontmatter
let nodes = parse_mdx("## Hello\n\n<Note>A note</Note>");
```

The parsed tree is serializable, so you can parse at build time and ship the
result instead of the parser — that is what
[`oxidt-docs-kit-build`](https://crates.io/crates/oxidt-docs-kit-build) does.

## Features

- `parse` (default) — the MDX/Markdown parser, including Markdown → HTML
  rendering. Pulls in `markdown` and a regex engine. Turn it off to keep only
  the document types (e.g. in a wasm client that loads a prebuilt bundle).
- `openapi` — the `OpenApiSpec` types. No extra dependencies.
- `openapi-parse` (default) — `parse_openapi()` and inline
  `<OpenAPI>…</OpenAPI>` blocks, plus `openapiv3` and `serde_yaml`. Without
  it an `<OpenAPI>` block renders as text. Frontmatter parsing is unaffected:
  it uses the built-in `parse_yaml_lite` subset parser.

## License

MIT
