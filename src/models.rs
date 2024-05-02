
use std::usize;

use serde::{Deserialize, Serialize};
use sqlite::State;

use crate::{ReturnedSubscribes, ReturnedSubscribe};

pub mod err{
    use sqlite::Error as sqERR;
    #[derive(Debug)]
    pub enum UserErr{
        DbErr(sqERR),
    }
    #[derive(Debug)]
    pub enum WorkersErr{
        DbErr(sqERR),
    }
    #[derive(Debug)]
    pub enum SubscribeErr{
        DbErr(sqERR),
    }
    #[derive(Debug)]
    pub enum RoleErr{
        DbErr(sqERR),
    }
    #[derive(Debug)]
    pub enum SubscribeAndUserErr{
        DbErr(sqERR),
    }
    #[derive(Debug)]
    pub enum HistoryErr{
        DbErr(sqERR),
    }
    #[derive(Debug)]
    pub enum FileErr{
        DbErr(sqERR),
    }
    #[derive(Debug)]
    pub enum ContentForPreferencesErr{
        DbErr(sqERR),
    }
    #[derive(Debug)]
    pub enum ContentErr{
        DbErr(sqERR),
    }
    #[derive(Debug)]
    pub enum CodepromoErr{
        DbErr(sqERR),
    }

    impl From<sqERR> for UserErr {
        fn from(s:sqERR)->Self{
            UserErr::DbErr(s)
        }
    }
    impl From<sqERR> for WorkersErr {
        fn from(s:sqERR)->Self{
            WorkersErr::DbErr(s)
        }
    }
    impl From<sqERR> for SubscribeErr {
        fn from(s:sqERR)->Self{
            SubscribeErr::DbErr(s)
        }
    }
    impl From<sqERR> for SubscribeAndUserErr {
        fn from(s:sqERR)->Self{
            SubscribeAndUserErr::DbErr(s)
        }
    }
    impl From<sqERR> for RoleErr {
        fn from(s:sqERR)->Self{
            RoleErr::DbErr(s)
        }
    }
    impl From<sqERR> for CodepromoErr {
        fn from(s:sqERR)->Self{
            CodepromoErr::DbErr(s)
        }
    }
    impl From<sqERR> for ContentErr {
        fn from(s:sqERR)->Self{
            ContentErr::DbErr(s)
        }
    }
    impl From<sqERR> for ContentForPreferencesErr {
        fn from(s:sqERR)->Self{
            ContentForPreferencesErr::DbErr(s)
        }
    }
    impl From<sqERR> for FileErr {
        fn from(s:sqERR)->Self{
            FileErr::DbErr(s)
        }
    }
    impl From<sqERR> for HistoryErr {
        fn from(s:sqERR)->Self{
            HistoryErr::DbErr(s)
        }
    }
}
pub struct Codepromo{
    pub description: String,
    pub id_subscribe:i32
}

pub struct Content{
    pub name: String, 
    pub description: String, 
    pub description_details: String
}

pub struct ContentForPreferences {
    pub id_content :i32,
    pub id_users :i32,
}

pub struct File {
    pub id_content :i32,
    pub path : String,
}

pub struct History {
    pub id_user :i32,
    pub id_content :i32,
}

pub struct Role {
    pub name : String,
}

#[derive(Deserialize, Serialize)]
pub struct Subscribe {
    pub name : String,
    pub count_month :i32,
    pub title : String,
    pub description : String,
    pub discount :i32,
}

pub struct SubscribeAndUser {
    pub id_subscribe :usize,
    pub id_users :usize,
    pub data_end : String,
}

pub struct Users{
    pub name: String,
    pub surname: String,
    pub password: String,
    pub email: String,
    pub image: String,
    pub role: i32,
}

pub struct Workers {
    pub name : String,
    pub surname : String,
    pub id_content :i32,
    pub role :i32,
}

impl Users {
    pub fn empty_user() -> Users{
        Users { name: "".to_string(), surname: "".to_string(), password: "".to_string(), email: "".to_string(), image: "".to_string(), role: 0 }
    }

