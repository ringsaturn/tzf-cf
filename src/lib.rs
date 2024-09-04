use axum::{extract::Query, routing::get, Router};
use reqwest;
use serde::Deserialize;
use tower_service::Service;
use tzf_rs::gen;
use tzf_rs::Finder;
use worker::*;

fn router() -> Router {
    Router::new().route("/", get(get_tz))
}

async fn new_finder_via_download() -> Finder {
    let download_url: &str =
        "https://github.com/ringsaturn/tzf-rel-lite/raw/main/combined-with-oceans.reduce.pb";
    let resp: reqwest::Response = reqwest::get(download_url).await.unwrap();
    let file_bytes: axum::body::Bytes = resp.bytes().await.unwrap();
    let file_bytes: Vec<u8> = file_bytes.to_vec();
    Finder::from_pb(gen::Timezones::try_from(file_bytes).unwrap_or_default())
}

#[event(fetch)]
async fn fetch(
    req: HttpRequest,
    _env: Env,
    _ctx: Context,
) -> Result<axum::http::Response<axum::body::Body>> {
    console_error_panic_hook::set_once();

    Ok(router().call(req).await?)
}

#[derive(Debug, Deserialize)]
pub struct Params {
    lng: f64,
    lat: f64,
}

pub async fn get_tz(Query(params): Query<Params>) -> String {

    // let lng = 116.4074;
    // let lat = 39.9042;
    // let tz = "{lng},{lat}""
    // let tz = format!("{lng},{lat}");
    let lng = params.lng;
    let lat = params.lat;
    let f = new_finder_via_download().await;

    let tz = f.get_tz_name(lng, lat);
    tz.to_string()
}
