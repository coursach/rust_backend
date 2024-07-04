use rocket::{data::ToByteUnit, http::Status, Data};
use rocket::serde::json::Json;
use crate::models::{Users, Subscribe, Content, Codepromo, File};
use crate::transmitted_models::{AddingUsers, TransmittedSubscribe, TransmittedContents, TransmittedPromocode, GetAdminUsers, AddAdminUsers, ReturnedSubscribes, TransmittedSubscribeWithId, ReturnedCodepromo, TransmittedFile, ReturnedFile};
use crate::function::*;
// All admin function for manipulating users
pub struct Token {
    info: String,
}

#[derive(Debug)]
pub enum ApiTokenError {
    Missing,
    Invalid,
}

use rocket::request::{Outcome, Request, FromRequest};
use crate::models;
use crate::models::err::{CodepromoErr, ContentErr, UserErr};

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Token {
    type Error = ApiTokenError;
    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match request.headers().get_one("token") {
            None => Outcome::Error((Status::BadRequest, ApiTokenError::Missing)),
            Some(k) => {
                if let Ok(c) = check_is_admin_with_token(k.to_string()) {
                    if c {
                        Outcome::Success(Token { info: k.to_string() })
                    } else {
                        Outcome::Error((Status::Unauthorized, ApiTokenError::Invalid))
                    }
                } else {
                    Outcome::Error((Status::BadRequest, ApiTokenError::Missing))
                }
            }
        }
    }
}

#[post("/user", data = "<user_data>", format = "json")]
pub fn add_user(user_data: Json<AddingUsers>, token: Token) -> Status {
    match check_is_admin_with_token(token.info) {
        Ok(u) => {
            match u {
                true => {
                    let user = Users { name: user_data.name.clone(), surname: user_data.surname.clone(), password: user_data.password.clone(), email: user_data.email.clone(), image: "data/image/default.png".to_string(), role: user_data.role };
                    match user.add() {
                        Ok(_) => Status::Created,
                        Err(_) => Status::Conflict,
                    }
                }
                false => Status::Unauthorized,
            }
        }
        Err(_) => Status::Unauthorized,
    }
}

#[post("/file", data = "<user_data>", format = "json")]
pub fn add_file(user_data: Json<TransmittedFile>, token: Token) -> Status {
    match check_is_admin_with_token(token.info) {
        Ok(u) => {
            match u {
                true => {
                    let file = File { path: user_data.path.clone(), id_content: user_data.idContent as i32};
                    match file.add() {
                        Ok(_) => Status::Created,
                        Err(_) => Status::Conflict,
                    }
                }
                false => Status::Unauthorized,
            }
        }
        Err(_) => Status::Unauthorized,
    }
}

#[post("/subscribe", data = "<data_subscribe>", format = "json")]
pub fn add_subscribe(data_subscribe: Json<TransmittedSubscribe>, token: Token) -> Status {
    match check_is_admin_with_token(token.info) {
        Ok(u) => {
            match u {
                true => {
                    let subscribe = Subscribe {
                        name: data_subscribe.name.clone(),
                        count_month: data_subscribe.count_month,
                        title: data_subscribe.title.clone(),
                        description: data_subscribe.description.clone(),
                        discount: data_subscribe.discount,
                        level: data_subscribe.level,
                        price: data_subscribe.price,
                    };
                    match subscribe.add() {
                        Ok(_) => Status::Ok,
                        Err(_) => Status::InternalServerError,
                    }
                }
                false => Status::Forbidden
            }
        }
        Err(_) => Status::ServiceUnavailable,
    }
}

#[post("/content", data = "<data_movie>", format = "json")]
pub fn add_movie(data_movie: Json<TransmittedContents>, token: Token) -> Status {
    match check_is_admin_with_token(token.info) {
        Ok(u) => {
            match u {
                true => {
                    let content = Content {
                        id: 0,
                        name: data_movie.name.clone(),
                        description: data_movie.description.clone(),
                        description_details: data_movie.description_details.clone(),
                        image_path: data_movie.image_path.clone(),
                        level: data_movie.level_subscribe,
                        id_mood: 1,
                    };
                    match content.add() {
                        Ok(_) => Status::Ok,
                        Err(_) => Status::InternalServerError,
                    }
                }
                false => Status::Forbidden
            }
        }
        Err(_) => Status::ServiceUnavailable,
    }
}

