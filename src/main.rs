
use models::*;
#[macro_use] extern crate rocket; 
use rocket::http::Status;             
pub mod function;
pub mod models;
mod claims;
use claims::Claims;
use serde_json::json;
use rocket_contrib::json::Json;

/* 
fn main(){
    /*let db = models::Users{
        name: "".to_string(),
        surname: "".to_string(),
        password: "".to_string(),
        role: 0,
        email: "".to_string(),
    };*/
    match  models::Users::all() {
        Ok(res) => {
            for user in res{
                println!("{} {} {} {} {}", user.name, user.surname, user.email, user.role, user.password)
            }
        },
        Err(_) =>{println!("error")},
    }
}
*/


#[post("/check", format = "application/json", data = "<input>")]
pub fn check(input: String) -> String{
    match serde_json::from_str::<UserData>(&input) {
    Ok(u) => {
        match Users::check_user_exsist(&u.email) {
            Ok(v) => Json(json!({
                "status": 200,
                "result": v
            })).to_string(),
            Err(_) => todo!(),
        }
    },
    Err(e) => {
        Json(json!({
            "status": 200,
            "result": e.to_string()
        })).to_string()
    },   
    }
}

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
                    match u {
                        true => {
                            let claim = Claims::from_name(&v.email);
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

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, get_all_subscribe, check, login])
}


