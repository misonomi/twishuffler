use rand::{seq::SliceRandom, thread_rng};
use rocket::get;
use rocket::http::CookieJar;
use rocket_dyn_templates::{context, Template};
use twitter_v2::{
    authorization::Oauth2Token,
    query::{MediaField, TweetExpansion},
    TwitterApi,
};

use crate::{errors::Error, list_type::ListType, tweet};

#[get("/<listtype>?<next>")]
pub async fn list(
    listtype: ListType,
    next: Option<&str>,
    cookies: &CookieJar<'_>,
) -> Result<Template, Error> {
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

    let mut builder = listtype.request_builder(api, me.id);

    builder
        .expansions(vec![
            TweetExpansion::AttachmentsMediaKeys,
            TweetExpansion::AuthorId,
        ])
        .media_fields(vec![
            MediaField::MediaKey,
            MediaField::Url,
            MediaField::PreviewImageUrl,
        ]);

    if let Some(n) = next {
        builder.pagination_token(n);
    }

    let (mut tweets, next): (Vec<tweet::Tweet>, Option<String>) = match builder.send().await {
        Ok(res) => (
            res.data()
                .unwrap_or(&vec![])
                .iter()
                .map(|t| tweet::Tweet::from(t, res.includes()))
                .collect(),
            res.meta().map(|m| m.next_token.to_owned()).flatten(),
        ),
        Err(e) => {
            println! {"{:?}", e};
            return Err(Error::GetLikesAPI);
        }
    };

    let mut rng = thread_rng();
    tweets.shuffle(&mut rng);

    // TODO: find out why match` arms have incompatible types happens
    match listtype {
        ListType::Likes => Ok(Template::render(
            "list",
            context! { tweets: tweets, next: context! { likes: next }},
        )),
        ListType::Bookmarks => Ok(Template::render(
            "list",
            context! { tweets: tweets, next: context! { bookmarks: next } },
        )),
    }
}