#[post("/image/<name>", format = "image/jpeg", data = "<data>")]
pub async fn add_image_jpeg(data: Data<'_>, token: Token, name: &str) -> Status {
    match check_is_admin_with_token(token.info) {
        Ok(u) => {
            match u {
                true => {
                    let mut buffer = vec![];
                    match data.open(1024.megabytes()).stream_to(&mut buffer).await {
                        Ok(_) => {
                            let path = format!("data/image/{}.jpeg", name).to_string();
                            match std::fs::write(path.clone(), buffer) {
                                Ok(_) => {
                                    Status::Ok
                                }
                                Err(_) => Status::Unauthorized,
                            }
                        }
                        Err(_) => Status::ExpectationFailed,
                    }
                }
                false => Status::Forbidden
            }
        }
        Err(_) => Status::ServiceUnavailable,
    }
}

#[post("/image/<name>", format = "image/png", data = "<data>")]
pub async fn add_image_png(data: Data<'_>, token: Token, name: &str) -> Status {
    match check_is_admin_with_token(token.info) {
        Ok(u) => {
            match u {
                true => {
                    let mut buffer = vec![];
                    match data.open(1024.megabytes()).stream_to(&mut buffer).await {
                        Ok(_) => {
                            let path = format!("data/image/{}.png", name).to_string();
                            match std::fs::write(path.clone(), buffer) {
                                Ok(_) => {
                                    Status::Ok
                                }
                                Err(_) => Status::Unauthorized,
                            }
                        }
                        Err(_) => Status::ExpectationFailed,
                    }
                }
                false => Status::Forbidden
            }
        }
        Err(_) => Status::ServiceUnavailable,
    }
}

#[post("/video/<name>", format = "video/mp4", data = "<data>")]
pub async fn add_video(data: Data<'_>, token: Token, name: &str) -> Status {
    match check_is_admin_with_token(token.info) {
        Ok(u) => {
            match u {
                true => {
                    let mut buffer = vec![];
                    match data.open(1024.megabytes()).stream_to(&mut buffer).await {
                        Ok(_) => {
                            let path = format!("data/video/{}.mp4", name).to_string();
                            match std::fs::write(path.clone(), buffer) {
                                Ok(_) => {
                                    Status::Ok
                                }
                                Err(_) => Status::Unauthorized,
                            }
                        }
                        Err(_) => Status::ExpectationFailed,
                    }
                }
                false => Status::Forbidden
            }
        }
        Err(_) => Status::ServiceUnavailable,
    }
}

#[post("/promo-code", data = "<data_promocode>", format = "json")]
pub fn add_promocode(data_promocode: Json<TransmittedPromocode>, token: Token) -> Status {
    match check_is_admin_with_token(token.info) {
        Ok(u) => {
            match u {
                true => {
                    let promo = Codepromo {
                        description: data_promocode.description.clone(),
                        id_subscribe: data_promocode.id_subscribe,
                        days: data_promocode.days,
                    };
                    match promo.add() {
                        Ok(_) => Status::Ok,
                        Err(_) => Status::InternalServerError,
                    }
                }
                false => Status::Forbidden
            }
        }
        Err(_) => Status::ServiceUnavailable,
    }
}


//delete

#[post("/user/<id>")]
pub fn delete_user(id: usize, token: Token) -> Status {
    match check_is_admin_with_token(token.info) {
        Ok(u) => {
            match u {
                true => {
                    match Users::delete(id) {
                        Ok(_) => Status::Ok,
                        Err(_) => Status::InternalServerError
                    }
                }
                false => Status::Unauthorized,
            }
        }
        Err(_) => Status::Unauthorized,
    }
}

#[post("/file/<id>")]
pub fn delete_file(id: usize, token: Token) -> Status {
    match check_is_admin_with_token(token.info) {
        Ok(u) => {
            match u {
                true => {
                    match models::File::delete(id) {
                        Ok(_) => Status::Ok,
                        Err(_) => Status::InternalServerError
                    }
                }
                false => Status::Unauthorized,
            }
        }
        Err(_) => Status::Unauthorized,
    }
}

#[post("/subscribe/<id>")]
pub fn delete_subscribe(id: usize, token: Token) -> Status {
    match check_is_admin_with_token(token.info) {
        Ok(u) => {
            match u {
                true => {
                    match Subscribe::delete(id) {
                        Ok(_) => Status::Ok,
                        Err(_) => Status::InternalServerError
                    }
                }
                false => Status::Unauthorized,
            }
        }
        Err(_) => Status::ServiceUnavailable,
    }
}

