//! Actix web Diesel for Sale API
//!
//! This is a spike for test Rust as new backend language

#[macro_use]
extern crate actix_web;

mod request;

use std::{io};
use db_middleware::Conn;
use serde::{Deserialize, Serialize};
use actix_web::{web, middleware, HttpServer, App, Error, HttpResponse};
use request::{Sale, MessageResponse};

#[get("/")]
async fn index(/*item: web::Json<MessageResponse>*/) -> Result<HttpResponse, Error> {
    let item = web::Json(MessageResponse { status: true, data: "Server Running".to_string() });
    println!("{:?}", &item);
    Ok(HttpResponse::Ok().json(item.0))
}

// Find sales in database
#[get("/sales/{id_product}/{id_moneda}/{anho}/{mes}/{usuario}")]
async fn get_sales(
    param: web::Path<(i32, String, i32, u32, String)>
) -> Result<HttpResponse, Error> {
    //- GET PARAMS
    let id_product = param.0;
    let anho = param.2;
    let mes = param.3;
    let usuario = param.4.to_string();
    // VERIFICA SI ES UNA MONEDA PERMITIDA
    let id_moneda = match param.1.as_str() {
        "PEN" => 1,
        "USD" => 2,
        _ => 3
    };
    if id_moneda == 3 {
        let res = web::Json(MessageResponse { status: true, data: "Currency not allowed".to_string() });
        return Ok(HttpResponse::Ok().json(res.0))
    }
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

#[post("/sales")]
async fn get_sales_post(
    form: web::Json<Sale>
) -> Result<HttpResponse, Error> {
    println!("{:?}", &form);
    //- GET PARAMS
    let id_product = form.producto;
    let anho = form.anho;
    let mes = form.mes;
    let usuario = form._usuario.to_string();
    //- VERIFICA SI ES UNA MONEDA PERMITIDA
    let id_moneda = match form.moneda.as_str() {
        "PEN" => 1,
        "USD" => 2,
        _ => 3
    };
    if id_moneda == 3 {
        let res = web::Json(MessageResponse { status: true, data: "Currency not allowed".to_string() });
        return Ok(HttpResponse::Ok().json(res.0))
    }
    //- OPEN CONNECTION
    let conn = Conn::new()?;
    //- MAKE A CONSULT
    let sales = web::block(move || conn.find_sale(Some(id_product), Some(id_moneda), anho, mes, usuario)).await?;

    if sales.len() > 0 {
        let sale_found = HttpResponse::Ok().json(sales);
        println!("{:?}", &sale_found);
        Ok(sale_found)
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
            .service(get_sales_post)
    })
    .workers(4)
    .bind(&bind)?
    .run()
    .await
}
