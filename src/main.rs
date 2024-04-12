
//#[macro_use] extern crate rocket;              // like document
pub mod function;
pub mod models;

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

/*
#[get("/")]
fn index() -> &'static str {
    let connections = String::from("./database/cinemadb.db");


    let db = Users{
        fname: connections,
    };

    match  db.add("name", "surname", "password", "email", 123) {
        Ok(_) => {
            "Success add new user"
        }
        Err(UserErr::DbErr(ref err)) => {
            "error"
        }
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}

*/