    pub fn login(email: String, password:String)-> Result<(bool, usize), err::UserErr>{
        let connection = sqlite::open("./data/cinemadb.db")?;
        let mut db = connection.prepare("SELECT Id FROM users where Email = ? and Password = ?;")?;
        db.bind::<&[(_, &str)]>(&[
            (1, email.as_str()),
            (2, password.as_str()),
        ][..])?;
        db.next()?;
        let res = db.read::<String, _>(0).unwrap().parse::<usize>().unwrap();
        match res != 0 {
            true => Ok((true, res)),
            false => Ok((false, 0)),
        }
    }

    pub fn add(&self) -> Result<(), err::UserErr>{
        let connection = sqlite::open("./data/cinemadb.db")?;
        match Users::check_user_exsist(&self.email) {
            Ok(b) => {
                if !b {
                    let mut db = connection.prepare("INSERT INTO users ('Name', 'Surname', 'Password', 'Email', 'Role', 'ImageProfileFile') VALUES (?, ?, ?, ?, ?, ?);")?;
                    db.bind::<&[(_, &str)]>(&[
                        (1, self.name.as_str()),
                        (2, self.surname.as_str()),
                        (3, self.password.as_str()),
                        (4, self.email.as_str()),
                        (5, &(self.role.to_string())),
                        (6, self.image.as_str())
                    ][..])?;
                    db.next()?;
                    Ok(())
                }else{
                    return Err(err::UserErr::DbErr(sqlite::Error { code: Some(12), message: Some("rep".to_string()) }));
                }
            },
            Err(e) => return Err(e),
        }
        
    }

    pub fn find_id(id:usize) -> Result<Users, err::UserErr>{
        let connection = sqlite::open("./data/cinemadb.db")?;
        let mut db = connection.prepare("SELECT * FROM users where Id = ?;")?;
        db.bind::<&[(_, &str)]>(&[
            (1, id.to_string().as_str()),
        ][..])?;
        db.next()?; 
        Ok(Users{
            name: db.read(1)?,
            surname: db.read(2)?,
            password: db.read(3)?,
            email: db.read(4)?,
            role: db.read::<String, _>(5).unwrap().parse::<i32>().unwrap(),
            image: db.read(6)?,
        } )
    }

    fn check_user_exsist(email:&str)->Result<bool, err::UserErr>{
        let connection = sqlite::open("./data/cinemadb.db")?;
        let mut db = connection.prepare("SELECT Id FROM users where Email = ?;")?;
        db.bind::<&[(_, &str)]>(&[
            (1, email),
        ][..])?;
        db.next()?;
        let res = db.read::<String, _>(0).unwrap_or("0".to_string()).parse::<i32>().unwrap_or_default();
        if res != 0{ 
            Ok(true)
        }else{
            Ok(false)
        }
    }

    pub fn update(&self, id:usize) -> Result<(), err::UserErr>{
        let connection = sqlite::open("./data/cinemadb.db")?;
        let mut changed = false;
        if self.name != "" {
            let mut db = connection.prepare("UPDATE users SET Name = ? WHERE Id = ?;")?;
            db.bind::<&[(_, &str)]>(&[
                (1, self.name.as_str()),
                (2, id.to_string().as_str())
            ][..])?;
            db.next()?;
            changed = true;
        }
        if self.surname != "" {
            let mut db = connection.prepare("UPDATE users SET Surname = ? WHERE Id = ?;")?;
            db.bind::<&[(_, &str)]>(&[
                (1, self.surname.as_str()),
                (2, id.to_string().as_str())
            ][..])?;
            db.next()?;
            changed = true;
        }
        if self.password != "" {
            let mut db = connection.prepare("UPDATE users SET Password = ? WHERE Id = ?;")?;
            db.bind::<&[(_, &str)]>(&[
                (1, self.password.as_str()),
                (2, id.to_string().as_str())
            ][..])?;
            db.next()?;
            changed = true;
        }
        if self.email != "" {
            let mut db = connection.prepare("UPDATE users SET Email = ? WHERE Id = ?;")?;
            db.bind::<&[(_, &str)]>(&[
                (1, self.email.as_str()),
                (2, id.to_string().as_str())
            ][..])?;
            db.next()?;
            changed = true;
        }
        if self.role != 0 {
            let mut db = connection.prepare("UPDATE users SET Role = ? WHERE Id = ?;")?;
            db.bind::<&[(_, &str)]>(&[
                (1, self.role.to_string().as_str()),
                (2, id.to_string().as_str())
            ][..])?;
            db.next()?;
            changed = true;
        }
        if self.image != "" {
            let mut db = connection.prepare("UPDATE users SET ImageProfileFile = ? WHERE Id = ?;")?;
            db.bind::<&[(_, &str)]>(&[
                (1, self.image.to_string().as_str()),
                (2, id.to_string().as_str()),
            ][..])?;
            db.next()?;
            changed = true;
        }
        if changed {
            return Ok(());
        }else{
            return Err(err::UserErr::DbErr(sqlite::Error{code: Some(300), message: rocket::serde::__private::Option::Some("".to_string()) }));
        }
    }
}