#[post("/content/<id>")]
pub fn delete_movie(id:usize, token: Token) -> Status {
    match check_is_admin_with_token(token.info) {
        Ok(u) => {
            match u {
                true => {
                    match Content::delete(id){
                        Ok(_) => Status::Ok,
                        Err(_) => Status::InternalServerError
                    }

                }
                false => Status::Forbidden
            }
        }
        Err(_) => Status::ServiceUnavailable,
    }
}

#[post("/image/<name>")]
pub async fn delete_image(token: Token, name: &str) -> Status {
    match check_is_admin_with_token(token.info) {
        Ok(u) => {
            match u {
                true => {
                    match std::fs::remove_file(format!("data/image/{}", name)){
                        Ok(_) => Status::Ok,
                        Err(_) => Status::BadRequest
                    }
                }
                false => Status::Forbidden
            }
        }
        Err(_) => Status::ServiceUnavailable,
    }
}

#[post("/video/<name>")]
pub async fn delete_video(token: Token, name: &str) -> Status {
    match check_is_admin_with_token(token.info) {
        Ok(u) => {
            match u {
                true => {
                    match std::fs::remove_file(format!("data/video/{}", name)){
                        Ok(_) => Status::Ok,
                        Err(_) => Status::BadRequest
                    }
                }
                false => Status::Forbidden
            }
        }
        Err(_) => Status::ServiceUnavailable,
    }
}

#[post("/promo-code/<description>")]
pub fn delete_promocode(description: &str, token: Token) -> Status {
    match check_is_admin_with_token(token.info) {
        Ok(u) => {
            match u {
                true => {
                    match Codepromo::delete(description){
                        Ok(_) => Status::Ok,
                        Err(_) => Status::BadRequest
                    }
                }
                false => Status::Forbidden
            }
        }
        Err(_) => Status::ServiceUnavailable,
    }
}




//get

#[post("/user")]
pub fn get_user(token: Token) -> Result<Json<Vec<GetAdminUsers>>, Status> {
    match check_is_admin_with_token(token.info) {
        Ok(u) => {
            match u {
                true => {
                    match Users::all() {
                        Ok(l) => Ok(Json(l)),
                        Err(_) => Err(Status::InternalServerError)
                    }
                }
                false => Err(Status::Unauthorized),
            }
        }
        Err(_) => Err(Status::Unauthorized),
    }
}

#[post("/files")]
pub fn get_files(token: Token) -> Result<Json<Vec<ReturnedFile>>, Status> {
    match check_is_admin_with_token(token.info) {
        Ok(u) => {
            match u {
                true => {
                    match File::all() {
                        Ok(l) => Ok(Json(l)),
                        Err(_) => Err(Status::InternalServerError)
                    }
                }
                false => Err(Status::Unauthorized),
            }
        }
        Err(_) => Err(Status::Unauthorized),
    }
}

#[post("/code-promo")]
pub fn get_codepromo(token: Token) -> Result<Json<Vec<ReturnedCodepromo>>, Status> {
    match check_is_admin_with_token(token.info) {
        Ok(u) => {
            match u {
                true => {
                    match Codepromo::all() {
                        Ok(l) => Ok(Json(l)),
                        Err(_) => Err(Status::InternalServerError)
                    }
                }
                false => Err(Status::Unauthorized),
            }
        }
        Err(_) => Err(Status::Unauthorized),
    }
}

#[post("/images")]
pub fn get_images(token: Token) -> Result<Json<Vec<String>>, Status> {
    match check_is_admin_with_token(token.info) {
        Ok(u) => {
            match u {
                true => {
                    let paths = std::fs::read_dir("./data/image").unwrap();
                    let mut result: Vec<String>= Vec::new();
                    for path in paths{
                        result.push(path.unwrap().file_name().into_string().unwrap())
                    }
                    Ok(Json(result))
                }
                false => Err(Status::Unauthorized),
            }
        }
        Err(_) => Err(Status::Unauthorized),
    }
}

#[post("/videos")]
pub fn get_videos(token: Token) -> Result<Json<Vec<String>>, Status> {
    match check_is_admin_with_token(token.info) {
        Ok(u) => {
            match u {
                true => {
                    let paths = std::fs::read_dir("./data/video").unwrap();
                    let mut result: Vec<String>= Vec::new();
                    for path in paths{
                        result.push(path.unwrap().file_name().into_string().unwrap())
                    }
                    Ok(Json(result))
                }
                false => Err(Status::Unauthorized),
            }
        }
        Err(_) => Err(Status::Unauthorized),
    }
}

