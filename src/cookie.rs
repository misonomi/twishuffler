use rocket::http::{Cookie, SameSite};

pub fn new_cookie<'a>(name: &'a str, value: String) -> Cookie<'a> {
    Cookie::build(name, value)
        .path("/")
        .secure(true)
        .same_site(SameSite::Lax)
        .finish()
}
