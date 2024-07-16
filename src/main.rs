use actix_web::{web, App, HttpServer, Responder, Result};
use rand::Rng;
use serde::{Deserialize, Serialize};
use tokio::task;

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

fn get_price_from_db(id: i32) -> PriceResult {
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
    println!("Current thread: {:?}", std::thread::current().id());
    let id = query.id;
    let price_result = task::spawn_blocking(move || get_price_from_db(id))
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    Ok(web::Json(price_result))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(web::resource("/price").route(web::get().to(get_price))))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
