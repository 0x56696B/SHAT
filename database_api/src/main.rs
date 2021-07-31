#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

//fn main()
#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