impl Workers {
    pub fn add(&self) -> Result<(), err::WorkersErr>{
        let connection = sqlite::open("./data/cinemadb.db")?;
        let mut db = connection.prepare("INSERT INTO workers ('Name', 'Surname', 'IdContent', 'Role') VALUES (?, ?, ?, ?);")?;
        db.bind::<&[(_, &str)]>(&[
            (1, self.name.as_str()),
            (2, self.surname.as_str()),
            (3, &self.id_content.to_string()),
            (4, &self.role.to_string())
        ][..])?;
        db.next()?;
        Ok(())
    }
}

impl Subscribe{
    pub fn add(&self) -> Result<(), err::SubscribeErr>{
        let connection = sqlite::open("./data/cinemadb.db")?;
        let mut db = connection.prepare("INSERT INTO subscribe ('Name', 'Count_mouth', 'Title', 'Description','Discount') VALUES (?, ?, ?, ?, ?);")?;
        db.bind::<&[(_, &str)]>(&[
            (1, self.name.as_str()),
            (2, &self.count_month.to_string()),
            (3, self.title.as_str()),
            (4, self.description.as_str()),
            (5, &self.discount.to_string())
        ][..])?;
        db.next()?;
        Ok(())
    }

    pub fn all() -> Result<Vec<ReturnedSubscribes>, err::SubscribeErr>{
        let connection = sqlite::open("./data/cinemadb.db")?;
        let mut res:Vec<ReturnedSubscribes> = Vec::new();
        let mut db = connection.prepare("SELECT * FROM subscribe;")?;
        while let State::Row = db.next()? {
            let ret = ReturnedSubscribes{
                id: db.read::<String, _>(0).unwrap().parse::<usize>().unwrap(),
                name: db.read(1)?,
                count_month: db.read::<String, _>(2).unwrap().parse::<i32>().unwrap(),
                title: db.read(3)?,
                description: db.read(4)?,
                discount: db.read::<String, _>(5).unwrap().parse::<i32>().unwrap(),
            } ;
            res.push(ret);
        }
        Ok(res)
    }

    pub fn count_month(id: usize) ->Result<usize, err::SubscribeErr>{
        let connection = sqlite::open("./data/cinemadb.db")?;
        let mut res:usize = 0;
        let mut db = connection.prepare("SELECT Count_month FROM subscribe where Id = ?;")?;
        db.bind::<&[(_, &str)]>(&[
            (1, id.to_string().as_str()),
        ][..])?;
        while let State::Row = db.next()? {
            res = db.read::<String, _>(0).unwrap().parse::<usize>().unwrap();
        }
        Ok(res)
    }

}

impl SubscribeAndUser{
    pub fn link(&self) -> Result<(), err::SubscribeAndUserErr>{
        let connection = sqlite::open("./data/cinemadb.db")?;
        match SubscribeAndUser::check_exist_link(self.id_users){
            Ok(b) => {
                if b {
                    let mut db = connection.prepare("UPDATE subscribe_and_user SET IdSubscribe = ?, DataEnd = ? WHERE IdUsers = ?;")?;
                    db.bind::<&[(_, &str)]>(&[
                        (1, self.id_subscribe.to_string().as_str()),
                        (2, self.data_end.to_string().as_str()),
                        (3, self.id_users.to_string().as_str()),
                    ][..])?;
                    db.next()?;
                }else{
                    let mut db = connection.prepare("INSERT INTO subscribe_and_user ('IdSubscribe', 'IdUsers', 'DataEnd') VALUES (?, ?, ?);")?;
                    db.bind::<&[(_, &str)]>(&[
                        (1, self.id_subscribe.to_string().as_str()),
                        (2, self.id_users.to_string().as_str()),
                        (3, self.data_end.as_str()),
                    ][..])?;
                    db.next()?;
                }
                Ok(())
            },
            Err(_) => todo!(),
        }
        
    }

