use rocket::serde::json::Json;
use crate::handlers::movie_handlers::{get_movies, get_movie};
use crate::models::movie::Movie;

#[get("/movies")]
pub fn get_movies_route() -> Json<Vec<Movie>> {
    get_movies()
}

#[get("/movies/<id>")]
pub fn get_movie_route(id: i32) -> Option<Json<Movie>> {
    get_movie(id)
}
