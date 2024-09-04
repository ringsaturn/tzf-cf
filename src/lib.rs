use axum::{extract::Query, http::StatusCode, response::IntoResponse, routing::get, Json, Router};
use serde::Deserialize;
use serde::Serialize;
use tower_service::Service;
use tzf_rs::gen;
use tzf_rs::Finder;
use worker::*;

async fn new_finder_via_download() -> Finder {
    let url: &str =
        "https://github.com/ringsaturn/tzf-rel-lite/raw/main/combined-with-oceans.reduce.pb";
    let request = Request::new(url, Method::Get).unwrap();
    let mut response = Fetch::Request(request).send().await.unwrap();
    let file_bytes = response.bytes().await.unwrap();
    Finder::from_pb(gen::Timezones::try_from(file_bytes).unwrap_or_default())
}

#[derive(Debug, Deserialize)]
pub struct Params {
    lng: f64,
    lat: f64,
}

#[derive(Serialize)]
pub struct GetTZResponse {
    tz: String,
}

pub async fn get_tz(Query(params): Query<Params> ) -> impl IntoResponse {
    let lng = params.lng;
    let lat = params.lat;

    let f = new_finder_via_download().await;
    let tz = f.get_tz_name(lng, lat);
    let tz = tz.to_string();

    (StatusCode::OK, Json(GetTZResponse { tz }))
}

fn router() -> Router {
    Router::new().route("/", get(get_tz))
}

#[event(fetch)]
pub async fn main(
    req: HttpRequest,
    _env: Env,
    _ctx: Context,
) -> Result<axum::http::Response<axum::body::Body>> {
    console_error_panic_hook::set_once();

    Ok(router().call(req).await?)
}
