// message.rs
use chrono::NaiveDateTime; // Importa el tipo de fecha y hora
use diesel::Queryable;
use serde::Serialize;

use crate::schema;
// Asegúrate de que el nombre del esquema sea correcto

#[derive(Queryable, Serialize, Debug, Clone)]
pub struct Message {
    pub id: i32,
    pub body: String,
    pub typeM: String,
    pub datetime: String,
    pub sender: String,
    pub sender_name: String,
    pub receiver: String,
    pub readed: String,
}

#[derive(Queryable, Serialize, Debug)]
pub struct MessageResponse {
    pub id: i32,
    pub body: String,
    pub typeM: String,
    pub datetime: String,
    pub sender_name: String,
    pub sender: String,
    pub receiver: String,
    pub readed: Vec<i32>,
}

#[derive(Queryable, Serialize, Debug)]

pub struct MessagesRoomInformation {
    pub last_message: Option<MessageResponse>,
    pub unreaded_messages: Vec<MessageResponse>,
}

impl Message {
    pub fn get_readed(&self) -> Vec<i32> {
        serde_json::from_str(&self.readed).unwrap_or_else(|_| vec![])
    }

    pub fn set_readed(&mut self, readed: Vec<i32>) {
        self.readed = serde_json::to_string(&readed).unwrap();
    }
}

impl From<Message> for MessageResponse {
    fn from(message: Message) -> Self {
        MessageResponse {
            id: message.id,
            body: message.body,
            typeM: message.typeM,
            datetime: message.datetime,
            sender: message.sender,
            sender_name: message.sender_name,
            receiver: message.receiver,
            readed: serde_json::from_str(&message.readed).unwrap_or_else(|_| vec![]),
        }
    }
}
