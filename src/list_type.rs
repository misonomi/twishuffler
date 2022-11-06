use rocket::request::FromParam;
use twitter_v2::{
    authorization::Oauth2Token, id::NumericId, meta::ResultCountMeta,
    query::GetRelatedTweetsRequestBuilder, Tweet, TwitterApi,
};

pub enum ListType {
    Likes,
    Bookmarks,
}

impl<'a> FromParam<'a> for ListType {
    type Error = &'a str;

    fn from_param(param: &'a str) -> Result<Self, Self::Error> {
        match param {
            "likes" => Ok(ListType::Likes),
            "bookmarks" => Ok(ListType::Bookmarks),
            _ => Err(param),
        }
    }
}

impl ListType {
    pub fn request_builder(
        &self,
        api: TwitterApi<Oauth2Token>,
        myid: NumericId,
    ) -> GetRelatedTweetsRequestBuilder<Oauth2Token, Vec<Tweet>, ResultCountMeta> {
        match self {
            ListType::Likes => api.get_user_liked_tweets(myid),
            ListType::Bookmarks => api.get_user_bookmarks(myid),
        }
    }
}
