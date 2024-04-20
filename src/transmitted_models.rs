
use serde::{Deserialize, Serialize};


#[derive(Deserialize)]
pub struct LoginRequest{
    pub email: String,
    pub password: String   
}

#[derive(Deserialize)]
pub struct UpdateProfileData{
    pub token: String,
    pub name_field: String,
    pub information: String,
}

#[derive(Serialize, Deserialize)]
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
pub struct ReturnedSubscribe {
    pub id: usize,
    pub name : String,
    pub count_month :i32,
    pub title : String,
    pub description : String,
    pub discount :i32,
}

#[derive(Deserialize)]
pub struct TransmittedSubscribeAndUser{
    pub token: String,
    pub id: usize,
}
