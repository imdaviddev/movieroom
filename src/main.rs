#[macro_use] extern crate rocket;

mod routes;
mod models;
mod handlers;

#[get("/check_health")]
fn check_health() -> &'static str {
    "I'm alive!"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/check_health", routes![check_health])
        .mount("/api", routes![
            routes::get_movies_route, 
            routes::get_movie_route
        ])
}