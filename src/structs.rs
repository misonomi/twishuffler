use rocket::serde::{Deserialize, Serialize};
use twitter_v2::{id::NumericId, Media};

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(crate = "rocket::serde")]
pub struct Tweet {
    pub author: String,
    pub text: String,
    pub media: Vec<Media>,
}

impl Tweet {
    pub fn from(tweet: &twitter_v2::Tweet, includes: Option<&Vec<Media>>) -> Self {
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
                        .iter()
                        .find(|i| i.media_key == *m)
                        .unwrap()
                        .clone()
                })
                .collect(),
            None => vec![],
        };

        Tweet {
            author: tweet.author_id.unwrap_or(NumericId::new(0)).to_string(),
            text: tweet.text.clone(),
            media: media,
        }
    }
}
