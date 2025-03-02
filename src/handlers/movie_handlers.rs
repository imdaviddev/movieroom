use rocket::serde::json::Json;
use crate::models::movie::Movie;

pub fn get_movies() -> Json<Vec<Movie>> {
    let movies = vec![
        Movie { id: 1, title: "Inception".to_string(), director: "Christopher Nolan".to_string(), year: 2010 },
        Movie { id: 2, title: "The Matrix".to_string(), director: "Lana Wachowski, Lilly Wachowski".to_string(), year: 1999 },
    ];
    Json(movies)
}

pub fn get_movie(id: i32) -> Option<Json<Movie>> {
    let movies = vec![
        Movie { id: 1, title: "Inception".to_string(), director: "Christopher Nolan".to_string(), year: 2010 },
        Movie { id: 2, title: "The Matrix".to_string(), director: "Lana Wachowski, Lilly Wachowski".to_string(), year: 1999 },
    ];
    movies.into_iter().find(|movie| movie.id == id).map(Json)
}
