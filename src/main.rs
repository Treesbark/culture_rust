#[macro_use] extern crate rocket;

mod hello;
mod name;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(hello::stage())
        .attach(name::stage())
}
