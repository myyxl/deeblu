use rocket::http::Status;
use rocket::Request;
use rocket::request::{FromRequest, Outcome};
use crate::AuthorizationState;

pub struct AuthGuard;

#[derive(Debug)]
pub enum AuthGuardError {
    Missing,
    Invalid
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthGuard {
    type Error = AuthGuardError;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {

        let auth_state = request.rocket().state::<AuthorizationState>().unwrap();

        match request.headers().get_one("Authorization") {
            None => Outcome::Failure((Status::Unauthorized, AuthGuardError::Missing)),
            Some(key) if key == auth_state.auth_token => Outcome::Success(AuthGuard {}),
            Some(_) => Outcome::Failure((Status::Unauthorized, AuthGuardError::Invalid)),
        }
    }
}
