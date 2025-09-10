use spin_sdk::http::{send, IntoResponse, Method, Request, RequestBuilder, Response};
use spin_sdk::{http_component, variables};

use crate::ndc::copenhagen::transformer::{self, Header};

wit_bindgen::generate!({
    world: "host",
    path: "../wit/world.wit"
});
#[http_component]
async fn ndc_proxy(req: Request) -> anyhow::Result<impl IntoResponse> {
    let Ok(upstream) = variables::get("upstream") else {
        return Ok(Response::new(500, "upstream not configured"));
    };

    let upstream_request =
        RequestBuilder::new(Method::Get, build_url(upstream, req.path_and_query()))
            .body(())
            .build();
    let upstream_response: Response = send(upstream_request).await?;
    let mut headers = vec![];

    for upstream_response_header in upstream_response.headers() {
        headers.push(Header {
            key: upstream_response_header.0.to_string(),
            value: upstream_response_header
                .1
                .as_str()
                .unwrap_or_default()
                .to_string(),
        });
    }
    let transformed = transformer::transform(headers.as_slice(), upstream_response.body());
    let mut res = Response::new(upstream_response.status().clone(), transformed.1);
    for transformed_header in transformed.0 {
        res.set_header(transformed_header.key, transformed_header.value);
    }
    Ok(res)
}

fn build_url(upstream: String, path_and_query: Option<&str>) -> String {
    let path_and_query = path_and_query.unwrap_or("/");
    format!("{}{}", upstream, path_and_query)
}