    pub fn check_exist_link(id_user:usize) -> Result<bool, err::SubscribeAndUserErr> {
        let connection = sqlite::open("./data/cinemadb.db")?;
        let mut db = connection.prepare("SELECT Id FROM subscribe_and_user WHERE IdUsers = ?;")?;
        db.bind::<&[(_, &str)]>(&[
            (1, id_user.to_string().as_str()),
        ][..])?;
        db.next()?;
        let result = match db.read::<i64, _>(0){
            Ok(u) => u as usize,
            Err(_) => 0 as usize, 
        };
        if result == 0{
            Ok(false)
        }
        else{
            Ok(true)
        }
    }

    pub fn delete_link(id_user: usize) -> Result<(), err::SubscribeAndUserErr>{
        let connection = sqlite::open("./data/cinemadb.db")?;
        match SubscribeAndUser::check_exist_link(id_user){
            Ok(b) => {
                if b {
                let mut db = connection.prepare("DELETE FROM subscribe_and_user WHERE IdUsers = ?;")?;
                db.bind::<&[(_, &str)]>(&[
                    (1, id_user.to_string().as_str()),
                ][..])?;
                db.next()?;
                Ok(())
                }else {
                    Ok(())
                }
            },
            Err(e) => Err(e),
        }
    }

    pub fn get_user_link(id_user:usize) -> Result<ReturnedSubscribe, err::SubscribeAndUserErr> {
        let connection = sqlite::open("./data/cinemadb.db")?;
        let mut db = connection.prepare("SELECT * FROM subscribe_and_user WHERE IdUsers = ?;")?;
        db.bind::<&[(_, &str)]>(&[
            (1, id_user.to_string().as_str()),
        ][..])?;
        db.next()?;
        let mut db1 = connection.prepare("SELECT * FROM subscribe WHERE Id = ?;")?;
        let id:String = db.read(1)?;
        db1.bind::<&[(_, &str)]>(&[
            (1, id.as_str()),
        ][..])?;
        db1.next()?;
        return Ok(ReturnedSubscribe{ 
            id: db1.read::<String, _>(0).unwrap().parse::<usize>().unwrap(),
            name: db1.read(1)?,
            dead_line: db.read(3)?,
            title: db1.read(3)?,
            description: db1.read(4)?,
            discount: db1.read::<String, _>(5).unwrap().parse::<i32>().unwrap(),
            level: db1.read::<String, _>(6).unwrap().parse::<usize>().unwrap()
         });
    }
}

impl Role{
    pub fn link(&self)-> Result<(), err::RoleErr>{
        let connection = sqlite::open("./data/cinemadb.db")?;
        let mut db = connection.prepare("INSERT INTO role ('Name') VALUES (?);")?;
        db.bind::<&[(_, &str)]>(&[
            (1, self.name.as_str()),
        ][..])?;
        db.next()?;
        Ok(())
    }
}

impl History{
    pub fn add(&self)-> Result<(), err::HistoryErr>{
        let connection = sqlite::open("./data/cinemadb.db")?;
        let mut db = connection.prepare("INSERT INTO history ('IdUser', 'IdContent') VALUES (?, ?);")?;
        db.bind::<&[(_, &str)]>(&[
            (1, self.id_user.to_string().as_str()),
            (2, self.id_content.to_string().as_str()),
        ][..])?;
        db.next()?;
        Ok(())
    }
}

impl File {
    pub fn add(&self)-> Result<(), err::FileErr>{
        let connection = sqlite::open("./data/cinemadb.db")?;
        let mut db = connection.prepare("INSERT INTO file ('IdContent', 'Path') VALUES (?, ?);")?;
        db.bind::<&[(_, &str)]>(&[
            (1, self.id_content.to_string().as_str()),
            (2, self.path.as_str()),
        ][..])?;
        db.next()?;
        Ok(())
    }

