use rocket::serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Movie {
    pub id: i32,
    pub title: String,
    pub director: String,
    pub year: i32,
}
