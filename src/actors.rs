use std::fmt::format;

use crate::db_messages::*;
use crate::db_utils::DbActor;
use crate::insertables::CreateMessageExternalApi;
use crate::insertables::NewMessage;
use crate::models::Message;
use crate::schema::messages;
use crate::schema::messages::dsl::*;
use crate::schema::messages::id as message_id;

use actix::Handler;

use diesel::{self, prelude::*};
use reqwest;

impl Handler<CreateMessage> for DbActor {
    type Result = QueryResult<Message>;

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

        result
    }
}

impl Handler<GetMessagesByRoom> for DbActor {
    type Result = QueryResult<Vec<Message>>;

    fn handle(
        &mut self,
        msg: GetMessagesByRoom,
        _: &mut Self::Context,
    ) -> QueryResult<Vec<Message>> {
        let mut conn = self
            .0
            .get()
            .expect("Get messages by room: Error connecting to database");

        let messages_result = messages
            .filter(receiver.eq(&msg.room_id))
            .load::<Message>(&mut conn);

    
        messages_result
    }
}
