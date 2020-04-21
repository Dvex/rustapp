//! Actix web Diesel for Sale API
//!
//! This is a spike for test Rust as new backend language

#[macro_use]
extern crate actix_web;

use std::{io};
use db_middleware::Conn;
use serde::{Deserialize, Serialize};

use actix_web::{web, middleware, HttpServer, App, Error, HttpResponse};

#[derive(Debug, Serialize, Deserialize)]
struct MessageResponse {
    status: bool,
    data: String
}

#[get("/")]
async fn index(item: web::Json<MessageResponse>) -> HttpResponse {
    println!("model: {:?}", &item);
    HttpResponse::Ok().json(item.0)
}

// Find sales in database
#[get("/sales/{id_product}/{id_moneda}/{anho}/{mes}/{usuario}")]
async fn get_sales(
    param: web::Path<(i32, i32, i32, u32, String)>
) -> Result<HttpResponse, Error> {
    //- GET PARAMS
    let id_product = param.0;
    let id_moneda = param.1;
    let anho = param.2;
    let mes = param.3;
    let usuario = param.4.to_string();
    //- OPEN CONNECTION
    let conn = Conn::new()?;
    //- MAKE A CONSULT
    let sales = web::block(move || conn.find_sale(Some(id_product), Some(id_moneda), anho, mes, usuario)).await?;
    
    if sales.len() > 0 {
        Ok(HttpResponse::Ok().json(sales))
    } else {
        let res = HttpResponse::NotFound()
            .body(format!("No sales found"));
        Ok(res)
    }
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info,diesel=debug");
    dotenv::dotenv().ok();
    env_logger::init();

    let bind = std::env::var("BIND").unwrap();

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .service(index)
            .service(get_sales)
    })
    .workers(4)
    .bind(&bind)?
    .run()
    .await
}
