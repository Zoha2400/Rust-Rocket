#[macro_use] extern crate rocket;

use rocket::tokio::time::{sleep, Duration};

#[get("/delay/<seconds>")]
async fn delay(seconds: u64) -> String {
    sleep(Duration::from_secs(seconds)).await;
    format!("Waited for {} seconds", seconds)
}

#[get("/hello/<name>")]
fn send_name(name: &str) -> String {
    if name == "Zoha2400"{
        format!("Hi my lord!")
    }else{
        format!("Hello stranger!")
    }
}

// #[get("/")]
// fn index() -> &'static str {
//     "Hello, world!"
// }

// #[get("/io")]
// fn io() -> &'static str {
//     "Io baby!"
// }

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![delay, send_name])
}