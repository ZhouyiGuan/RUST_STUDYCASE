#[macro_use] extern crate rocket;
 
#[get("/")]
fn hello() -> String {
    "hello world".to_string()
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![hello])
}