#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate serde;

use rocket::Rocket;
use rocket_contrib::{templates::Template, serve::StaticFiles};

#[derive(Serialize, Deserialize, Debug)]
struct Context {
    x: bool
}

#[get("/")]
fn index() -> Template {
    let context = Context { x: false };
    Template::render("index", context)
}

fn rocket() -> Rocket {
    rocket::ignite()
        .mount("/", StaticFiles::from("static/"))
        .mount("/", routes![index])
        .attach(Template::fairing())
}

fn main() {
    rocket().launch();
}
