//! Raw Markdown copies of doc pages, for agents that read `<page>.md` instead
//! of rendered HTML.

use oxidt_docs_kit_core::DocsRegistry;
use topcoat::{
    Result,
    router::{HeaderValue, error::not_found, header, response::IntoResponse},
    view::{View, component, view},
};

/// The Markdown source of the doc page at `path` (a trailing `.md` is
/// ignored; empty means the registry's default page), as `text/markdown`
/// with the page title and description restored as a heading and quote.
/// OpenAPI endpoint pages and unknown paths are a 404.
///
/// Topcoat can't register a route beside the docs catch-all page, so the page
/// rewrites `.md` requests to a route of your own that calls this:
///
/// ```rust,ignore
/// #[page("/docs/{*doc_path}")]
/// async fn docs(cx: &Cx) -> Result<impl View> {
///     let path = path_param::<DocPath>(cx).collect::<Vec<_>>().join("/");
///     if path.ends_with(".md") {
///         return Err(rewrite(format!("/_docs/md/{path}"), Body::empty()).into());
///     }
///     // ...
/// }
///
/// #[route(GET "/_docs/md/{*doc_path}")]
/// async fn docs_md(cx: &Cx) -> Result<impl IntoResponse> {
///     let path = path_param::<DocPath>(cx).collect::<Vec<_>>().join("/");
///     topcoat_docs_kit::docs_markdown(&DOCS, &path)
/// }
/// ```
pub fn docs_markdown(registry: &DocsRegistry, path: &str) -> Result<impl IntoResponse + use<>> {
    let path = path.strip_suffix(".md").unwrap_or(path);
    let path = if path.is_empty() {
        registry.default_path.as_str()
    } else {
        path
    };
    let body = registry.get_doc_content(path).ok_or_else(not_found)?;
    let mut out = String::new();
    if let Some(title) = registry.get_page_title(path) {
        out.push_str(&format!("# {title}\n\n"));
    }
    if let Some(description) = registry.get_page_description(path) {
        out.push_str(&format!("> {description}\n\n"));
    }
    out.push_str(body);
    Ok((
        [(
            header::CONTENT_TYPE,
            HeaderValue::from_static("text/markdown; charset=utf-8"),
        )],
        out,
    ))
}

/// `<link rel="alternate" type="text/markdown">` pointing at the page's
/// `<base_path>/<page>.md` copy (see [`docs_markdown`]); place it in your
/// document's `<head>`. Renders nothing for pages without a Markdown source.
#[component]
pub async fn docs_markdown_link(
    registry: &DocsRegistry,
    base_path: &str,
    path: &str,
) -> Result<impl View> {
    let path = if path.is_empty() {
        registry.default_path.as_str()
    } else {
        path
    };
    let href = registry
        .get_doc_content(path)
        .map(|_| format!("{base_path}/{path}.md"));
    Ok(view! {
        if let Some(href) = href {
            <link rel="alternate" type="text/markdown" href=(href)>
        }
    })
}
