use rocket_db_pools::Database;
pub mod models;
pub mod repositories;
pub mod rocket_routes;
pub mod schema;

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount(
            "/",
            rocket::routes![
                rocket_routes::fruits::get_fruits,
                rocket_routes::fruits::get_fruit,
                rocket_routes::fruits::create_fruit,
                rocket_routes::fruits::update_fruit,
                rocket_routes::fruits::delete_fruit,
            ],
        )
        .attach(rocket_routes::DbConn::init())
        .launch()
        .await;
}
