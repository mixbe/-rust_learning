#[macro_use]
extern crate rocket;
// use rocket;
use rocket::{Build, Rocket, routes};


#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

// #[launch]
fn get_server() -> Rocket<Build> {
    rocket::build().mount("/", routes![index])
}


#[rocket::main]
async fn main() {
    if let Err(e) = get_server().launch().await {
        println!("Whoops! Rocket didn't launch!");
        // We drop the error to get a Rocket-formatted panic.
        drop(e);
    };
}