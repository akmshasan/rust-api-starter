use rocket::{
    http::Status,
    response::status::{Custom, NoContent},
    serde::json::Json,
};
use rocket_db_pools::Connection;
use serde_json::{json, Value};

use crate::models::{NewFruit, UpdateFruit};
use crate::repositories::FruitRepository;
use crate::rocket_routes::{server_error, DbConn};

#[rocket::get("/fruits")]
pub async fn get_fruits(mut db: Connection<DbConn>) -> Result<Custom<Value>, Custom<Value>> {
    FruitRepository::get_all(&mut db, 100)
        .await
        .map(|fruits| Custom(Status::Ok, json!(fruits)))
        .map_err(|e| server_error(e.into()))
}

#[rocket::get("/fruits/<fruit_id>")]
pub async fn get_fruit(
    mut db: Connection<DbConn>,
    fruit_id: i64,
) -> Result<Custom<Value>, Custom<Value>> {
    FruitRepository::get(&mut db, fruit_id)
        .await
        .map(|fruits| Custom(Status::Ok, json!(fruits)))
        .map_err(|e| match e {
            diesel::result::Error::NotFound => Custom(Status::NotFound, json!("Not found")),
            _ => server_error(e.into()),
        })
}

#[rocket::post("/fruits", format = "json", data = "<new_fruit>")]
pub async fn create_fruit(
    mut db: Connection<DbConn>,
    new_fruit: Json<NewFruit>,
) -> Result<Custom<Value>, Custom<Value>> {
    FruitRepository::create(&mut db, new_fruit.into_inner())
        .await
        .map(|fruits| Custom(Status::Created, json!(fruits)))
        .map_err(|e| server_error(e.into()))
}

#[rocket::put("/fruits/<fruit_id>", format = "json", data = "<update_fruit>")]
pub async fn update_fruit(
    mut db: Connection<DbConn>,
    update_fruit: Json<UpdateFruit>,
    fruit_id: i64,
) -> Result<Custom<Value>, Custom<Value>> {
    FruitRepository::update(&mut db, fruit_id, update_fruit.into_inner())
        .await
        .map(|fruits| Custom(Status::Accepted, json!(fruits)))
        .map_err(|e| server_error(e.into()))
}

#[rocket::delete("/fruits/<fruit_id>")]
pub async fn delete_fruit(
    mut db: Connection<DbConn>,
    fruit_id: i64,
) -> Result<NoContent, Custom<Value>> {
    FruitRepository::delete(&mut db, fruit_id)
        .await
        .map(|_| NoContent)
        .map_err(|e| match e {
            diesel::result::Error::NotFound => Custom(Status::NotFound, json!("Not found")),
            _ => server_error(e.into()),
        })
}
