#[get("/")]
fn index() -> &'static str {
    "Nothing on widebeam..."
}

pub fn stage() -> rocket::fairing::AdHoc {
    rocket::fairing::AdHoc::on_ignite("MessagePack", |rocket| async {
        rocket.mount("/", routes![index])
    })
}