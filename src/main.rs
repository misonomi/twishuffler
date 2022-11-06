use rocket::{
    fs::{relative, FileServer},
    get, launch, routes,
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

#[launch]
fn rocket() -> _ {
    let client = Oauth2Client::new(
        env::var("SHUFFLER_CLIENT_ID").unwrap(),
        env::var("SHUFFLER_CLIENT_SECRET").unwrap(),
        Url::parse(&env::var("SHUFFLER_CALLBACK_URL").unwrap()).unwrap(),
    );

    rocket::build()
        .manage(client)
        .mount("/static", FileServer::from(relative!("static")))
        .mount(
            "/",
            routes![index, error, oauth::authorize, oauth::callback, lists::list,],
        )
        .attach(Template::fairing())
}
