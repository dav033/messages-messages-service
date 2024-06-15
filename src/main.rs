use actix::SyncArbiter;
use dotenv::dotenv;
use std::env;
mod actors;
mod db_messages;
mod db_utils;
mod insertables;
mod models;
mod schema; 
mod services;
use actix_web::{http, web};
use diesel::{
    r2d2::{ConnectionManager, Pool},
    MysqlConnection,
};
use actix_web::{App, HttpServer};
use services::{get_messages_by_room, send_messages};
use db_utils::{get_pool, AppState, DbActor};
use actix_cors::Cors;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let db_url: String = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    //print
    println!("DATABASE_URL: {}", db_url);

    let pool: Pool<ConnectionManager<MysqlConnection>> = get_pool(&db_url);
    let db_addr = SyncArbiter::start(5, move || DbActor(pool.clone()));

    HttpServer::new(move || {

        let cors = Cors::default()
        .allowed_origin("http://localhost:3000")
        .allowed_origin("http://localhost:8082")
        .allowed_origin("http://localhost:8000")
        .allowed_methods(vec!["GET", "POST", "PUT"])
        .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
        .allowed_header(http::header::CONTENT_TYPE)
        .max_age(3600);
    
        App::new()
            .app_data(web::Data::new(AppState {
                db: db_addr.clone(),
            }))
            .wrap(cors)
            .service(send_messages)
            .service(get_messages_by_room)
    })
    .bind(("127.0.0.1", 8082))?
    .run()
    .await
}
