use rocket::serde::{Deserialize, Serialize};
use twitter_v2::{data::Expansions, id::NumericId, Media};

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(crate = "rocket::serde")]
pub struct Tweet {
    pub id: NumericId,
    pub author: String,
    pub text: String,
    pub media: Vec<Media>,
}

impl Tweet {
    pub fn from(tweet: &twitter_v2::Tweet, includes: Option<&Expansions>) -> Self {
        let media = match tweet
            .attachments
            .as_ref()
            .map(|a| a.media_keys.as_ref())
            .flatten()
        {
            Some(m) => m
                .iter()
                .map(|m| {
                    includes
                        .unwrap()
                        .media
                        .as_ref()
                        .unwrap()
                        .iter()
                        .find(|i| i.media_key == *m)
                        .unwrap()
                        .clone()
                })
                .collect(),
            None => vec![],
        };

        let author = includes
            .unwrap()
            .users
            .as_ref()
            .unwrap()
            .iter()
            .find(|u| u.id == tweet.author_id.unwrap())
            .unwrap()
            .username
            .to_string();

        Tweet {
            id: tweet.id,
            author: author,
            text: tweet.text.clone(),
            media: media,
        }
    }
}
