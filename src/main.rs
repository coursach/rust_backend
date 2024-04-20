
#[macro_use] extern crate rocket; 

use rocket::http::Status; 
use serde_json::json;
use rocket_contrib::json::Json;

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

#[get("/subscribe")]
fn get_all_subscribe() ->(Status, String){
    match Subscribe::all() {
        Ok(v) =>(Status::Ok, Json(json!({"result": v})).to_string()),
        Err(e) => (Status::InternalServerError, Json(json!({"result": format!("{:?}", e)})).to_string()),
    }
}

#[post("/login", data="<login_string>", format ="json")]
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



#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![get_all_subscribe, login])
    .mount("/user/update", routes![update_profile, update_image_profile_jpeg, update_image_profile_png])
    .mount("/user/link", routes![link_subscibe_to_user])
    .mount("/user/unlink", routes![unlink_subscibe_to_user])
    .mount("/user/get", routes![get_subscibe_to_profile])
    .mount("/registration", routes![registration_user])
    .mount("/admin/update", routes![update_subscibe, update_user])
    .mount("/admin/add", routes![add_subscibe, add_user])
}


