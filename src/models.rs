// message.rs
use chrono::NaiveDateTime; // Importa el tipo de fecha y hora
use diesel::Queryable;
use serde::{ Serialize};

use crate::schema;
// Asegúrate de que el nombre del esquema sea correcto

#[derive(Queryable, Serialize, Debug)]
pub struct Message { 
    pub id: i32,
    pub body: String,
    pub typeM: String,
    pub datetime: String,
    pub sender: String,
    pub receiver: String,
}

// #[derive(Insertable)]
// #[table_name = "messages"]
// pub struct NewMessage {
//     pub body: String,
//     pub typeM: String,
//     pub datetime: NaiveDateTime,
//     pub sender: String,
