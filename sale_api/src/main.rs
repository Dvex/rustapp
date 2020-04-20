#[macro_use]
extern crate actix_web;

use std::{io};
use db_middleware::Conn;

// use actix_web::http::{header, Method, StatusCode};
use actix_web::{web, middleware, HttpServer, App, Error, HttpResponse};

#[get("/sales/{id_product}/{id_moneda}/{anho}/{mes}/{usuario}")]
async fn get_sales(
    id_product: web::Path<i32>,
    id_moneda: web::Path<i32>,
    anho: web::Path<i32>,
    mes: web::Path<u32>,
    usuario: web::Path<String>
) -> Result<HttpResponse, Error> {
    //- GET PARAMS
    let id_product = id_product.into_inner();
    let id_moneda = id_moneda.into_inner();
    let anho = anho.into_inner();
    let mes = mes.into_inner();
    let usuario = usuario.into_inner();
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
