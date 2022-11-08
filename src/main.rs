use rocket::{
    fs::{relative, FileServer},
    get, launch, routes, catch, catchers, response::Redirect,
};
use rocket_dyn_templates::{context, Template};
use std::env;
use twitter_v2::{authorization::Oauth2Client, oauth2::url::Url};

mod cookie;
mod errors;
mod list_type;
mod lists;
mod oauth;
mod tweet;

#[get("/")]
async fn index() -> Template {
    Template::render("top", context! {})
}

#[get("/error?<reason>")]
async fn error(reason: &str) -> Template {
    Template::render("top", context! { error: reason })
}

#[catch(404)]
fn not_found() -> Redirect {
    Redirect::to("/error?reason=not_found")
}

#[launch]
fn rocket() -> _ {
    let client = Oauth2Client::new(
        env::var("SHUFFLER_CLIENT_ID").unwrap(),
        env::var("SHUFFLER_CLIENT_SECRET").unwrap(),
        Url::parse(&env::var("SHUFFLER_CALLBACK_URL").unwrap()).unwrap(),
    );

    rocket::build()
        .manage(client)
        .register("/", catchers![not_found])
        .mount("/static", FileServer::from(relative!("static")))
        .mount(
            "/",
            routes![index, error, oauth::authorize, oauth::callback, lists::list,],
        )
        .attach(Template::fairing())
}
