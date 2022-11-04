use rocket::{
    http::Status,
    response::{Responder, Response, Result},
    Request,
};

pub enum Error {
    StateUnmatch,
    NoState,
    NoVerifier,
    NoToken,
    SerializeToken,
    DeserializeToken,
    RequestTokenAPI,
    GetMeAPI,
    GetLikesAPI,
}

impl<'a> Error {
    fn str(self) -> &'a str {
        match self {
            Self::StateUnmatch => "state_unmatch",
            Self::NoState => "no_state",
            Self::NoVerifier => "no_verifier",
            Self::NoToken => "no_token",
            Self::SerializeToken => "serialize_token",
            Self::DeserializeToken => "deserialize_token",
            Self::RequestTokenAPI => "request_token",
            Self::GetMeAPI => "get_me",
            Self::GetLikesAPI => "get_likes",
        }
    }
}

impl<'r> Responder<'r, 'static> for Error {
    fn respond_to(self, _: &'r Request<'_>) -> Result<'static> {
        Response::build()
            .status(Status::SeeOther)
            .raw_header("Location", format!("/error?reason={}", self.str()))
            .ok()
    }
}
