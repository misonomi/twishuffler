use crate::{cookie::new_cookie, errors::Error};
use rocket::{
    get,
    http::{Cookie, CookieJar},
    response::Redirect,
    FromForm, State,
};
use twitter_v2::{
    authorization::{Oauth2Client, Scope},
    oauth2::{AuthorizationCode, PkceCodeChallenge, PkceCodeVerifier},
};

#[derive(FromForm)]
pub struct Callback<'a> {
    code: &'a str,
    state: &'a str,
}

#[get("/authorize")]
pub fn authorize(client: &State<Oauth2Client>, cookies: &CookieJar<'_>) -> Redirect {
    let (challenge, verifier) = PkceCodeChallenge::new_random_sha256();

    let (url, state) = client.auth_url(
        challenge,
        [
            Scope::TweetRead,
            Scope::UsersRead,
            Scope::LikeRead,
            Scope::BookmarkRead,
        ],
    );

    cookies.add_private(new_cookie("verifier", verifier.secret().clone()));
    cookies.add_private(new_cookie("state", state.secret().clone()));

    Redirect::to(url.to_string())
}

#[get("/callback?<callback..>")]
pub async fn callback(
    callback: Callback<'_>,
    client: &State<Oauth2Client>,
    cookies: &CookieJar<'_>,
) -> Result<Redirect, Error> {
    match cookies.get_private("state") {
        Some(state) => {
            if state.value() != callback.state {
                return Err(Error::StateUnmatch);
            }
        }
        None => {
            return Err(Error::NoState);
        }
    };

    let verifier = match cookies.get_private("verifier") {
        Some(v) => v,
        None => {
            return Err(Error::NoVerifier);
        }
    };

    let token_str = match client
        .request_token(
            AuthorizationCode::new(callback.code.to_string()),
            PkceCodeVerifier::new(verifier.value().to_string()),
        )
        .await
    {
        Ok(token) => match serde_json::to_string(&token) {
            Ok(token_str) => token_str,
            Err(e) => {
                println! {"{:?}", e};
                return Err(Error::SerializeToken);
            }
        },
        Err(e) => {
            println! {"{:?}", e};
            return Err(Error::RequestTokenAPI);
        }
    };

    cookies.add_private(new_cookie("token", token_str));
    cookies.remove_private(Cookie::named("state"));
    cookies.remove_private(Cookie::named("verifier"));

    Ok(Redirect::to("/likes"))
}
