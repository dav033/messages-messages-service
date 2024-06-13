use actix_web::{post, web, Error, HttpResponse, Responder};

use actix::Addr;

use crate::{db_messages::CreateMessage, AppState, DbActor};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateMessageBody {
    pub body: String,
    pub typeM: String,
    pub sender: String,
    pub receiver: String,
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
            receiver: body.receiver.clone(),
            datetime: chrono::Utc::now().naive_utc(),
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
