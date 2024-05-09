use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize)]
#[diesel(table_name=crate::schema::fruits)]
pub struct Fruit {
    #[serde(skip_deserializing)]
    pub id: i64,
    pub name: String,
    pub color: String,
    pub price: f64,
    #[serde(skip_deserializing)]
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name=crate::schema::fruits)]
pub struct NewFruit {
    pub name: String,
    pub color: String,
    pub price: f64,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name=crate::schema::fruits)]
pub struct UpdateFruit {
    pub price: f64,
}
