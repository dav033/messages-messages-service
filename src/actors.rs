use std::fmt::format;

use crate::db_messages::*;
use crate::db_utils::DbActor;
use crate::insertables::CreateMessageExternalApi;
use crate::insertables::NewMessage;
use crate::models::Message;
use crate::models::MessageResponse;
use crate::models::MessagesRoomInformation;
use crate::schema::messages::dsl::*;
use crate::schema::messages::id as message_id;
use crate::schema::messages::receiver;

use actix::Handler;

use diesel::{self, prelude::*};
use reqwest;

impl Handler<CreateMessage> for DbActor {
    type Result = QueryResult<MessageResponse>;

    fn handle(&mut self, msg: CreateMessage, _ctx: &mut Self::Context) -> Self::Result {
        let mut conn = self
            .0
            .get()
            .expect("Create message: Error connecting to database");

        println!("Create message: {:?}", msg);
        let new_message = NewMessage {
            body: msg.body,
            typeM: msg.typeM,
            datetime: msg.datetime.format("%Y-%m-%d %H:%M:%S").to_string(),
            sender: msg.sender,
            receiver: msg.receiver.clone(),
            readed: msg.readed.clone(),
        };

        println!("New message: {:?}", &new_message);

        let reult_inser = diesel::insert_into(messages)
            .values(&new_message)
            .execute(&mut conn);

        if let Err(err) = reult_inser {
            println!("Error inserting message: {:?}", err);
        }

        let result: Result<Message, diesel::result::Error> =
            messages.order(message_id.desc()).first(&mut conn);

        let room = msg.receiver.clone();

        let new_external_message = CreateMessageExternalApi {
            message_id: result.as_ref().unwrap().id,
        };

        let client = reqwest::blocking::Client::new();
        let response = client
            .put(format!("http://localhost:8080/create_message/{}", room).as_str())
            .json(&new_external_message)
            .send()
            .expect("Error sending POST request to external app");

        if response.status().is_success() {
            println!("Message successfully sent to external app");
        } else {
            println!("Failed to send message to external app: {:?}", response);
        }

        result.map(MessageResponse::from)
    }
}

impl Handler<GetMessagesByRoom> for DbActor {
    type Result = QueryResult<Vec<MessageResponse>>;

    fn handle(
        &mut self,
        msg: GetMessagesByRoom,
        _: &mut Self::Context,
    ) -> QueryResult<Vec<MessageResponse>> {
        let mut conn = self
            .0
            .get()
            .expect("Get messages by room: Error connecting to database");

        let messages_result: Result<Vec<Message>, diesel::result::Error> = messages
            .filter(receiver.like(format!("%{}%", msg.room_id)))
            .load::<Message>(&mut conn);

        messages_result.map(|messages_result| {
            messages_result
                .into_iter()
                .map(MessageResponse::from)
                .collect()
        })

        // messages_result;
    }
}

impl Handler<GetUnreadedMessagesByRoom> for DbActor {
    type Result = QueryResult<Vec<MessageResponse>>;

    fn handle(
        &mut self,
        msg: GetUnreadedMessagesByRoom,
        _: &mut Self::Context,
    ) -> QueryResult<Vec<MessageResponse>> {
        let mut conn = self
            .0
            .get()
            .expect("Get unreaded messages by room: Error connecting to database");

        let messages_result: Result<Vec<Message>, diesel::result::Error> = messages
            .filter(receiver.like(format!("%{}%", msg.room_id)))
            .filter(readed.not_like(format!("%{}%", msg.user_id)))
            .filter(sender.ne(msg.user_id.to_string())) // Filtro adicional para excluir mensajes del propio usuario
            .load::<Message>(&mut conn);

        messages_result.map(|messages_result| {
            messages_result
                .into_iter()
                .map(MessageResponse::from)
                .collect()
        })
    }
}

impl Handler<GetLastMessageByRoom> for DbActor {
    type Result = QueryResult<MessageResponse>;

    fn handle(
        &mut self,
        msg: GetLastMessageByRoom,
        _: &mut Self::Context,
    ) -> QueryResult<MessageResponse> {
        let mut conn = self
            .0
            .get()
            .expect("Get last message by room: Error connecting to database");

        let message_result: Result<Message, diesel::result::Error> = messages
            .filter(receiver.like(format!("%{}%", msg.room_id)))
            .order(message_id.desc())
            .first(&mut conn);

        message_result.map(MessageResponse::from)
    }
}

impl Handler<GetMessagesRoomInformation> for DbActor {
    type Result = QueryResult<MessagesRoomInformation>;

    fn handle(
        &mut self,
        msg: GetMessagesRoomInformation,
        _: &mut Self::Context,
    ) -> QueryResult<MessagesRoomInformation> {
        let mut conn = self
            .0
            .get()
            .expect("Get messages by room: Error connecting to database");

        let unreaded_messages_result: Result<Vec<Message>, diesel::result::Error> = messages
            .filter(receiver.like(format!("%{}%", msg.room_id)))
            .filter(readed.not_like(format!("%{}%", msg.user_id)))
            .filter(sender.ne(msg.user_id.to_string())) // Filtro adicional para excluir mensajes del propio usuario
            .load::<Message>(&mut conn)
            .map_err(|e| e.into());

        let unreaded_messages = unreaded_messages_result
            .map(|messages_result| {
                messages_result
                    .into_iter()
                    .map(MessageResponse::from)
                    .collect::<Vec<MessageResponse>>()
            })
            .unwrap_or_else(|_| Vec::new());

        // Obtener el último mensaje
        let last_message_result: Result<Message, diesel::result::Error> = messages
            .filter(receiver.like(format!("%{}%", msg.room_id)))
            .order(message_id.desc())
            .first(&mut conn);

        let last_message = match last_message_result {
            Ok(message) => Some(MessageResponse::from(message)),
            Err(diesel::result::Error::NotFound) => None, // Si no hay mensajes, devolver None
            Err(e) => return Err(e),
        };

        Ok(MessagesRoomInformation {
            last_message,
            unreaded_messages,
        })
    }
}