
use rocket::http::Status; 
use crate::function::*;
use crate::models::{Users, Subscribe};
use crate::transmitted_models::{AddingUsers, TransmittedSubscribe, UpdatingUsers};

// All admin function for manipulating users

#[post("/user", data="<user_data>", format ="json")]
pub fn add_user(user_data: String) -> Status{
    match serde_json::from_str::<AddingUsers>(&user_data) {
        Ok(t_u) => {
            match check_is_admin_with_token(t_u.token) {
                Ok(u) => {
                    match u {
                        true => {
                            let user = Users{ name: t_u.name, surname: t_u.surname, password: t_u.password, email: t_u.email, image: "".to_string(), role: t_u.role };
                            match user.add(){
                                Ok(_) => Status::Created,
                                Err(_) => Status::Conflict,
                            }    
                            },
                        false => Status::Unauthorized,   
                    }
                },
                Err(_) => todo!(),
            }
        },
        Err(_) => Status::BadRequest,   
    } 
}

#[post("/user", data="<user_data>", format ="json")]
pub fn update_user(user_data: String) -> Status{
    match serde_json::from_str::<UpdatingUsers>(&user_data) {
        Ok(t_u) => {
            match check_is_admin_with_token(t_u.token) {
                Ok(u) => {
                    match u {
                        true => {
                            match t_u.name_field.as_str() {
                                "Name" => {
                                    let mut user = Users::empty_user();
                                    user.name = t_u.information;
                                    match user.update(t_u.id) {
                                        Ok(_) => Status::Ok,
                                        Err(_) => Status::InternalServerError,
                                    }
                                },
                                "Surname" => {
                                    let mut user = Users::empty_user();
                                    user.surname = t_u.information;
                                    match user.update(t_u.id) {
                                        Ok(_) => Status::Ok,
                                        Err(_) => Status::InternalServerError,
                                    }
                                },
                                "Email" => {
                                    let mut user = Users::empty_user();
                                    user.email = t_u.information;
                                    match user.update(t_u.id) {
                                        Ok(_) => Status::Ok,
                                        Err(_) => Status::InternalServerError,
                                    }
                                },
                                "Password" => {
                                    let mut user = Users::empty_user();
                                    user.password = t_u.information;
                                    match user.update(t_u.id) {
                                        Ok(_) => Status::Ok,
                                        Err(_) => Status::InternalServerError,
                                    }
                                },
                                "Role" =>{
                                    let mut user = Users::empty_user();
                                    match t_u.information.parse::<i32>() {
                                        Ok(role) => {
                                            user.role = role;
                                            match user.update(t_u.id) {
                                                Ok(_) => Status::Ok,
                                                Err(_) => Status::InternalServerError,
                                            }
                                        },
                                        Err(_) => Status::BadRequest,  
                                    }
                                }
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



// All admin function for manipulating subscriptions
#[post("/subscribe", data="<data_subscribe>", format ="json")]
pub fn update_subscibe(data_subscribe: String) -> Status{
    match serde_json::from_str::<TransmittedSubscribe>(&data_subscribe) {
    Ok(t_d) => {
        match check_is_admin_with_token(t_d.token) {
            Ok(u) => {
                match u{
                    true => {
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
                    },
                    false => Status::Unauthorized,   
                }
            },
            Err(_) => Status::ServiceUnavailable,
        }
    },
    Err(_) => Status::BadRequest,
    }
}

#[post("/subscribe", data="<data_subscribe>", format ="json")]
pub fn add_subscibe(data_subscribe: String) -> Status{
    match serde_json::from_str::<TransmittedSubscribe>(&data_subscribe) {
    Ok(t_d) => {
        match check_is_admin_with_token(t_d.token) {
            Ok(u) => {
                match u {
                    true => {
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
                    }, 
                    false => {
                        Status::Forbidden
                    }
                    }
                },
                Err(_) => Status::ServiceUnavailable,
                } 
            },
    Err(_) => Status::BadRequest,
    }
}