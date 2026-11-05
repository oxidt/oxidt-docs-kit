//! # oxidt-mdx
//!
//! Parser and document types for Mintlify-style MDX documentation, with no UI
//! framework dependency. The topcoat docs kit renders these types to HTML;
//! `dioxus-mdx` renders them as Dioxus components.
//!
//! - **Parser** (`parse`): frontmatter, custom components, Markdown to HTML
//! - **OpenAPI** (`openapi`, `openapi-parse`): spec types and parser
//! - **Helpers**: heading slugs, Lucide icon data, bundled Pele diagram assets
//!
//! ```rust
//! use oxidt_mdx::{parse_document, parse_mdx, DocNode};
//!
//! let mdx_content = r#"---
//! title: Getting Started
//! ---
//!
//! <Tip>This is a helpful tip!</Tip>
//!
//! ## Introduction
//!
//! Welcome to the documentation.
//! "#;
//!
//! // Parse with frontmatter
//! let doc = parse_document(mdx_content);
//! assert_eq!(doc.frontmatter.title, "Getting Started");
//!
//! // Parse content only
//! let nodes = parse_mdx("## Hello\n\n<Note>A note</Note>");
//! ```
//!
//! ## Supported Components
//!
//! - **Callouts**: `<Tip>`, `<Note>`, `<Warning>`, `<Info>`
//! - **Cards**: `<Card>`, `<CardGroup>`
//! - **Tabs**: `<Tabs>`, `<Tab>`
//! - **Steps**: `<Steps>`, `<Step>`
//! - **Accordion**: `<AccordionGroup>`, `<Accordion>`
//! - **Code**: `<CodeGroup>`, fenced code blocks
//! - **API Docs**: `<ParamField>`, `<ResponseField>`, `<Expandable>`
//! - **Examples**: `<RequestExample>`, `<ResponseExample>`
//! - **Changelog**: `<Update>`

pub mod lucide;
mod mermaid_js;
pub mod parser;
mod pele_assets;
#[cfg(feature = "parse")]
mod re;
mod text;

// Re-export parser types and functions
pub use parser::{
    AccordionGroupNode, AccordionNode, ApiInfo, ApiOperation, ApiParameter, ApiRequestBody,
    ApiResponse, ApiServer, ApiTag, CalloutNode, CalloutType, CardGroupNode, CardNode,
    CodeBlockNode, CodeGroupNode, DocFrontmatter, DocNode, ExpandableNode, HttpMethod,
    MediaTypeContent, OpenApiNode, OpenApiSpec, ParamFieldNode, ParamLocation, ParameterLocation,
    ParsedDoc, RequestExampleNode, ResponseExampleNode, ResponseFieldNode, SchemaDefinition,
    SchemaType, StepNode, StepsNode, TabNode, TabsNode, UpdateNode, YamlLiteError, YamlMap,
    YamlValue, parse_yaml_lite,
};

// Heading helpers are free of both dioxus and regex, so they are available in
// every configuration (the renderer, the TOC and the build-time bundle
// generator all have to agree on anchor ids).
pub use text::{
    clean_step_title, extract_headers, parse_atx_heading, slugify, strip_markdown_links,
};

// Parsing lives behind the `parse` feature (default). A docs-kit app turns it
// off: its content is parsed into a bundle at build time.
#[cfg(feature = "parse")]
pub use parser::{
    extract_frontmatter, parse_body, parse_document, parse_mdx, strip_leading_h1, to_html,
    to_html_with_heading_ids,
};

// Spec parsing lives behind the `openapi-parse` feature (default); the
// `OpenApiSpec` types are always available.
#[cfg(feature = "openapi-parse")]
pub use parser::{OpenApiError, parse_openapi};

pub use mermaid_js::MERMAID_JS;

pub use pele_assets::PELE_VERSION;
#[cfg(not(target_arch = "wasm32"))]
pub use pele_assets::pele_asset;
