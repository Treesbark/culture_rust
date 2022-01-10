use rand::seq::SliceRandom;
use rocket::serde::{Serialize, Deserialize};
use rocket::response::status;

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct Response<'r> {
    name: &'r str
}


#[get("/return-culture-ship-name")]
fn return_culture_ship_name() -> status::Accepted<String> {
    let culture_ship_names = vec![
        "So Much For Subtlety",
        "Little Rascal",
        "Unfortunate Conflict Of Evidence",
        "Just Read The Instructions",
        "Flexible Demeanour",
        "Of Course I Still Love You",
        "Limiting Factor",
        "Gunboat Diplomat",
        "Size Isn't Everything",
        "Congenital Optimist",
        "Sweet and Full of Grace",
        "Death And Gravity",
        "Ethics Gradient",
        "Honest Mistake",
        "Serious Callers Only",
        "Fate Amenable To Change",
        "Very Little Gravitas Indeed",
        "Problem Child",
        "It's Character Forming",
        "Killing Time",
        "Quietly Confident",
        "Experiencing A Significant Gravitas Shortfall",
        "Don't Try This At Home",
        "Now We Try It My Way",
        "Falling Outside the Normal Moral Constraints",
        "Mistake Not...",
        "Smile Tolerantly"];

    let culture_ship: Vec<_> = culture_ship_names
        .choose_multiple(&mut rand::thread_rng(), 1)
        .collect();

    status::Accepted(Some(format!("name: '{}'", culture_ship[0])))
}

pub fn stage() -> rocket::fairing::AdHoc {
    rocket::fairing::AdHoc::on_ignite("MessagePack", |rocket| async {
        rocket.mount("/", routes![return_culture_ship_name])
    })
}