
use crate::{claims::AuthenticationError, Claims, Users};
pub fn get_user_data_from_token(token: String) -> Result<(usize, String, String), AuthenticationError>{
    let token = format!("Bearer {token}");
    match Claims::from_authorization(&token){
        Ok(c) => {
            let vec:Vec<&str> = c.name.split(':').collect();
            if vec.len() == 3{
                let id:usize;
                match vec.get(0).unwrap_or(&"0").parse::<usize>() {
                    Ok(u) => {
                        if u != 0 {
                            id = u;
                        }
                        else{
                            return Err(AuthenticationError::Missing) 
                        }
                    },
                    Err(_) => return Err(AuthenticationError::Missing),
                }
                let email = vec.get(1).unwrap_or(&"").to_string();
                let password = vec.get(2).unwrap_or(&"").to_string();
                return Ok((id, email, password));
            }else{
                return Err(AuthenticationError::Missing);

            }
            },
        Err(e) => Err(e),    
    }
}

pub fn check_correct_data(id: usize, email: String, password: String) -> bool{
    match Users::find_id(id) {
        Ok(u) => {
            if u.email == email{
                if u.password == password{
                    return true;
                }
            }
            false
        },
        Err(_) => false,
    }
}

pub fn check_is_admin_with_token(token: String) -> Result<bool, AuthenticationError>{
    let user_data = get_user_data_from_token(token)?;
    match check_correct_data(user_data.0, user_data.1, user_data.2){
        true => {
            match Users::find_id(user_data.0) {
                Ok(u) => {
                    Ok(u.role == 1)
                },
                Err(_) => Err(AuthenticationError::Missing),
            }
        },
        false => Err(AuthenticationError::Expired),
    }
}

pub fn check_is_user_correct_with_token(token: String) -> Result<bool, AuthenticationError>{
    let user_data = get_user_data_from_token(token)?;
    match check_correct_data(user_data.0, user_data.1, user_data.2){
        true => {
            Ok(true)
        },
        false => Err(AuthenticationError::Expired),
    }
}