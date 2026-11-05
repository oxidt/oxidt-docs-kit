#![cfg(feature = "mermaid")]

use topcoat::router::{Body, Router, StatusCode, header, request::Request, to_bytes};
use topcoat_docs_kit::diagram_assets;

#[tokio::test]
async fn serves_the_pinned_entry_and_its_chunks() {
    let router = Router::builder().route(diagram_assets).build();
    let response = router
        .handle(
            Request::builder()
                .uri("/_docs/pele/1.6.0/lazy.js")
                .body(Body::empty())
                .unwrap(),
        )
        .await;
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
        response.headers()[header::CONTENT_TYPE],
        "text/javascript; charset=utf-8"
    );
    assert_eq!(
        response.headers()[header::CACHE_CONTROL],
        "public, max-age=31536000, immutable"
    );
    let bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    assert_eq!(&bytes[..], oxidt_mdx::pele_asset("lazy.js").unwrap());
    let entry = std::str::from_utf8(&bytes).unwrap();
    let chunk = entry
        .split("./chunks/")
        .nth(1)
        .unwrap()
        .split('"')
        .next()
        .unwrap();
    let response = router
        .handle(
            Request::builder()
                .uri(format!("/_docs/pele/1.6.0/chunks/{chunk}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await;
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn unknown_versions_and_paths_are_not_served() {
    let router = Router::builder().route(diagram_assets).build();
    for path in [
        "/_docs/pele/0.0.0/lazy.js",
        "/_docs/pele/1.6.0/missing.js",
        "/_docs/pele/1.6.0/../Cargo.toml",
    ] {
        let response = router
            .handle(Request::builder().uri(path).body(Body::empty()).unwrap())
            .await;
        assert_eq!(response.status(), StatusCode::NOT_FOUND, "{path}");
    }
}
