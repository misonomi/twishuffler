use rand::{seq::SliceRandom, thread_rng};
use rocket::get;
use rocket::http::CookieJar;
use rocket_dyn_templates::{context, Template};
use twitter_v2::{
    authorization::Oauth2Token,
    query::{MediaField, TweetExpansion},
    TwitterApi,
};

use crate::{errors::Error, structs};

#[get("/likes")]
pub async fn likes(cookies: &CookieJar<'_>) -> Result<Template, Error> {
    let token = match cookies.get_private("token") {
        Some(v) => match serde_json::from_str::<Oauth2Token>(v.value()) {
            Ok(t) => t,
            Err(e) => {
                println! {"{:?}", e};
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
            println! {"{:?}", e};
            return Err(Error::GetMeAPI);
        }
    };

    let mut likes: Vec<structs::Tweet> = match api
        .get_user_liked_tweets(me.id)
        .expansions(vec![TweetExpansion::AttachmentsMediaKeys])
        .media_fields(vec![
            MediaField::MediaKey,
            MediaField::Url,
            MediaField::PreviewImageUrl,
        ])
        .send()
        .await
    {
        Ok(res) => res
            .data()
            .unwrap_or(&vec![])
            .iter()
            .map(|t| structs::Tweet::from(t, res.includes().map(|e| e.media.as_ref()).flatten()))
            .collect(),
        Err(e) => {
            println! {"{:?}", e};
            return Err(Error::GetLikesAPI);
        }
    };

    let mut rng = thread_rng();
    likes.shuffle(&mut rng);

    Ok(Template::render("list", context! { tweets: likes }))
}

#[get("/bookmarks")]
pub async fn bookmarks(cookies: &CookieJar<'_>) -> Result<Template, Error> {
    let token = match cookies.get_private("token") {
        Some(v) => match serde_json::from_str::<Oauth2Token>(v.value()) {
            Ok(t) => t,
            Err(e) => {
                println! {"{:?}", e};
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
            println! {"{:?}", e};
            return Err(Error::GetMeAPI);
        }
    };

    let mut bookmarks: Vec<structs::Tweet> = match api
        .get_user_bookmarks(me.id)
        .expansions(vec![TweetExpansion::AttachmentsMediaKeys])
        .media_fields(vec![
            MediaField::MediaKey,
            MediaField::Url,
            MediaField::PreviewImageUrl,
        ])
        .send()
        .await
    {
        Ok(res) => res
            .data()
            .unwrap_or(&vec![])
            .iter()
            .map(|t| structs::Tweet::from(t, res.includes().map(|e| e.media.as_ref()).flatten()))
            .collect(),
        Err(e) => {
            println! {"{:?}", e};
            return Err(Error::GetLikesAPI);
        }
    };

    let mut rng = thread_rng();
    bookmarks.shuffle(&mut rng);

    Ok(Template::render("list", context! { tweets: bookmarks }))
}
