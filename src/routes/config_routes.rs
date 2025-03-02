use rocket::get;

#[get("/health")]
pub fn check_health() -> &'static str {
    "I'm alive!"
}