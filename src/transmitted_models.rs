
use serde::{Deserialize, Serialize};


#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct LoginRequest{
    pub email: String,
    pub password: String   
}

#[derive(Deserialize)]
pub struct UpdateProfileData{
    pub name_field: String,
    pub information: String,
}


#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct TransmittedToken{
    pub token: String
}

#[derive(Deserialize)]
pub struct RegistrationUsers{
    pub password: String,
    pub email: String,
}

#[derive(Deserialize)]
pub struct AddingUsers{
    pub token: String,
    pub name: String,
    pub surname: String,
    pub password: String,
    pub email: String,
    pub role: i32,
}

#[derive(Deserialize)]
pub struct UpdatingUsers{
    pub token: String,
    pub id: usize,
    pub name_field: String,
    pub information: String,
}

#[derive(Deserialize)]
pub struct TransmittedSubscribe {
    pub token: String,
    pub name : String,
    pub count_month :i32,
    pub title : String,
    pub description : String,
    pub discount :i32,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ReturnedSubscribes {
    pub id: usize,
    pub name : String,
    pub count_month :i32,
    pub title : String,
    pub description : String,
    pub discount :i32,
    pub price: usize,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ReturnedContens {
    pub id: usize,
    pub name : String,
    pub description: String,
    pub description_details: String,
    pub image_path: String,
    pub level_subscribe: usize,
}
#[derive(Serialize)]
pub struct ReturnedSubscribe {
    pub id: usize,
    pub name : String,
    pub dead_line :String,
    pub title : String,
    pub description : String,
    pub discount :i32,
    pub level: usize,
    pub price: usize,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct GetUser{
    pub name: String,
    pub surname: String,
    pub email: String,
    pub role: String,
    pub image_url: String,
    pub have_subscribe: bool,
}