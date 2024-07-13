use actix::Addr;
use actix_web::{
    get, post, put, web,
    web::{Data, Json, Path},
    Error, HttpRequest, HttpResponse, Responder,
};
use serde::Deserialize;

use crate::{db_messages::*, AppState, DbActor};

#[derive(Deserialize)]
pub struct CreateMessageBody {
    pub body: String,
    pub typeM: String,
    pub sender: String,
    pub sender_name: String,
    pub receiver: String,
}

#[derive(Deserialize)]
pub struct UpdateMessagesUsernameBody {
    pub username: String,
}

#[post("/messages")]
pub async fn send_messages(
    state: web::Data<AppState>,
    body: web::Json<CreateMessageBody>,
) -> impl Responder {
    let db: Addr<DbActor> = state.db.clone();

    match db
        .send(CreateMessage {
            body: body.body.clone(),
            typeM: body.typeM.clone(),
            sender: body.sender.clone(),
            sender_name: body.sender_name.clone(),
            receiver: body.receiver.clone(),
            datetime: chrono::Utc::now().naive_utc(),
            readed: body.receiver.clone(),
        })
        .await
    {
        Ok(Ok(info)) => HttpResponse::Ok().json(info),
        Ok(Err(e)) => {
            eprintln!("Database error: {}", e);
            HttpResponse::InternalServerError().json("Failed to get user rooms")
        }
        Err(e) => {
            eprintln!("Mailbox error: {}", e);
            HttpResponse::InternalServerError().json("Failed to send message to database")
        }
    }
}

#[get("/messages/{room_id}")]
pub async fn get_messages_by_room(
    state: web::Data<AppState>,
    room_id: Path<i32>,
) -> impl Responder {
    let db: Addr<DbActor> = state.db.clone();
    let room_id = room_id.into_inner();

    match db.send(GetMessagesByRoom { room_id }).await {
        Ok(Ok(info)) => HttpResponse::Ok().json(info),
        Ok(Err(e)) => {
            eprintln!("Database error: {}", e);
            HttpResponse::InternalServerError().json("Failed to get user rooms")
        }
        Err(e) => {
            eprintln!("Mailbox error: {}", e);
            HttpResponse::InternalServerError().json("Failed to send message to database")
        }
    }
}

#[get("/messages/{room_id}/unreaded/{user_id}")]
pub async fn get_unreaded_messages_by_room(
    state: web::Data<AppState>,
    path: Path<(i32, i32)>,
) -> impl Responder {
    let db: Addr<DbActor> = state.db.clone();

    let (room_id, user_id) = path.into_inner();

    match db
        .send(GetUnreadedMessagesByRoom { room_id, user_id })
        .await
    {
        Ok(Ok(info)) => HttpResponse::Ok().json(info),
        Ok(Err(e)) => {
            eprintln!("Database error: {}", e);
            HttpResponse::InternalServerError().json("Failed to get user rooms")
        }
        Err(e) => {
            eprintln!("Mailbox error: {}", e);
            HttpResponse::InternalServerError().json("Failed to send message to database")
        }
    }
}

#[get("/messages/{room_id}/last")]
pub async fn get_last_message_by_room(
    state: web::Data<AppState>,
    room_id: Path<i32>,
) -> impl Responder {
    let db: Addr<DbActor> = state.db.clone();
    let room_id = room_id.into_inner();

    match db.send(GetLastMessageByRoom { room_id }).await {
        Ok(Ok(info)) => HttpResponse::Ok().json(info),
        Ok(Err(e)) => {
            eprintln!("Database error: {}", e);
            HttpResponse::InternalServerError().json("Failed to get user rooms")
        }
        Err(e) => {
            eprintln!("Mailbox error: {}", e);
            HttpResponse::InternalServerError().json("Failed to send message to database")
        }
    }
}

#[get("/messages/{room_id}/info/{user_id}")]
pub async fn get_messages_room_information(
    state: Data<AppState>,
    path: Path<(i32, i32)>,
) -> impl Responder {
    let db: Addr<DbActor> = state.db.clone();

    let (room_id, user_id) = path.into_inner();

    match db
        .send(GetMessagesRoomInformation { room_id, user_id })
        .await
    {
        Ok(Ok(info)) => HttpResponse::Ok().json(info),
        Ok(Err(e)) => {
            eprintln!("Database error: {}", e);
            HttpResponse::InternalServerError().json("Failed to get user rooms")
        }
        Err(e) => {
            eprintln!("Mailbox error: {}", e);
            HttpResponse::InternalServerError().json("Failed to send message to database")
        }
    }
}

#[put("/messages/set_readed/{room_id}/{user_id}")]
pub async fn set_readed(state: Data<AppState>, path: Path<(i32, i32)>) -> impl Responder {
    let db: Addr<DbActor> = state.db.clone();

    let (room_id, user_id) = path.into_inner();

    match db.send(SetReaded { room_id, user_id }).await {
        Ok(Ok(info)) => HttpResponse::Ok().json(info),
        Ok(Err(e)) => {
            eprintln!("Database error: {}", e);
            HttpResponse::InternalServerError().json("Failed to get user rooms")
        }
        Err(e) => {
            eprintln!("Mailbox error: {}", e);
            HttpResponse::InternalServerError().json("Failed to send message to database")
        }
    }
}

#[put("/messages/update_username/{user_id}")]
pub async fn update_username(
    state: Data<AppState>,
    path: Path<i32>,
    body: Json<UpdateMessagesUsernameBody>,
) -> impl Responder {
    let db: Addr<DbActor> = state.db.clone();

    let user_id = path.into_inner();

    match db
        .send(UpdateMessagesUsername {
            user_id,
            username: body.into_inner().username,
        })
        .await
    {
        Ok(Ok(info)) => HttpResponse::Ok().json(info),
        Ok(Err(e)) => {
            eprintln!("Database error: {}", e);
            HttpResponse::InternalServerError().json("Failed to get user rooms")
        }
        Err(e) => {
            eprintln!("Mailbox error: {}", e);
            HttpResponse::InternalServerError().json("Failed to send message to database")
        }
    }
}
