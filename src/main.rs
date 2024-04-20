
#[macro_use] extern crate rocket; 

use rocket::{data::ToByteUnit, http::Status, Data}; 
use serde_json::json;
use rocket_contrib::json::Json;
use function::*;
use models::*;
use transmitted_models::*;
use claims::Claims;
use chrono::prelude::*;

mod function;
pub mod models;
mod transmitted_models;
mod claims;


#[get("/subscribe")]
fn get_all_subscribe() ->(Status, String){
    match Subscribe::all() {
        Ok(v) =>(Status::Ok, Json(json!({"result": v})).to_string()),
        Err(e) => (Status::InternalServerError, Json(json!({"result": format!("{:?}", e)})).to_string()),
    }
}

#[post("/subscribe", data="<data_subscribe>")]
fn add_subscibe_to_user(data_subscribe: String) -> Status{
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


#[post("/subscribe", data="<data_subscribe>")]
fn add_subscibe(data_subscribe: String) -> Status{
    match serde_json::from_str::<TransmittedSubscribe>(&data_subscribe) {
    Ok(t_d) => {
        match get_user_data_from_token(t_d.token) {
            Ok(token_data) => {
                match check_correct_data(token_data.0, token_data.1, token_data.2){
                    true => {
                        match Users::find_id(token_data.0){
                            Ok(u) => {
                                if u.role == 1{
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
                                } 
                                else {
                                    Status::Forbidden
                                }
                            },
                            Err(_) => Status::ServiceUnavailable,
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
/*
#[post("/subscribe", data="<data_subscribe>")]
fn update_subscibe(data_subscribe: String) -> Status{
    match serde_json::from_str::<TransmittedSubscribe>(&data_subscribe) {
    Ok(t_d) => {
        match get_user_data_from_token(t_d.token) {
            Ok(token_data) => {
                match check_correct_data(token_data.0, token_data.1, token_data.2){
                    true => {
                        match Users::find_id(token_data.0){
                            Ok(u) => {
                                if u.role == 1{
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
                                } 
                                else {
                                    Status::Forbidden
                                }
                            },
                            Err(_) => Status::ServiceUnavailable,
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
*/
#[post("/login", data="<login_string>")]
fn login(login_string: String) ->(Status, String){
    match serde_json::from_str::<LoginRequest>(&login_string) {
        Ok(v)=>{
            match Users::login(v.email.clone(), v.password.clone()) {
                Ok(u) => {
                    match u.0 {
                        true => {
                            let claim = Claims::from_name(&format!("{}:{}:{}",u.1 , v.email, v.password));
                            let response = TransmittedToken{
                                token: match claim.into_token(){
                                    Ok(s) => s,
                                    Err(_) => todo!(),
                                }
                            };
                            (Status::Ok, Json(json!(response)).to_string())
                        },
                        false => (Status::Unauthorized, Json(json!({
                            "result": "нет"
                        })).to_string()),
                    }
                },
                Err(_) => (Status::Unauthorized, Json(json!({
                    "result": "null"
                })).to_string()),
            }
        }, 
        Err(e)=>(Status::Ok, Json(json!({
            "result": e.to_string()
        })).to_string()),
    }
}

#[post("/user", data="<user_data>")]
fn update_user(user_data: String) -> Status{
    match serde_json::from_str::<UserData>(&user_data) {
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
}

#[post("/user?<token>", format = "image/jpeg", data = "<data>")]
async fn update_image_jpeg(data: Data<'_>, token: String) -> Status{
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
async fn update_image_png(data: Data<'_>, token: String) -> Status{
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

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![get_all_subscribe, login])
    .mount("/update", routes![update_user, update_image_jpeg, update_image_png])
    .mount("/add", routes![add_subscibe])
    .mount("/link", routes![add_subscibe_to_user])
}


