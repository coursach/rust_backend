
use crate::{claims::AuthenticationError, Claims, Users};
pub fn get_user_data_from_token(token: String) -> Result<(usize, String, String), String>{
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
                            return Err(format!("{:?}", AuthenticationError::Missing)) 
                        }
                    },
                    Err(_) => return Err(format!("{:?}", AuthenticationError::Missing)),
                }
                let email = vec.get(1).unwrap_or(&"").to_string();
                let password = vec.get(2).unwrap_or(&"").to_string();
                return Ok((id, email, password));
            }else{
                return Err(format!("{:?}", AuthenticationError::Missing));

            }
            },
        Err(e) => Err(format!("{:?}", e)),    
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