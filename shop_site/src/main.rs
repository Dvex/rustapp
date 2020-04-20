#[macro_use]
extern crate actix_web;

use std::{io};

use actix_files as fs;
use actix_web::http::{header, Method, StatusCode};
use actix_web::{web,HttpServer,App,HttpResponse,HttpRequest};

#[actix_rt::main]
async fn main() -> io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(fs::Files::new("/site/static", "static"))
            .service(web::resource("/").route(web::get().to(|req: HttpRequest| {
                println!("{:?}", req);
                HttpResponse::Found()
                    .header(header::LOCATION, "static/index.html")
                    .finish()
            })))
        })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
