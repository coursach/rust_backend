pub struct Codepromo{
    pub id:i32,
    pub description: String,
    pub id_subscribe:i32
}

pub struct Content{
    pub id:i32,
    pub name: String, 
    pub description: String, 
    pub description_details: String
}

pub struct ContentForPreferences {
    pub id:i32,
    pub id_content :i32,
    pub id_users :i32,
}

pub struct File {
    pub id :i32,
    pub id_content :i32,
    pub path : String,
}

pub struct History {
    pub id :i32,
    pub id_user :i32,
    pub id_content :i32,
    pub end_see :i32,
}

pub struct Role {
    pub id :i32,
    pub name : String,
}

pub struct Subscribe {
    pub id :i32,
    pub name : String,
    pub count_month :i32,
    pub title : String,
    pub description : String,
    pub discount :i32,
}

pub struct SubscribeAndUser {
    pub id :i32,
    pub id_subscribe :i32,
    pub id_users :i32,
    pub data_end : String,
}
/* 
pub struct Users{
    pub id: i32,
    pub name: String,
    pub surname: String,
    pub password: String,
    pub email: String,
    pub role: i32,
}*/

pub struct Users{
    pub fname:String,
}

pub struct Workers {
    pub id :i32,
    pub name : String,
    pub surname : String,
    pub id_content :i32,
    pub role :i32,
}