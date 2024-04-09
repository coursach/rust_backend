use sqlite::Error as sqERR;
use crate::models::*;

//#[macro_use] extern crate rocket;              // like document
pub mod models;

#[derive(Debug)]
pub enum UserErr{
    DbErr(sqERR),
}

impl From<sqERR> for UserErr {
    fn from(s:sqERR)->Self{
        UserErr::DbErr(s)
    }
}

impl Users {
    pub fn add(&self, name:&str, surname:&str, password:&str, email:&str, role:i32) -> Result<(), UserErr>{
        let connection = sqlite::open(&self.fname)?;
        let mut db = connection.prepare("INSERT INTO users ('Name', 'Surname', 'Password', 'Email', 'Role') VALUES (?, ?, ?, ?, ?);")?;
        db.bind::<&[(_, &str)]>(&[
            (1, name),
            (2, surname),
            (3, password),
            (4, email),
            (5, &(role.to_string()))
        ][..])?;
        db.next()?;
        Ok(())
    }
}

fn main(){
    let connections = String::from("./database/cinemadb.db");


    let db = Users{
        fname: connections,
    };

    match  db.add("name", "surname", "password", "email", 123) {
        Ok(_) => {
            println!("Success add new user");
        }
        Err(UserErr::DbErr(ref err)) => {
            println!("{:?}", err);
        }
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