//update
#[post("/user", data="<user_data>", format ="json")]
pub fn update_user(user_data: Json<AddAdminUsers>, token: Token) -> Status{
    match check_is_admin_with_token(token.info) {
        Ok(u) => {
            match u {
                true => {
                    let user = Users{
                        name: user_data.name.clone(),
                        surname: user_data.surname.clone(),
                        password: user_data.password.clone(),
                        email: user_data.email.clone(),
                        image: user_data.image.clone(),
                        role: user_data.role as i32,
                    };
                    match user.update_to_admin(user_data.id){
                        Ok(_) => Status::Ok,
                        Err(_) => Status::InternalServerError
                    }

                }
                false => Status::Unauthorized,
            }
        }
        Err(_) => Status::Unauthorized,
    }
}

#[post("/subscribe", data="<user_data>", format ="json")]
pub fn update_subscribe(user_data: Json<TransmittedSubscribeWithId>, token: Token) -> Status{
    match check_is_admin_with_token(token.info) {
        Ok(u) => {
            match u {
                true => {
                    let subscribe = Subscribe{
                        name: user_data.name.clone(),
                        price: user_data.price.clone(),
                        level: user_data.level.clone(),
                        description: user_data.description.clone(),
                        title: user_data.title.clone(),
                        count_month: user_data.count_month,
                        discount: user_data.discount
                    };
                    match subscribe.update(user_data.id){
                        Ok(_) => Status::Ok,
                        Err(_) => Status::InternalServerError
                    }

                }
                false => Status::Unauthorized,
            }
        }
        Err(_) => Status::Unauthorized,
    }
}
/*
}

#[post("/user", data="<user_data>", format ="json")]
pub fn update_user(user_data: String) -> Status{
    match serde_json::from_str::<UpdatingUsers>(&user_data) {
        Ok(t_u) => {
            match check_is_admin_with_token(t_u.token) {
                Ok(u) => {
                    match u {
                        true => {
                            match t_u.name_field.as_str() {
                                "Name" => {
                                    let mut user = Users::empty_user();
                                    user.name = t_u.information;
                                    match user.update(t_u.id) {
                                        Ok(_) => Status::Ok,
                                        Err(_) => Status::InternalServerError,
                                    }
                                },
                                "Surname" => {
                                    let mut user = Users::empty_user();
                                    user.surname = t_u.information;
                                    match user.update(t_u.id) {
                                        Ok(_) => Status::Ok,
                                        Err(_) => Status::InternalServerError,
                                    }
                                },
                                "Email" => {
                                    let mut user = Users::empty_user();
                                    user.email = t_u.information;
                                    match user.update(t_u.id) {
                                        Ok(_) => Status::Ok,
                                        Err(_) => Status::InternalServerError,
                                    }
                                },
                                "Password" => {
                                    let mut user = Users::empty_user();
                                    user.password = t_u.information;
                                    match user.update(t_u.id) {
                                        Ok(_) => Status::Ok,
                                        Err(_) => Status::InternalServerError,
                                    }
                                },
                                "Role" =>{
                                    let mut user = Users::empty_user();
                                    match t_u.information.parse::<i32>() {
                                        Ok(role) => {
                                            user.role = role;
                                            match user.update(t_u.id) {
                                                Ok(_) => Status::Ok,
                                                Err(_) => Status::InternalServerError,
                                            }
                                        },
                                        Err(_) => Status::BadRequest,
                                    }
                                }
                                _ => Status::UnprocessableEntity,
                            }
                            },
                        false => Status::Unauthorized,
                    }
                },
                Err(_) => Status::Unauthorized,
            }
        },
        Err(_) => Status::BadRequest,
    }
}



// All admin function for manipulating subscriptions
#[post("/subscribe", data="<data_subscribe>", format ="json")]
pub fn update_subscibe(data_subscribe: String) -> Status{
    match serde_json::from_str::<TransmittedSubscribe>(&data_subscribe) {
    Ok(t_d) => {
        match check_is_admin_with_token(t_d.token) {
            Ok(u) => {
                match u{
                    true => {
                        let subscribe = Subscribe{
                            name: t_d.name,
                            count_month: t_d.count_month,
                            title: t_d.title,
                            description: t_d.description,
                            discount: t_d.discount
                        };
                        match subscribe.add() {
                            Ok(_) => Status::Ok,
                            Err(_) => Status::InternalServerError,
                        }
                    },
                    false => Status::Unauthorized,
                }
            },
            Err(_) => Status::ServiceUnavailable,
        }
    },
    Err(_) => Status::BadRequest,
    }
}



*/
// All admin function for manipulating content
