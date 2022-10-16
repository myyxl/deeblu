mod routes;
mod guards;

#[macro_use] extern crate rocket;

use crate::routes::deploy::deploy;

use std::path::Path;
use std::{fs, process};
use log::{error, info};
use uuid::Uuid;


struct AuthorizationState {
    auth_token: String
}

#[launch]
fn rocket() -> _ {
    env_logger::init();
    let auth_file_name = ".auth";
    let auth_token =
        if !Path::new(auth_file_name).exists() {
            info!("Could not find auth token file, creating one");
            let auth_token = Uuid::new_v4().to_string();
            match fs::write(auth_file_name, &auth_token) {
                Ok(()) => auth_token,
                Err(_) => {
                    error!("Could not write to file .auth!");
                    process::exit(1);
                }
            }
        } else {
            match fs::read_to_string(".auth") {
                Ok(auth_token) => auth_token,
                Err(_) => {
                    error!("Could not read file .auth!");
                    process::exit(1);
                }
            }
        };
    info!("Loaded auth token");
    rocket::build()
        .mount("/", routes![deploy])
        .manage(AuthorizationState { auth_token })
}