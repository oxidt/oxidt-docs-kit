# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build & Dev Commands

```sh
cargo run -p topcoat-docs-example   # Topcoat example site (sample content in docs/ and blog/)
just css                            # Rebuild topcoat-docs-kit's precompiled stylesheet
just oxidt-ui                       # Re-vendor oxidt-ui from the tag pinned in scripts/vendor-oxidt-ui.sh
```

**Linting & testing (matches CI):**
```sh
cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo machete
cargo test --workspace
```

**Shortcuts via justfile:** `just build`, `just test`, `just serve`, `just check`

## Project Architecture

**oxidt-docs-kit** — Rust documentation site framework: Mintlify-style MDX, OpenAPI reference pages, search. All content is parsed at build time and embedded. The shell is Topcoat (server-rendered, no runtime script) styled with oxidt-ui.

The Dioxus + DaisyUI shell (`dioxus-docs-kit`, the `dioxus-mdx` renderer, the Dioxus example site) lives in a separate repo, [hauju/dioxus-docs-kit](https://github.com/hauju/dioxus-docs-kit), and consumes the pipeline crates below from crates.io. Changes to the bundle format or `oxidt-docs-kit-core`'s API reach it only through a release.

### Crate Layout

| Crate | Path | Purpose |
|-------|------|---------|
| `topcoat-docs-kit` | `crates/topcoat-docs-kit/` | **Topcoat docs shell** (experimental) — server-rendered `docs_page`/`docs_head`/`doc_content`; own precompiled CSS. Rust 1.98. Example: `examples/topcoat-docs` |
| `oxidt-docs-kit-core` | `crates/oxidt-docs-kit-core/` | **UI-agnostic core** — bundle loader, `DocsRegistry`/`BlogRegistry`/`SkillsRegistry`, configs, search, SEO/XML helpers. Renders nothing |
| `oxidt-docs-kit-build` | `crates/oxidt-docs-kit-build/` | Build-time pipeline: parses `_nav.json` + all MDX + OpenAPI into one JSON bundle |
| `oxidt-mdx` | `crates/oxidt-mdx/` | MDX parser + document types, Lucide icon data, bundled Pele diagram assets. No UI framework |
| `hl-lite` | `crates/hl-lite/` | Zero-dependency syntax highlighter (12 hand-written lexers → `hl-*` spans) |
| `oxidt-docs` | `crates/oxidt-docs/` | Static site generator (unpublished) |

**Dependency direction:** `topcoat-docs-kit` → `oxidt-docs-kit-core` → `oxidt-mdx` (no features, types only). `oxidt-docs-kit-build` → `oxidt-mdx` with `features = ["parse", "openapi-parse"]`. Keep `oxidt-mdx` and `oxidt-docs-kit-core` buildable with `--no-default-features` and on Rust 1.88: `dioxus-docs-kit`'s wasm client links them that way.

### Content Pipeline

1. `build.rs` calls `oxidt_docs_kit_build::DocsBuild::new("docs/_nav.json").with_openapi(prefix, spec_path).generate()` (and `BlogBuild::new("blog/_blog.json")`, `SkillsBuild::new("skills/_skills.json")`)
2. The build crate parses every `.mdx` into `Vec<DocNode>` with prose rendered to HTML (`DocNode::Html`), parses the OpenAPI spec into `OpenApiSpec`, builds the section-level search index and writes `$OUT_DIR/docs_bundle.json` / `blog_bundle.json`. `cargo:rerun-if-changed` is emitted per file.
3. `docs_bundle!()` / `blog_bundle!()` / `skills_bundle!()` embed the JSON with `include_str!`
4. `DocsConfig::new(docs_bundle!()).build()` creates a `DocsRegistry` — it deserializes the bundle and derives only the cheap runtime bits (API sidebar entries, operation index, blog categories)

The bundle carries a `version` field (`BUNDLE_VERSION`, mirrored in `crates/oxidt-docs-kit-core/src/bundle.rs` and `crates/oxidt-docs-kit-build/src/bundle.rs`); bump it whenever the layout changes incompatibly. The build crate's `bundle.rs` holds serialize-only mirrors of `SearchEntry`, `BlogPost`, `BlogFrontmatter` and `BlogSearchEntry` — the core and kit tests dev-depend on the build crate and generate real bundles, which is what keeps the two definitions in sync.

**Adding a new doc page:** create `docs/<group>/<slug>.mdx` and add the path to `docs/_nav.json` (`tabs`, `groups[].tab`, `groups[].pages` relative to `docs/` without `.mdx`).

### Styling

- `crates/topcoat-docs-kit/tailwind.css` → precompiled `assets/docs-kit.css` (`just css`, CI checks freshness). Tailwind 4 + `@tailwindcss/typography`, oxidt-ui tokens/components from `vendor/oxidt-ui` (do not edit; bump the tag and run `just oxidt-ui`)
- The input uses `@import "tailwindcss" source(none)` + explicit `@source`s: the automatic repo-wide scan leaks classes from prose
- **Code block colors**: thirteen `--dk-hl-*` tokens in `crates/topcoat-docs-kit/theme.css`, each a `light-dark()` pair. Colors are CSS, never Rust; an unknown language renders the same markup with no spans
- Icons: Lucide SVG data in `crates/oxidt-mdx/src/lucide.rs`. Regenerate with `scripts/vendor-lucide-icons.py` after adding a new `Ld*` name

### Key Conventions

- Cargo features (oxidt-mdx): `parse` gates the parser implementation (markdown-rs + regex) while the AST types and `src/text.rs` are always compiled; `openapi` is types only, `openapi-parse` adds `openapiv3`/`serde_yaml`. Frontmatter uses `oxidt_mdx::parse_yaml_lite`, not `serde_yaml`
- CI toolchain: Rust 1.98.0 (Topcoat's MSRV); MSRV job checks the pipeline crates on 1.88. Bun for Tailwind and the Pele diagram tests
