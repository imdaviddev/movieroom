#[macro_use] extern crate rocket;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::{Request, Response};
use std::env;

#[get("/check_health")]
fn check_health() -> &'static str {
    "API is healthy"
}

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "GET, POST, PUT, DELETE, OPTIONS"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "Content-Type, Authorization"));
    }
}

#[launch]
fn rocket() -> _ {
    let port = env::var("PORT").unwrap_or_else(|_| "10000".to_string()).parse::<u16>().expect("Invalid port number");
    rocket::build()
        .attach(CORS)
        .mount("/", routes![check_health])
        .configure(rocket::Config {
            address: "0.0.0.0".parse().unwrap(),
            port,
            ..rocket::Config::default()
        })
}