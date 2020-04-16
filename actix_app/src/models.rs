use crate::schema::currency;
use crate::schema::product;
use crate::schema::sale;
use bigdecimal::BigDecimal;
use std::time::SystemTime;

#[derive(Queryable, Clone, Debug)]
pub struct Currency {
    pub id: i32,
    pub iso_name: String,
    pub description: String
}

#[derive(Queryable, Clone, Debug)]
pub struct Product {
    pub id: i32,
    pub description: String
}

#[derive(Queryable, Clone, Debug)]
pub struct Sale {
    pub id_product: i32,
    pub id_currency: i32,
    pub amount: BigDecimal,
    pub date_sale: SystemTime,
    pub id: i32
}

#[derive(Queryable, Insertable, Clone, Debug)]
#[table_name = "currency"]
pub struct NewCurrency<'a> {
    pub iso_name: &'a str,
    pub description: &'a str
}

#[derive(Queryable, Insertable, Clone, Debug)]
#[table_name = "product"]
pub struct NewProduct<'a> {
    pub description: &'a str
}

#[derive(Queryable, Insertable, Clone, Debug)]
#[table_name = "sale"]
pub struct NewSale {
    pub id_product: i32,
    pub id_currency: i32,
    pub amount: BigDecimal,
    pub date_sale: SystemTime
}