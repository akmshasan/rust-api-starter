use diesel::{ExpressionMethods, QueryDsl, QueryResult};
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::{
    models::{Fruit, NewFruit, UpdateFruit},
    schema::fruits,
};

pub struct FruitRepository;

impl FruitRepository {
    pub async fn get(c: &mut AsyncPgConnection, fruit_id: i64) -> QueryResult<Fruit> {
        fruits::table.find(fruit_id).get_result(c).await
    }

    pub async fn get_all(c: &mut AsyncPgConnection, limit: i64) -> QueryResult<Vec<Fruit>> {
        fruits::table.limit(limit).get_results(c).await
    }

    pub async fn create(c: &mut AsyncPgConnection, new_fruit: NewFruit) -> QueryResult<Fruit> {
        diesel::insert_into(fruits::table)
            .values(new_fruit)
            .get_result(c)
            .await
    }

    pub async fn update(
        c: &mut AsyncPgConnection,
        fruit_id: i64,
        update_fruit: UpdateFruit,
    ) -> QueryResult<Fruit> {
        diesel::update(fruits::table.find(fruit_id))
            .set(fruits::price.eq(update_fruit.price))
            .get_result(c)
            .await
    }

    pub async fn delete(c: &mut AsyncPgConnection, fruit_id: i64) -> QueryResult<usize> {
        diesel::delete(fruits::table.find(fruit_id))
            .execute(c)
            .await
    }
}
