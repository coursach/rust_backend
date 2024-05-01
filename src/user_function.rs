
use chrono::prelude::*;
use rocket::{data::ToByteUnit, http::Status, Data};  

use crate::models::{Users, Subscribe, SubscribeAndUser};
use crate::transmitted_models::{UpdateProfileData, RegistrationUsers, TransmittedSubscribeAndUser, TransmittedToken};
use crate::function::*;


pub struct Token{
    message:String,
}

#[derive(Debug)]
pub enum ApiTokenError{
    Missing, Invalid
}

use rocket::request::{Outcome, Request, FromRequest};

#[rocket::async_trait]
impl <'r> FromRequest<'r> for Token{
    type Error = ApiTokenError;
    async fn from_request(request: &'r Request<'_>) -> Outcome<Self,Self::Error>{
        match request.headers().get_one("token") {
            None => Outcome::Error((Status::BadRequest, ApiTokenError::Missing)),
            Some(k) => {
                match check_is_user_correct_with_token(k.to_string()) {
                    Ok(c) =>{
                        if c {
                            return Outcome::Success(Token{message:k.to_string()})
                        }else{
                            return Outcome::Error((Status::Unauthorized, ApiTokenError::Invalid));
                        }
                    },
                    Err(_) => {
                        println!("{}", k);
                        return Outcome::Error((Status::BadRequest, ApiTokenError::Missing));}
                }
            }
        }  
    }
}

//All function to manipulate with profile user
#[post("/user", data="<user_data>", format ="json")]
pub fn registration_user(user_data: String) -> Status{
    match serde_json::from_str::<RegistrationUsers>(&user_data) {
    Ok(r_u) => {
        let mut user = Users::empty_user();
        user.email = r_u.email;
        user.password = r_u.password;
        user.role = 2;
        match user.add(){
            Ok(_) => Status::Created,
            Err(_) => Status::Conflict,
        }     
    },
    Err(_) => Status::BadRequest,
    }
}


#[post("/user", data="<user_data>", format ="json")]
pub fn update_profile(user_data: String, token: Token) -> Status{
    match serde_json::from_str::<UpdateProfileData>(&user_data) {
    Ok(u) => {
        match get_user_data_from_token(token.message.to_string()) {
            Ok(token_data) => { 
                match u.name_field.as_str() {
                    "Name" => {
                        println!("fsdfsdf");
                        let mut user = Users::empty_user();
                        user.name = u.information;
                        match user.update(token_data.0) {
                            Ok(_) => Status::Ok,
                            Err(_) => Status::InternalServerError,
                        }
                    },
                    "Surname" => {
                        let mut user = Users::empty_user();
                        user.surname = u.information;
                        match user.update(token_data.0) {
                            Ok(_) => Status::Ok,
                            Err(_) => Status::InternalServerError,
                        }
                    },
                    "Email" => {
                        let mut user = Users::empty_user();
                        user.email = u.information;
                        match user.update(token_data.0) {
                            Ok(_) => Status::Ok,
                            Err(_) => Status::InternalServerError,
                        }
                    },
                    "Password" => {
                        let mut user = Users::empty_user();
                        user.password = u.information;
                        match user.update(token_data.0) {
                            Ok(_) => Status::Ok,
                            Err(_) => Status::InternalServerError,
                        }
                    },
                    _ => Status::UnprocessableEntity,
                }
            },
            Err(_) => Status::Unauthorized,
        }
    },
    Err(_) => Status::BadRequest,
    }
}
/*
#[post("/user", data="<user_data>", format ="json")]
pub fn update_profile(user_data: String) -> Status{
    match serde_json::from_str::<UpdateProfileData>(&user_data) {
    Ok(u) => {
        match get_user_data_from_token(u.token) {
            Ok(token_data) => {
                match check_correct_data(token_data.0, token_data.1, token_data.2){
                    true => {
                        match u.name_field.as_str() {
                            "Name" => {
                                let mut user = Users::empty_user();
                                user.name = u.information;
                                match user.update(token_data.0) {
                                    Ok(_) => Status::Ok,
                                    Err(_) => Status::InternalServerError,
                                }
                            },
                            "Surname" => {
                                let mut user = Users::empty_user();
                                user.surname = u.information;
                                match user.update(token_data.0) {
                                    Ok(_) => Status::Ok,
                                    Err(_) => Status::InternalServerError,
                                }
                            },
                            "Email" => {
                                let mut user = Users::empty_user();
                                user.email = u.information;
                                match user.update(token_data.0) {
                                    Ok(_) => Status::Ok,
                                    Err(_) => Status::InternalServerError,
                                }
                            },
                            "Password" => {
                                let mut user = Users::empty_user();
                                user.password = u.information;
                                match user.update(token_data.0) {
                                    Ok(_) => Status::Ok,
                                    Err(_) => Status::InternalServerError,
                                }
                            },
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
}*/