    pub fn return_path(id: usize) -> Result<String, err::FileErr>{
        let connection = sqlite::open("./data/cinemadb.db")?;
        let mut db = connection.prepare("SELECT * FROM file WHERE IdContent = ?;")?;
        db.bind::<&[(_, &str)]>(&[
            (1, id.to_string().as_str()),
        ][..])?;
        db.next()?;
        let result = match db.read::<String, _>(2){
            Ok(u) => u,
            Err(_) => "".to_string(), 
        };
        Ok(result)
    }
}

impl ContentForPreferences {
    pub fn add(&self)-> Result<(), err::ContentForPreferencesErr>{
        let connection = sqlite::open("./data/cinemadb.db")?;
        let mut db = connection.prepare("INSERT INTO content_for_preferences ('IdContent', 'IdUsers') VALUES (?, ?);")?;
        db.bind::<&[(_, &str)]>(&[
            (1, self.id_content.to_string().as_str()),
            (2, self.id_users.to_string().as_str()),
        ][..])?;
        db.next()?;
        Ok(())
    }
}

impl Content {
    pub fn add(&self)-> Result<(), err::ContentErr>{
        let connection = sqlite::open("./data/cinemadb.db")?;
        let mut db = connection.prepare("INSERT INTO content ('Name', 'Description', 'DescriptionDetails') VALUES (?, ?, ?);")?;
        db.bind::<&[(_, &str)]>(&[
            (1, self.name.as_str()),
            (2, self.description.as_str()),
            (3, self.description_details.as_str()),
        ][..])?;
        db.next()?;
        Ok(())
    }

    pub fn return_level_subscribe_id(id: usize) -> Result<usize, err::ContentErr>{
        let connection = sqlite::open("./data/cinemadb.db")?;
        let mut db = connection.prepare("SELECT * FROM content WHERE Id = ?;")?;
        db.bind::<&[(_, &str)]>(&[
            (1, id.to_string().as_str()),
        ][..])?;
        db.next()?;
        match db.read::<String, _>(5)?.parse::<usize>() {
            Ok(u) => Ok(u),
            Err(_) => return Err(err::ContentErr::DbErr(sqlite::Error { code: Some(12), message: Some("rep".to_string()) })),
        }
    }

    pub fn return_content_id(name:String) -> Result<usize, err::ContentErr>{
        let connection = sqlite::open("./data/cinemadb.db")?;
        let mut db = connection.prepare("SELECT * FROM content WHERE Name = ?;")?;
        db.bind::<&[(_, &str)]>(&[
            (1, name.as_str()),
        ][..])?;
        db.next()?;
        match db.read::<String, _>(0)?.parse::<usize>().ok().unwrap_or(0) {
            0 => return Err(err::ContentErr::DbErr(sqlite::Error { code: Some(12), message: Some("rep".to_string()) })),
            u => Ok(u) 
        }
    }

    pub fn check_video_path(id: usize) -> Result<String, err::ContentErr>{
        match File::return_path(id) {
            Ok(s) => Ok(s),
            Err(_) => Err(err::ContentErr::DbErr(sqlite::Error { code: Some(12), message: Some("rep".to_string()) })),
        }
    }
}

impl Codepromo {
    pub fn add(&self)-> Result<(), err::CodepromoErr>{
        let connection = sqlite::open("./data/cinemadb.db")?;
        let mut db = connection.prepare("INSERT INTO codepromo ('IdSubscribe', 'Description') VALUES (?, ?);")?;
        db.bind::<&[(_, &str)]>(&[
            (1, self.id_subscribe.to_string().as_str()),
            (2, self.description.as_str()),
        ][..])?;
        db.next()?;
        Ok(())
    }

    pub fn check_for_description(description: &str) -> Result<(usize, usize), err::CodepromoErr>{
        let connection = sqlite::open("./data/cinemadb.db")?;
        let mut db = connection.prepare("SELECT * FROM codepromo WHERE Description = ?;")?;
        db.bind::<&[(_, &str)]>(&[
            (1, description),
        ][..])?;
        db.next()?;

        return Ok((db.read::<i64, _>(2).unwrap() as usize, db.read::<i64, _>(3).unwrap() as usize));
    }
}