#[macro_use]
#[warn(unused_imports)]
extern crate diesel;
extern crate chrono;

mod models;
mod schema;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use failure::Error;
// use bigdecimal::BigDecimal;

use chrono::{NaiveDate, NaiveDateTime};

use models::{TablaMoneda, NewTablaMoneda};
// use models::{TablaProducto};
use models::{TablaSummarySale};

use schema::tabla_moneda;
use schema::tabla_producto;
use schema::tabla_ventas;

// use mongodb::{Client, options::ClientOptions};

pub struct Conn(PgConnection);

impl Conn {
    pub fn new() -> Result<Self,Error> {
        dotenv::dotenv().ok();
        let db_url = std::env::var("DATABASE_URL")?;
        // println!("db_url: {:?}", db_url);
        Ok(Conn(PgConnection::establish(&db_url)?))
    }
    /*
    pub fn newMongo() -> Result<Self,Error> {
        dotenv::dotenv().ok();
        let db_mongo_uri = std::env::var("DATABASE_MONGO_URI").unwrap()?;
        let mut client_options = ClientOptions::parse(db_mongo_uri)?;
        let client = Client::with_options(client_options)?;
        for db_name in client.list_database_names(None)? {
            println!("{}", db_name);
        }
        Ok(client)
    }
    */
    pub fn put_currency(&self, iso_moneda:Option<String>, descripcion_moneda:Option<String>) -> Result<TablaMoneda, Error> {
        let ncu = NewTablaMoneda{iso_moneda, descripcion_moneda};
        diesel::insert_into(tabla_moneda::table)
            .values(&ncu)
            .get_result(&self.0)
            .map_err(|x| x.into())
    }

    pub fn find_currency(&self, iso_moneda:Option<String>, lim:i64) -> Result<Vec<TablaMoneda>,Error> {
        let lname = format!("%{}%",iso_moneda.unwrap());
        tabla_moneda::table
            .filter(tabla_moneda::iso_moneda.ilike(lname))
            .order(tabla_moneda::id_moneda.desc())
            .limit(lim)
            .load(&self.0)
            .map_err(|e|e.into())
    }

    pub fn set_currency(&self, id:i32, iso_moneda:Option<String>) -> Result<TablaMoneda,Error> {
        diesel::update(tabla_moneda::table::find(tabla_moneda::table,id))
            .set(tabla_moneda::iso_moneda.eq(iso_moneda))
            .get_result(&self.0)
            .map_err(|x| x.into())
    }

    pub fn find_sale(&self,id_producto:Option<i32>, id_moneda:Option<i32>, anho:i32, mes:u32, _usuario:String) -> Result<Vec<TablaSummarySale>, Error> {
        let last_day = get_days_from_month(anho, mes) as u32;
        let dt_start: Option<NaiveDateTime> = Some(NaiveDate::from_ymd(anho, mes, 1).and_hms(0, 0, 0));
        let dt_end: Option<NaiveDateTime> = Some(NaiveDate::from_ymd(anho, mes, last_day).and_hms(0, 0, 0));
        tabla_ventas::table
            .inner_join(tabla_moneda::table)
            .inner_join(tabla_producto::table)
            .select((
                tabla_moneda::descripcion_moneda,
                tabla_producto::descripcion_producto,
                tabla_ventas::monto,
                tabla_ventas::fecha
            ))
            .filter(tabla_ventas::id_producto.eq(id_producto))
            .filter(tabla_ventas::id_moneda.eq(id_moneda))
            .filter(tabla_ventas::fecha.between(dt_start, dt_end))
            .load(&self.0)
            .map_err(|e|e.into())
    }
}

//#[PRIVATE]
fn get_days_from_month(anho: i32, mes: u32) -> i64 {
    NaiveDate::from_ymd(
        match anho {
            12 => anho + 1,
            _ => anho,
        },
        match mes {
            12 => 1,
            _ => mes + 1,
        },
        1,
    )
    .signed_duration_since(NaiveDate::from_ymd(anho, mes, 1))
    .num_days()
}
