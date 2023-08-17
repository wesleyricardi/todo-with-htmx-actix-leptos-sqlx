use actix_files as fs;
use actix_web::{web, App, HttpServer};
use router::todo::{add_task, delete_task, index, update_task};
use sqlx::{Pool, Postgres};
use std::sync::Mutex;

use crate::database::postgres_pool::get_postgres_pool;

mod components;
mod controller;
mod database;
mod entities;
mod error;
mod model;
mod repository;
mod router;
mod services;
mod utils;
mod view;

pub struct AppState {
    pub counter: Mutex<u8>,
    pub postgres_pool: Pool<Postgres>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let postgres_pool = get_postgres_pool(None).await;

    let app_state = web::Data::new(AppState {
        counter: Mutex::new(0),
        postgres_pool,
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(fs::Files::new("/static", "./static").show_files_listing())
            .service(index)
            .service(add_task)
            .service(update_task)
            .service(delete_task)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
