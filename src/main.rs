#[macro_use]
extern crate rocket;


use rocket::fs::NamedFile;
use rocket::http::{Header, Status};
use rocket::serde::json::Json;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::{Request, Response};

use models::*;
use transmitted_models::*;
use claims::Claims;

use user_function::*;
use admin_function::*;

mod function;
pub mod models;
mod transmitted_models;
mod claims;
mod user_function;
mod admin_function;


pub struct Cors;

#[rocket::async_trait]
impl Fairing for Cors {
    fn info(&self) -> Info {
        Info {
            name: "Cross-Origin-Resource-Sharing Fairing",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "http://localhost:3000"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, PATCH, PUT, DELETE, HEAD, OPTIONS, GET",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

#[get("/subscribe")]
fn get_all_subscribe() -> Result<Json<Vec<ReturnedSubscribes>>, Status> {
    match Subscribe::all() {
        Ok(v) => Ok(Json(v)),
        Err(_) => Err(Status::Unauthorized),
    }
}

#[post("/login", data = "<login_data>")]
fn login(login_data: Json<LoginRequest>) -> Result<Json<TransmittedToken>, Status> {
    match Users::login(login_data.email.clone(), login_data.password.clone()) {
        Ok(u) => {
            match u.0 {
                true => {
                    let claim = Claims::from_name(&format!("{}:{}:{}", u.1, login_data.email, login_data.password));
                    let token_string;
                    match claim.into_token() {
                        Ok(s) => token_string = s,
                        Err(_) => return Err(Status::Unauthorized),
                    };
                    let transmitted_token = TransmittedToken {
                        token: token_string,
                    };
                    Ok(Json(transmitted_token))
                }
                false => Err(Status::Unauthorized),
            }
        }
        Err(_) => Err(Status::Unauthorized),
    }
}


#[options("/<_..>")]
fn everything() -> Status {
    Status::Ok
}

#[get("/images/<name>")]
async fn get_image(name: &str) -> Result<NamedFile, Status> {
    match NamedFile::open(format!("data/image/{}", name.to_string())).await.ok() {
        Some(v) => Ok(v),
        None => Err(Status::NotFound),
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Cors)
        .mount("/", routes![get_all_subscribe, login, get_image, everything, all_content_movie, all_content_anime, return_info_content_by_id])
        .mount("/user/update", routes![update_profile, update_image_profile_jpeg, update_image_profile_png])
        .mount("/user/link", routes![link_subscibe_to_user])
        .mount("/user/unlink", routes![unlink_subscibe_to_user])
        .mount("/user/get", routes![get_subscibe_to_profile, get_user_profile, get_subscibe_to_promocode, get_content_from_token, get_history_by_token])
        .mount("/find", routes![find_content])
        .mount("/registration", routes![registration_user])
        //.mount("/admin/update", routes![update_subscibe, update_user])
        .mount("/admin/add", routes![add_user, add_subscribe, add_movie, add_image_png, add_image_jpeg, add_video, add_promocode, add_file])
        .mount("/admin/delete", routes![delete_image, delete_movie, delete_promocode, delete_subscribe, delete_user, delete_video, delete_file])
        .mount("/admin/get", routes![get_user, get_codepromo, get_images, get_videos, get_files])
        .mount("/admin/update", routes![update_user, update_subscribe])
}


