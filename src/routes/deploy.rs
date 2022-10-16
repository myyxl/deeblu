use crate::guards::auth_guard::AuthGuard;

#[get("/deploy")]
pub fn deploy(_auth_guard: AuthGuard) -> &'static str {
    "Hello, world!"
}