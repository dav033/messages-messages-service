use crate::schema::messages;
use diesel::Insertable;
use serde::Serialize;

#[derive(Insertable, Serialize, Clone, Debug)]
#[diesel(table_name=messages)]
pub struct NewMessage {
    pub body: String,
    pub typeM: String,
    pub datetime: String,
    pub sender: String,
    pub receiver: String,
}

#[derive(Serialize, Clone)]
pub struct CreateMessageExternalApi {
    pub message_id: i32,
}


