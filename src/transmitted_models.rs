use serde::{Deserialize, Serialize};


#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct UpdateProfileData {
    pub name_field: String,
    pub information: String,
}

#[derive(Deserialize)]
pub struct TransmittedPromocode {
    pub description: String,
    pub id_subscribe: usize,
    pub days: usize
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct TransmittedToken {
    pub token: String,
}

#[derive(Deserialize)]
pub struct RegistrationUsers {
    pub password: String,
    pub email: String,
}

#[derive(Deserialize)]
pub struct AddingUsers {
    pub name: String,
    pub surname: String,
    pub password: String,
    pub email: String,
    pub role: i32,
}

#[derive(Deserialize)]
pub struct UpdatingUsers {
    pub id: usize,
    pub name_field: String,
    pub information: String,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct TransmittedSubscribe {
    pub name: String,
    pub count_month: usize,
    pub title: String,
    pub description: String,
    pub discount: usize,
    pub level: usize,
    pub price: usize
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct TransmittedSubscribeWithId {
    pub id: usize,
    pub name: String,
    pub count_month: usize,
    pub title: String,
    pub description: String,
    pub discount: usize,
    pub level: usize,
    pub price: usize
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct TransmittedContents {
    pub name: String,
    pub description: String,
    pub description_details: String,
    pub image_path: String,
    pub level_subscribe: usize,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ReturnedCodepromo {
    pub id: usize,
    pub description: String,
    pub idSubscribe: usize,
    pub days: usize,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ReturnedSubscribes {
    pub id: usize,
    pub name: String,
    pub count_month: i32,
    pub title: String,
    pub description: String,
    pub discount: i32,
    pub price: usize,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ReturnedContents {
    pub id: usize,
    pub name: String,
    pub description: String,
    pub description_details: String,
    pub image_path: String,
    pub level_subscribe: usize,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ReturnedAllInfoContent {
    pub id: usize,
    pub name: String,
    pub description: String,
    pub description_details: String,
    pub image_path: String,
    pub level_subscribe: usize,
    pub actor: Vec<Actor>,
    pub director: Vec<Actor>,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Actor {
    pub name: String,
    pub surname: String,
}
#[derive(Serialize, Deserialize)]
pub struct ReturnedSubscribe {
    pub id: usize,
    pub name: String,
    pub dead_line: String,
    pub title: String,
    pub description: String,
    pub discount: i32,
    pub level: usize,
    pub price: usize,
}

#[derive(Serialize)]
pub struct ReturnedFile {
    pub id: usize,
    pub idContent: usize,
    pub path: String
}

#[derive(Deserialize)]
pub struct TransmittedFile {
    pub idContent: usize,
    pub path: String
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct GetUser {
    pub name: String,
    pub surname: String,
    pub email: String,
    pub role: String,
    pub image_url: String,
    pub have_subscribe: bool,
}
#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct GetAdminUsers {
    pub id: usize,
    pub name: String,
    pub surname: String,
    pub password: String,
    pub email: String,
    pub image: String,
    pub role: usize,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct AddAdminUsers {
    pub id: usize,
    pub name: String,
    pub surname: String,
    pub password: String,
    pub email: String,
    pub image: String,
    pub role: usize,
}