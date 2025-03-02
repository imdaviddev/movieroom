#[macro_use] extern crate rocket;

mod routes;
mod handlers;
mod models;

use routes::movie_routes::{get_movies_route, get_movie_route};
use routes::config_routes::check_health;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![check_health])
        .mount("/api", routes![get_movies_route, get_movie_route])
}