#[macro_use]
extern crate diesel;

use actix_web::{web, App, HttpServer};
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

mod db;
mod models;
mod schema;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let database_url = env::var("postgresql://postgres:6sF3P2WCloWw60AV@db.bsegypiunotvcwatrxgy.supabase.co:5432/postgres").expect("DATABASE_URL must be set");
    let pool = db::establish_connection(&database_url);

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            // Add your Actix web routes and handlers here
            // For example:
            // .service(web::resource("/users").route(web::get().to(users_handler)))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
