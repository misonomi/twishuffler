use rocket::http::CookieJar;
use rocket_dyn_templates::{context, Template};
use twitter_v2::{authorization::Oauth2Token, TwitterApi, query::{GetRelatedTweetsRequestBuilder, TweetField, MediaField, TweetExpansion}, data::Expansions};

use crate::{errors::Error, structs};

#[get("/likes")]
pub async fn likes(cookies: &CookieJar<'_>) -> Result<Template, Error> {
    let token = match cookies.get("token") {
        Some(v) => match serde_json::from_str::<Oauth2Token>(v.value()) {
            Ok(t) => t,
            Err(e) => {
                println! {"{}", e};
                return Err(Error::DeserializeToken);
            }
        },
        None => {
            return Err(Error::NoToken);
        }
    };

    let api = TwitterApi::new(token);

    let me = match api.get_users_me().send().await {
        Ok(res) => res.into_data().unwrap(),
        Err(e) => {
            println! {"{}", e};
            return Err(Error::GetMeAPI);
        }
    };

    let likes: Vec<structs::Tweet> = match api.get_user_liked_tweets(me.id).expansions(vec!(TweetExpansion::AttachmentsMediaKeys)).media_fields(vec!(MediaField::MediaKey, MediaField::Url, MediaField::PreviewImageUrl)).send().await {
        Ok(res) => {
            let media = res.includes().map(|e| e.media.as_ref()).flatten();
            res.data().unwrap().iter().map(|t| structs::Tweet::from(t, media)).collect()
        },
        Err(e) => {
            println! {"{}", e};
            return Err(Error::GetLikesAPI);
        }
    };

    Ok(Template::render(
        "likes",
        context! { title: "top", likes: likes },
    ))
}

#[get("/bookmarks")]
pub fn bookmarks() -> Template {
    Template::render("top", context! { title: "top" })
}
