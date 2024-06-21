use crate::models::*;
use chrono::NaiveDateTime;
use diesel::QueryResult;

use actix::Message as ActixMessage;

#[derive(ActixMessage, Debug)]
#[rtype(result = "QueryResult<MessageResponse>")]
pub struct CreateMessage {
    pub body: String,
    pub typeM: String,
    pub sender: String,
    pub receiver: String,
    pub datetime: NaiveDateTime,
    pub readed: String,
}

#[derive(ActixMessage, Debug, Clone, Copy)]
#[rtype(result = "QueryResult<Vec<MessageResponse>>")]
pub struct GetMessagesByRoom {
    pub room_id: i32,
}

#[derive(ActixMessage, Debug, Clone, Copy)]
#[rtype(result = "QueryResult<MessageResponse>")]
pub struct GetLastMessageByRoom {
    pub room_id: i32,
}

#[derive(ActixMessage, Debug)]
#[rtype(result = "QueryResult<Vec<MessageResponse>>")]
pub struct GetUnreadedMessagesByRoom {
    pub room_id: i32,
    pub user_id: i32,
}

#[derive(ActixMessage, Debug)]
#[rtype(result = "QueryResult<MessagesRoomInformation>")]
pub struct GetMessagesRoomInformation {
    pub room_id: i32,
    pub user_id: i32,
}
