
use models::*;
#[macro_use] extern crate rocket;              // like document
pub mod function;
pub mod models;
use serde_json::json;
use serde::Deserialize;
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
#[derive(Deserialize)]
pub struct UserData{
    pub email: String,
}

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

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, get_all_subscribe, check])
}


