use bytes::Bytes;
use lol_html::{RewriteStrSettings, element, rewrite_str};
use spin_sdk::http::body::IncomingBodyExt;
use spin_sdk::http::{EmptyBody, FullBody, IntoResponse, Method, Request, Response, send};
use spin_sdk::{http_service, variables};

fn rewrite_for_bots(html: &str) -> anyhow::Result<String> {
    let settings = RewriteStrSettings {
        element_content_handlers: vec![
            element!(".price_color", |el| {
                el.set_inner_content(
                    "Contact Sales",
                    lol_html::html_content::ContentType::Text,
                );
                Ok(())
            }),
            element!(".availability", |el| {
                el.remove();
                Ok(())
            }),
        ],
        ..RewriteStrSettings::default()
    };
    Ok(rewrite_str(html, settings)?)
}

#[http_service]
async fn handle_html_rewrite(_req: Request) -> anyhow::Result<impl IntoResponse> {
  
    let Ok(upstream_url) = variables::get("upstream_url").await else {
        return Ok(Response::builder()
            .status(500)
            .body(FullBody::new(Bytes::from("Invalid App Config"))));
    };

    let origin_req = Request::builder()
        .method(Method::GET)
        .uri(&upstream_url)
        .body(EmptyBody::new())?;
    let origin_res: Response = send(origin_req).await?;

    let status = origin_res.status();
    let content_type = origin_res
        .headers()
        .get("content-type")
        .cloned();

    let body_bytes = origin_res.into_body().bytes().await?;

    let is_html = content_type
        .as_ref()
        .and_then(|v| v.to_str().ok())
        .map(|ct| ct.starts_with("text/html"))
        .unwrap_or(true);

    if is_html {
        let html = String::from_utf8_lossy(&body_bytes);
        // 3. Rewrite sensitive nodes before returning to callee
        let protected = rewrite_for_bots(&html)?;

        Ok(Response::builder()
            .status(status)
            .header("content-type", "text/html; charset=utf-8")
            .body(FullBody::new(Bytes::from(protected))))
    } else {
        let mut builder = Response::builder().status(status);
        if let Some(ct) = content_type {
            builder = builder.header("content-type", ct);
        }
        Ok(builder.body(FullBody::new(body_bytes)))
    }
}
