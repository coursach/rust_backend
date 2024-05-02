
use chrono::prelude::*;
use serde_json::json;
use rocket_contrib::json::Json;
use rocket::{data::ToByteUnit, http::Status, Data};  

use crate::models::{Users, Subscribe, SubscribeAndUser, Codepromo};
use crate::transmitted_models::{UpdateProfileData, RegistrationUsers, GetUser};
use crate::function::*;


pub struct Token{
    info:String,
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
                            return Outcome::Success(Token{info:k.to_string()})
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
        match get_user_data_from_token(token.info.to_string()) {
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

#[post("/user", format = "image/jpeg", data = "<data>")]
pub async fn update_image_profile_jpeg(data: Data<'_>, token: Token) -> Status{
    match get_user_data_from_token(token.info.to_string()) {
    Ok(token_data) => {
        let mut buffer = vec![];
        match data.open(1024.megabytes()).stream_to(&mut buffer).await{
            Ok(_) => {
                let path = format!("data/image/{}.jpeg",token.info).to_string();
                match std::fs::write(path.clone(), buffer) {
                    Ok(_) => {
                        let old_image:String;
                        match Users::find_id(token_data.0){
                            Ok(u)=>{
                                old_image = u.image;
                            },
                            Err(_) => return Status::InternalServerError, 
                        }
                        let mut user = Users::empty_user();
                        user.image = path;
                        match user.update(token_data.0){
                            Ok(_) => {
                                if user.image != old_image{
                                    match std::fs::remove_file(old_image){
                                        Ok(_) => Status::Ok,
                                        Err(_) => {
                                            Status::InternalServerError
                                        },
                                    }
                                }else
                                {
                                    Status::Ok
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
    },
    Err(_) => Status::BadRequest,
    }   
}

#[post("/user", format = "image/png", data = "<data>")]
pub async fn update_image_profile_png(data: Data<'_>, token: Token) -> Status{
    match get_user_data_from_token(token.info.to_string()) {
        Ok(token_data) => {
            let mut buffer = vec![];
            match data.open(1024.megabytes()).stream_to(&mut buffer).await{
                Ok(_) => {
                    let path = format!("data/image/{}.png",token.info).to_string();
                    match std::fs::write(path.clone(), buffer) {
                        Ok(_) => {
                            let old_image:String;
                            match Users::find_id(token_data.0){
                                Ok(u)=>{
                                    old_image = u.image;
                                },
                                Err(_) => return Status::InternalServerError, 
                            }
                            let mut user = Users::empty_user();
                            user.image = path;
                            match user.update(token_data.0){
                                Ok(_) => {
                                    if user.image != old_image{
                                        match std::fs::remove_file(old_image){
                                            Ok(_) => Status::Ok,
                                            Err(_) => {
                                                Status::InternalServerError
                                            },
                                        }
                                    }else{
                                        Status::Ok
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
        },
        Err(_) => Status::BadRequest,
    }   
}

#[post("/profile")]
pub fn get_user_profile(token: Token) -> Result<String, Status>{
    match get_user_data_from_token(token.info.to_string()) {
        Ok(token_data) => {
            match Users::find_id(token_data.0) {
                Ok(u) => {
                    match SubscribeAndUser::check_exist_link(token_data.0) {
                        Ok(b) =>{
                            let user = GetUser{
                                name: u.name,
                                surname: u.surname,
                                email: u.email,
                                role: match u.role{
                                    1 => "Admin".to_string(),
                                    2 => "User".to_string(),
                                    _ => "User".to_string(),
                                },
                                image_url: format!("images/{}", u.image.to_string().split_off(11)),
                                have_subscribe: b, 
                            };
                        Ok(Json(json!(user)).to_string())
                    },
                        Err(_) => Err(Status::InternalServerError),
                    }
                    
                },
                Err(_) => Err(Status::InternalServerError),
            }
        },
        Err(_) => Err(Status::Unauthorized),
    }
}
//All function to manipulate subscribe 
#[post("/subscribe/<id>")]
pub fn link_subscibe_to_user(id:usize, token: Token) -> Status{
    match get_user_data_from_token(token.info.to_string()) {
        Ok(token_data) => {
            match Subscribe::count_month(id) {
                Ok(u) => {
                    let utc:DateTime<Utc> = Utc::now();
                    match utc.checked_add_months(chrono::Months::new(u as u32)) {
                        Some(date_time) => {
                            let end = date_time.format("%d-%m-%Y").to_string();
                            let subscribe_and_user = SubscribeAndUser{ id_subscribe: id, id_users: token_data.0, data_end: end };
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
        Err(_) => Status::Unauthorized,
    }
}

#[post("/subscribe")]
pub fn unlink_subscibe_to_user(token:Token) -> Status{
    match get_user_data_from_token(token.info.to_string()) {
        Ok(token_data) => {
            match SubscribeAndUser::delete_link(token_data.0) {
                Ok(_) => Status::Ok,
                Err(_) => Status::InternalServerError,
            }
        },
        Err(_) => Status::Unauthorized,
    }
}

#[post("/subscribe")]
pub fn get_subscibe_to_profile(token: Token) -> Result<String, Status>{
    match get_user_data_from_token(token.info) {
        Ok(token_data) => {
            match SubscribeAndUser::get_user_link(token_data.0) {
                Ok(r) => Ok(Json(json!(r)).to_string()),
                Err(_) => Err(Status::InternalServerError)
            }
        },
        Err(_) => Err(Status::Unauthorized)
    }
}

#[post("/promocode/<code>")]
pub fn get_subscibe_to_promocode(code:&str ,token:Token) -> Status{
    match Codepromo::check_for_description(code){
        Ok(date_and_id) => {
            match get_user_data_from_token(token.info.to_string()) {
                Ok(token_data) => {
                    let utc:DateTime<Utc> = Utc::now();
                    match utc.checked_add_days(chrono::Days::new(date_and_id.1 as u64)) {
                        Some(date_time) => {
                            let end = date_time.format("%d-%m-%Y").to_string();
                            let subscribe_and_user = SubscribeAndUser{ id_subscribe: date_and_id.0, id_users: token_data.0, data_end: end };
                            match subscribe_and_user.link(){
                                Ok(_) => return Status::Ok,
                                Err(_) => return Status::InternalServerError,
                            }
                        },
                        None => return Status::InternalServerError,      
                    };   
                },
                Err(_) => Status::Unauthorized,
            }
        },
        Err(_) => Status::InternalServerError,
    }
}
