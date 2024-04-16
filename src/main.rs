


use models::*;
use rocket_contrib::json::Json;
use serde_json::json;
#[macro_use] extern crate rocket;              // like document
pub mod function;
pub mod models;

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

#[post("/users", format = "application/json")]
pub fn get_all() -> String{
    match Users::all() {
    Ok(v) => Json(json!({
        "status": 200,
        "result": v
    })).to_string(),
    Err(_) => todo!(),
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
    let user = Users{ name: "Антон".to_string(), surname: "Яковлев".to_string(), password: "best".to_string(), email: "anton".to_string(), role: 2, image: "null".to_string() };
    match user.add() {
        Ok(s) => s.leak(),
        Err(e) => format!("{:?}", e).leak(),
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, get_all_subscribe])
}


