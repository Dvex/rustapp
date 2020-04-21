#[macro_use]
extern crate actix_web;

use std::{io};
use db_middleware::Conn;

// use actix_web::http::{header, Method, StatusCode};
use actix_web::{web, middleware, HttpServer, App, Error, HttpResponse};

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
    // std::env::set_var("RUST_LOG", "actix_web=debug,diesel=debug");
    env_logger::init();
    let bind = "127.0.0.1:8080";
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .service(get_sales)
    })
    .bind(&bind)?
    .run()
    .await
}
