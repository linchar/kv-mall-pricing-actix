use actix_web::{web, App, HttpServer, Responder, Result};
use rand::Rng;
use serde::{Deserialize, Serialize};
use tokio::task;

use opentelemetry::{
    global,
    trace::{TraceContextExt, Tracer},
    Context,
};
// use opentelemetry_http::{Bytes, HeaderExtractor};
use actix_web_opentelemetry::RequestTracing;

mod telemetry;

#[derive(Serialize, Debug)]
pub struct PriceResult {
    id: i32,
    price: f64,
}

impl PriceResult {
    fn new(id: i32, price: f64) -> Self {
        Self { id, price }
    }
}

fn get_price_from_db(cx: Context, id: i32) -> PriceResult {
    let tracer = global::tracer("request");
    let _child = tracer.start_with_context("get_price_from_db", &cx);

    println!("getPriceFromDb thread: {:?}", std::thread::current().id());
    let mut rng = rand::thread_rng();
    let price: f64 = rng.gen_range(1.0..51.0);
    let price: f64 = (price * 100.0).round() / 100.0;
    PriceResult::new(id, price)
}

#[derive(Deserialize)]
struct PriceQuery {
    id: i32,
}

async fn get_price(query: web::Query<PriceQuery>) -> Result<impl Responder> {
    let tracer = global::tracer("request");

    let parent = tracer.start("get_price");
    let parent_cx = Context::current_with_span(parent);

    println!("Current thread: {:?}", std::thread::current().id());
    let id = query.id;
    let price_result = task::spawn_blocking(move || get_price_from_db(parent_cx, id))
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    Ok(web::Json(price_result))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    telemetry::init_tracer();

    HttpServer::new(|| {
        App::new()
            .wrap(RequestTracing::new())
            .service(web::resource("/price").route(web::get().to(get_price)))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await?;

    // Ensure all spans have been reported
    global::shutdown_tracer_provider();

    Ok(())
}
