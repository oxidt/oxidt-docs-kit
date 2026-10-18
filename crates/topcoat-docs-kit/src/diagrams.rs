//! Self-hosted Pele modules; register `diagram_assets` on the app router.

use topcoat::{
    Result,
    context::Cx,
    router::{
        CatchAllSegments, HeaderValue, error::not_found, header, path_param,
        response::IntoResponse, route,
    },
};

path_param!(pele_version);
path_param!(*pele_path);

/// Serve bundled diagram modules without a CDN or runtime filesystem access.
/// Register with `Router::builder().route(topcoat_docs_kit::diagram_assets)`.
#[route(GET "/_docs/pele/{pele_version}/{*pele_path}")]
pub async fn diagram_assets(cx: &Cx) -> Result<impl IntoResponse> {
    if path_param::<PeleVersion>(cx) != dioxus_mdx::PELE_VERSION {
        return Err(not_found().into());
    }
    let segments: CatchAllSegments<'_> = path_param::<PelePath>(cx);
    let path = segments.collect::<Vec<_>>().join("/");
    let body = dioxus_mdx::pele_asset(&path).ok_or_else(not_found)?;
    Ok((
        [
            (
                header::CONTENT_TYPE,
                HeaderValue::from_static("text/javascript; charset=utf-8"),
            ),
            (
                header::CACHE_CONTROL,
                HeaderValue::from_static("public, max-age=31536000, immutable"),
            ),
        ],
        body,
    ))
}
