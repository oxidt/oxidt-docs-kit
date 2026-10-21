# oxidt-docs-kit-core

The UI-framework-agnostic core of
[oxidt-docs-kit](https://github.com/oxidt/oxidt-docs-kit) (`topcoat-docs-kit`)
and [dioxus-docs-kit](https://crates.io/crates/dioxus-docs-kit).

It loads the content bundle written by
[`oxidt-docs-kit-build`](https://crates.io/crates/oxidt-docs-kit-build) and
answers every question a docs or blog shell asks of it — page lookup,
navigation, search, OpenAPI operations, `llms.txt`, sitemaps and RSS feeds. It
renders nothing and has no `dioxus` dependency, so any UI layer can sit on top.

Dioxus apps should depend on `dioxus-docs-kit`, which re-exports this crate's
API at its existing paths.

```rust,ignore
use oxidt_docs_kit_core::{DocsConfig, DocsRegistry};
use std::sync::LazyLock;

static DOCS: LazyLock<DocsRegistry> = LazyLock::new(|| {
    DocsConfig::new(oxidt_docs_kit_core::docs_bundle!())
        .with_default_path("getting-started/introduction")
        .build()
});
```
