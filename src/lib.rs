use axum::{extract::Query, routing::get, Router};
use lazy_static::lazy_static;
use serde::Deserialize;
use tower_service::Service;
use tzf_rs::DefaultFinder;
use worker::*;

lazy_static! {
    static ref FINDER: DefaultFinder = DefaultFinder::new();
}

fn router() -> Router {
    Router::new().route("/", get(get_tz))
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
    let lng = params.lng;
    let lat = params.lat;
    // let lng = 116.4074;
    // let lat = 39.9042;
    let tz = FINDER.get_tz_name(lng, lat);
    tz.to_string()
}
