#![allow(unused)]
#![allow(clippy::all)]

use serde::{Deserialize, Serialize};
use serde_derive::*;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Sale {
    pub producto: i32,
    pub moneda: String,
    pub anho: i32,
    pub mes: u32,
    pub _usuario: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageResponse {
    status: bool,
    data: String
}