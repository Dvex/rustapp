#[macro_use]
extern crate diesel;
mod models;
mod schema;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use failure::Error;
use bigdecimal::BigDecimal;
use std::time::SystemTime;

use models::{Currency, NewCurrency};
use models::{Product, NewProduct};
use models::{Sale, NewSale};

use schema::currency;
use schema::product;
use schema::sale;

pub struct Conn(PgConnection);

impl Conn {
    pub fn new() -> Result<Self,Error>{
        dotenv::dotenv().ok();
        let db_url = std::env::var("DATABASE_URL")?;
        Ok(Conn(PgConnection::establish(&db_url)?))
    }

    pub fn put_currency(&self, iso_name:&str, description:&str) -> Result<Currency, Error> {
        let ncu = NewCurrency{iso_name, description};
        diesel::insert_into(currency::table)
            .values(&ncu)
            .get_result(&self.0)
            .map_err(|x| x.into())
    }

    pub fn put_product(&self, description:&str) -> Result<Product, Error> {
        let ncu = NewProduct{description};
        diesel::insert_into(product::table)
            .values(&ncu)
            .get_result(&self.0)
            .map_err(|x| x.into())
    }

    pub fn put_sale(&self, id_product:i32, id_currency:i32, amount:BigDecimal) -> Result<Sale, Error> {
        let date_sale = SystemTime::now();
        let ncu = NewSale{id_product, id_currency, amount, date_sale};
        diesel::insert_into(sale::table)
            .values(&ncu)
            .get_result(&self.0)
            .map_err(|x| x.into())
    }
}