#[post("/user?<token>", format = "image/jpeg", data = "<data>")]
pub async fn update_image_profile_jpeg(data: Data<'_>, token: String) -> Status{
    match get_user_data_from_token(token.clone()) {
    Ok(token_data) => {
        match check_correct_data(token_data.0, token_data.1, token_data.2){
            true => {
                let mut buffer = vec![];
                match data.open(1024.megabytes()).stream_to(&mut buffer).await{
                    Ok(_) => {
                        match std::fs::write(format!("data/image/{}.jpeg",token), buffer) {
                            Ok(_) => {
                                let old_image:String;
                                match Users::find_id(token_data.0){
                                    Ok(u)=>{
                                        old_image = u.image;
                                    },
                                    Err(_) => return Status::InternalServerError, 
                                }
                                let mut user = Users::empty_user();
                                user.image = format!("data/image/{}.jpeg",token).to_string();
                                match user.update(token_data.0){
                                    Ok(_) => {
                                        match std::fs::remove_file(old_image){
                                            Ok(_) => Status::Ok,
                                            Err(_) => {
                                                Status::InternalServerError
                                            },
                                        }
                                        },
                                    Err(_) => Status::InternalServerError,  
                                }
                            },
                            Err(_) => Status::InternalServerError,
                        }
                    },
                    Err(_) => Status::ExpectationFailed,
                }
            }
            false => Status::Unauthorized,
        }
    },
    Err(_) => Status::BadRequest,
    }   
}

#[post("/user?<token>", format = "image/png", data = "<data>")]
pub async fn update_image_profile_png(data: Data<'_>, token: String) -> Status{
    match get_user_data_from_token(token.clone()) {
    Ok(token_data) => {
        match check_correct_data(token_data.0, token_data.1, token_data.2){
            true => {
                let mut buffer = vec![];
                match data.open(1024.megabytes()).stream_to(&mut buffer).await{
                    Ok(_) => {
                        match std::fs::write(format!("data/image/{}.png",token), buffer) {
                            Ok(_) => {
                                let mut user = Users::empty_user();
                                user.image = format!("data/image/{}.png",token).to_string();
                                match user.update(token_data.0){
                                    Ok(_) => {
                                            Status::Ok
                                        },
                                    Err(_) => Status::InternalServerError,  
                                }
                            },
                            Err(_) => Status::InternalServerError,
                        }
                    },
                    Err(_) => Status::ExpectationFailed,
                }
            }
            false => Status::Unauthorized,
        }
    },
    Err(_) => Status::BadRequest,
    }   
}


//All function to manipulate subscribe 
#[post("/subscribe", data="<data_subscribe>", format ="json")]
pub fn link_subscibe_to_user(data_subscribe: String) -> Status{
    match serde_json::from_str::<TransmittedSubscribeAndUser>(&data_subscribe) {
    Ok(t_d) => {
        match get_user_data_from_token(t_d.token) {
            Ok(token_data) => {
                match check_correct_data(token_data.0, token_data.1, token_data.2){
                    true => {
                        match Subscribe::count_month(t_d.id as usize) {
                            Ok(u) => {
                                let utc:DateTime<Utc> = Utc::now();
                                match utc.checked_add_months(chrono::Months::new(u as u32)) {
                                    Some(date_time) => {
                                        let end = date_time.format("%d-%m-%Y").to_string();
                                        let subscribe_and_user = SubscribeAndUser{ id_subscribe: t_d.id as usize, id_users: token_data.0, data_end: end };
                                        match subscribe_and_user.link(){
                                            Ok(_) => return Status::Ok,
                                            Err(_) => return Status::InternalServerError,
                                        }
                                    },
                                    None => return Status::InternalServerError,      
                                };
                            },
                            Err(e) => {
                                println!("{:?}", e);
                                return Status::InternalServerError
                            },
                        }

                    },
                    false => Status::Unauthorized,
                }
            },
            Err(_) => Status::Unauthorized,
        }
    },
    Err(_) => {
        
        Status::BadRequest}
    }
}

#[post("/subscribe", data="<data_subscribe>", format ="json")]
pub fn unlink_subscibe_to_user(data_subscribe: String) -> Status{
    match serde_json::from_str::<TransmittedToken>(&data_subscribe) {
    Ok(t_d) => {
        match get_user_data_from_token(t_d.token) {
            Ok(token_data) => {
                match check_correct_data(token_data.0, token_data.1, token_data.2){
                    true => {
                        match SubscribeAndUser::delete_link(token_data.0) {
                            Ok(_) => Status::Ok,
                            Err(_) => Status::InternalServerError,
                        }
                    },
                    false => Status::Unauthorized,
                }
            },
            Err(_) => Status::Unauthorized,
        }
    },
    Err(_) => Status::BadRequest
    }
}

#[post("/subscribe", data="<data_subscribe>", format ="json")]
pub fn get_subscibe_to_profile(data_subscribe: String) -> (Status, String){
    match serde_json::from_str::<TransmittedToken>(&data_subscribe) {
    Ok(t_d) => {
        match get_user_data_from_token(t_d.token) {
            Ok(token_data) => {
                match check_correct_data(token_data.0, token_data.1, token_data.2){
                    true => {
                        match SubscribeAndUser::get_user_link(token_data.0) {
                            Ok(r) => (Status::Ok, rocket_contrib::json::Json(serde_json::json!(r)).to_string()),
                            Err(_) => (Status::InternalServerError, String::new())
                        }
                    },
                    false => (Status::Unauthorized, String::new())
                }
            },
            Err(_) => (Status::Unauthorized, String::new())
        }
    },
    Err(_) => (Status::BadRequest, String::new())
    }
}