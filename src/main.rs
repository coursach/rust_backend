
use function::{check_correct_data, get_user_data_from_token};
use models::*;
#[macro_use] extern crate rocket; 
use rocket::{data::ToByteUnit, http::Status, tokio, Data};             
pub mod function;
pub mod models;
mod claims;
use claims::Claims;
use serde_json::json;
use rocket_contrib::json::Json;

#[get("/subscribe", format = "application/json")]
pub fn get_all_subscribe() -> String{
    match Subscribe::all() {
    Ok(v) => Json(json!({
        "status": 200,
        "result": v
    })).to_string(),
    Err(_) => todo!(),
    }
}


#[get("/")]
fn index() -> &'static str {
    "hi"
}

#[post("/login", data="<login_string>")]
fn login(login_string: String) -> String{
    match serde_json::from_str::<LoginRequest>(&login_string) {
        Ok(v)=>{
            match Users::login(v.email.clone(), v.password.clone()) {
                Ok(u) => {
                    match u.0 {
                        true => {
                            let claim = Claims::from_name(&format!("{}:{}:{}",u.1 , v.email, v.password));
                            let response = LoginResponse{
                                token: match claim.into_token(){
                                    Ok(s) => s,
                                    Err(_) => todo!(),
                                }
                            };
                            Json(json!(response)).to_string()
                        },
                        false => Json(json!({
                            "status": Status::Unauthorized,
                            "result": "нет"
                        })).to_string(),
                    }
                },
                Err(_) => Json(json!({
                    "status": Status::Unauthorized,
                    "result": "null"
                })).to_string(),
            }
        }, 
        Err(e)=>Json(json!({
            "status": 200,
            "result": e.to_string()
        })).to_string(),
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
                            "ImageProfileFile" => {
                                let mut user = Users::empty_user();
                                user.image = u.information;
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

#[post("/upload", format = "image/jpeg", data = "<data>")]
async fn upload(data: Data<'_>) -> std::io::Result<()>{
    let mut buffer = vec![];
    data.open(1024.megabytes()).stream_to(&mut buffer).await?;
    std::fs::write("foo.jpeg", buffer);
    Ok(())
}
/*
#[post("/private", data="<token_string>")]
fn private(token_string: String) -> String{
    match serde_json::from_str::<LoginResponse>(&token_string) {
        Ok(l) => {
            let token = l.token;
            
        },
        Err(e) => e.to_string(),
    }
    
}*/

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, get_all_subscribe, login, upload])
    .mount("/update", routes![update_user])
}


