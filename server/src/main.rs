use rocket::fs::FileServer;
use rocket_dyn_templates::Template;

#[macro_use]
extern crate rocket;

mod routes;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Template::fairing())
        .mount("/static", FileServer::from("static"))
        .mount(
            "/",
            routes![routes::frontend::home, routes::frontend::search],
        )
}
