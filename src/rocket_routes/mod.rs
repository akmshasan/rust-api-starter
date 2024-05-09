pub mod fruits;

use rocket::{http::Status, response::status::Custom};
use serde_json::{json, Value};
use std::error::Error;

use rocket_db_pools::Database;

#[derive(Database)]
#[database("postgres")]
pub struct DbConn(rocket_db_pools::diesel::PgPool);

pub fn server_error(e: Box<dyn Error>) -> Custom<Value> {
    rocket::error!("{}", e);
    Custom(Status::InternalServerError, json!("Error"))
}
