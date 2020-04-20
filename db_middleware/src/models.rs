#![allow(unused)]
#![allow(clippy::all)]
extern crate chrono;

use crate::schema::tabla_moneda;
use crate::schema::tabla_producto;
use crate::schema::tabla_ventas;

use chrono::NaiveDateTime;
use bigdecimal::BigDecimal;

#[derive(Queryable, Debug, Identifiable)]
#[table_name = "tabla_moneda"]
#[primary_key(id_moneda)]
pub struct TablaMoneda {
    pub id_moneda: i32,
    pub iso_moneda: Option<String>,
    pub descripcion_moneda: Option<String>,
}

#[derive(Queryable, Debug, Identifiable)]
#[table_name = "tabla_producto"]
#[primary_key(id_producto)]
pub struct TablaProducto {
    pub id_producto: i32,
    pub descripcion_producto: Option<String>,
}

#[derive(Queryable, Debug, Identifiable)]
#[table_name = "tabla_ventas"]
#[primary_key(id_venta)]
pub struct TablaVentas {
    pub id_venta: i32,
    pub id_producto: Option<i32>,
    pub id_moneda: Option<i32>,
    pub monto: Option<BigDecimal>,
    pub fecha: Option<NaiveDateTime>,
}

#[derive(Queryable, Insertable, Clone, Debug)]
#[table_name = "tabla_moneda"]
pub struct NewTablaMoneda {
    pub iso_moneda: Option<String>,
    pub descripcion_moneda: Option<String>
}
