//! `docs_markdown` behind a route, and the `<head>` link pointing at it.

use oxidt_docs_kit_core::{DocsConfig, DocsRegistry};
use std::sync::LazyLock;
use topcoat::{
    Result,
    context::Cx,
    router::{
        Body, CatchAllSegments, Router, StatusCode, header, path_param, request::Request,
        response::IntoResponse, route, to_bytes,
    },
    view::{View, ViewExt, view},
};
use topcoat_docs_kit::{docs_markdown, docs_markdown_link};

const NAV: &str = r#"{
    "groups": [
        { "group": "Guide", "pages": ["guide/intro", "guide/bare"] },
        { "group": "API", "pages": ["api/overview"] }
    ]
}"#;
const INTRO: &str =
    "---\ntitle: Intro\ndescription: The first page\n---\n\n# Intro\n\nSome **prose**.\n";
const BARE: &str = "Just text.\n";
const OVERVIEW: &str = "---\ntitle: API Overview\n---\n\nEndpoints below.\n";
const SPEC: &str = r#"
openapi: "3.0.0"
info: { title: Pets, version: "1" }
paths:
  /pets:
    get:
      operationId: listPets
      responses: { "200": { description: OK } }
"#;

static DOCS: LazyLock<DocsRegistry> = LazyLock::new(|| {
    let bundle = dioxus_docs_kit_build::docs_bundle_json(
        NAV,
        &[
            ("guide/intro", INTRO),
            ("guide/bare", BARE),
            ("api/overview", OVERVIEW),
        ],
        &[("api", SPEC)],
    )
    .expect("generate bundle")
    .leak();
    DocsConfig::new(bundle).build()
});

path_param!(*doc_path);

#[route(GET "/_docs/md/{*doc_path}")]
async fn docs_md(cx: &Cx) -> Result<impl IntoResponse> {
    let segments: CatchAllSegments<'_> = path_param::<DocPath>(cx);
    docs_markdown(&DOCS, &segments.collect::<Vec<_>>().join("/"))
}

async fn get(path: &str) -> (StatusCode, Option<String>, String) {
    let router = Router::builder().route(docs_md).build();
    let response = router
        .handle(Request::builder().uri(path).body(Body::empty()).unwrap())
        .await;
    let status = response.status();
    let content_type = response
        .headers()
        .get(header::CONTENT_TYPE)
        .map(|v| v.to_str().unwrap().to_owned());
    let bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    (
        status,
        content_type,
        String::from_utf8(bytes.to_vec()).unwrap(),
    )
}

#[tokio::test]
async fn serves_markdown_with_title_and_description() {
    let (status, content_type, body) = get("/_docs/md/guide/intro.md").await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(
        content_type.as_deref(),
        Some("text/markdown; charset=utf-8")
    );
    assert!(
        body.starts_with("# Intro\n\n> The first page\n\nSome **prose**."),
        "{body}"
    );
    // The source's own leading H1 is not repeated.
    assert_eq!(body.matches("# Intro").count(), 1, "{body}");
}

#[tokio::test]
async fn suffix_is_optional_and_missing_frontmatter_is_fine() {
    let (status, _, body) = get("/_docs/md/guide/bare").await;
    assert_eq!(status, StatusCode::OK);
    assert!(body.contains("Just text."), "{body}");
    assert!(!body.contains("> "), "{body}");
}

#[tokio::test]
async fn api_pages_and_unknown_paths_are_not_found() {
    for path in ["/_docs/md/api/list-pets.md", "/_docs/md/missing.md"] {
        let (status, _, _) = get(path).await;
        assert_eq!(status, StatusCode::NOT_FOUND, "{path}");
    }
}

async fn render(v: impl View) -> String {
    v.single().await.unwrap().render(&Cx::default())
}

#[tokio::test]
async fn head_link_only_for_pages_with_markdown() {
    let cx = &Cx::default();
    let html = render(
        view! { cx => docs_markdown_link(registry: &DOCS, base_path: "/docs", path: "guide/intro") },
    )
    .await;
    assert!(
        html.contains(r#"rel="alternate" type="text/markdown" href="/docs/guide/intro.md""#),
        "{html}"
    );
    let html = render(
        view! { cx => docs_markdown_link(registry: &DOCS, base_path: "/docs", path: "api/list-pets") },
    )
    .await;
    assert!(!html.contains("<link"), "{html}");
}
