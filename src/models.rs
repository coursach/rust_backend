use std::usize;

use serde::{Deserialize, Serialize};
use sqlite::State;

use crate::{ReturnedSubscribes, ReturnedSubscribe, GetAdminUsers, ReturnedCodepromo, ReturnedFile};

pub mod err {
    use sqlite::Error as sqERR;
    #[derive(Debug)]
    pub enum UserErr {
        DbErr(sqERR),
    }
    #[derive(Debug)]
    pub enum WorkersErr {
        DbErr(sqERR),
    }
    #[derive(Debug)]
    pub enum SubscribeErr {
        DbErr(sqERR),
    }
    #[derive(Debug)]
    pub enum RoleErr {
        DbErr(sqERR),
    }
    #[derive(Debug)]
    pub enum SubscribeAndUserErr {
        DbErr(sqERR),
    }
    #[derive(Debug)]
    pub enum HistoryErr {
        DbErr(sqERR),
    }
    #[derive(Debug)]
    pub enum FileErr {
        DbErr(sqERR),
    }
    #[derive(Debug)]
    pub enum ContentForPreferencesErr {
        DbErr(sqERR),
    }
    #[derive(Debug)]
    pub enum ContentErr {
        DbErr(sqERR),
    }
    #[derive(Debug)]
    pub enum CodepromoErr {
        DbErr(sqERR),
    }
    #[derive(Debug)]
    pub enum WorkersForContentErr {
        DbErr(sqERR),
    }

    impl From<sqERR> for UserErr {
        fn from(s: sqERR) -> Self {
            UserErr::DbErr(s)
        }
    }
    impl From<sqERR> for WorkersErr {
        fn from(s: sqERR) -> Self {
            WorkersErr::DbErr(s)
        }
    }
    impl From<sqERR> for SubscribeErr {
        fn from(s: sqERR) -> Self {
            SubscribeErr::DbErr(s)
        }
    }
    impl From<sqERR> for SubscribeAndUserErr {
        fn from(s: sqERR) -> Self {
            SubscribeAndUserErr::DbErr(s)
        }
    }
    impl From<sqERR> for RoleErr {
        fn from(s: sqERR) -> Self {
            RoleErr::DbErr(s)
        }
    }
    impl From<sqERR> for CodepromoErr {
        fn from(s: sqERR) -> Self {
            CodepromoErr::DbErr(s)
        }
    }
    impl From<sqERR> for ContentErr {
        fn from(s: sqERR) -> Self {
            ContentErr::DbErr(s)
        }
    }
    impl From<sqERR> for ContentForPreferencesErr {
        fn from(s: sqERR) -> Self {
            ContentForPreferencesErr::DbErr(s)
        }
    }
    impl From<sqERR> for FileErr {
        fn from(s: sqERR) -> Self {
            FileErr::DbErr(s)
        }
    }
    impl From<sqERR> for HistoryErr {
        fn from(s: sqERR) -> Self {
            HistoryErr::DbErr(s)
        }
    }
    impl From<sqERR> for WorkersForContentErr {
        fn from(s: sqERR) -> Self {
            WorkersForContentErr::DbErr(s)
        }
    }
}
pub struct Codepromo {
    pub description: String,
    pub id_subscribe: usize,
    pub days: usize,
}

pub struct Content {
    pub id: usize,
    pub name: String,
    pub description: String,
    pub description_details: String,
    pub image_path: String,
    pub level: usize,
    pub id_mood: usize,
}

pub struct ContentForPreferences {
    pub id_content: i32,
    pub id_users: i32,
}

pub struct File {
    pub id_content: i32,
    pub path: String,
}

pub struct History {
    pub id_user: usize,
    pub id_content: usize,
}

pub struct Role {
    pub name: String,
}

#[derive(Deserialize, Serialize)]
pub struct Subscribe {
    pub name: String,
    pub count_month: usize,
    pub title: String,
    pub description: String,
    pub discount: usize,
    pub level: usize,
    pub price: usize,
}

pub struct SubscribeAndUser {
    pub id_subscribe: usize,
    pub id_users: usize,
    pub data_end: String,
}

pub struct Users {
    pub name: String,
    pub surname: String,
    pub password: String,
    pub email: String,
    pub image: String,
    pub role: i32,
}

pub struct Workers {
    pub name: String,
    pub surname: String,
    pub role: i32,
}

pub struct WorkersForContent {
    pub id: usize,
    pub id_content: usize,
    pub id_workers: usize,
}

impl Users {
    pub fn empty_user() -> Users {
        Users { name: "".to_string(), surname: "".to_string(), password: "".to_string(), email: "".to_string(), image: "".to_string(), role: 0 }
    }

    pub fn login(email: String, password: String) -> Result<(bool, usize), err::UserErr> {
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

    pub fn add(&self) -> Result<(), err::UserErr> {
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
                } else {
                    return Err(err::UserErr::DbErr(sqlite::Error { code: Some(12), message: Some("rep".to_string()) }));
                }
            }
            Err(e) => return Err(e),
        }
    }

    pub fn delete(id: usize) -> Result<(), err::UserErr> {
        let connection = sqlite::open("./data/cinemadb.db")?;
        let mut db = connection.prepare(format!("DELETE FROM users WHERE Id = {};", id))?;
        db.next()?;
        Ok(())
    }


    pub fn find_id(id: usize) -> Result<Users, err::UserErr> {
        let connection = sqlite::open("./data/cinemadb.db")?;
        let mut db = connection.prepare("SELECT * FROM users where Id = ?;")?;
        db.bind::<&[(_, &str)]>(&[
            (1, id.to_string().as_str()),
        ][..])?;
        db.next()?;
        Ok(Users {
            name: db.read(1)?,
            surname: db.read(2)?,
            password: db.read(3)?,
            email: db.read(4)?,
            role: db.read::<String, _>(5).unwrap().parse::<i32>().unwrap(),
            image: db.read(6)?,
        })
    }
    pub fn all() -> Result<Vec<GetAdminUsers>, err::UserErr> {
        let connection = sqlite::open("./data/cinemadb.db")?;
        let mut res: Vec<GetAdminUsers> = Vec::new();
        let mut db = connection.prepare("SELECT * FROM users;")?;
        while let State::Row = db.next()? {
            let ret = GetAdminUsers {
                id: db.read::<String, _>(0).unwrap().parse::<usize>().unwrap(),
                name: db.read(1)?,
                surname: db.read(2)?,
                password: db.read(3)?,
                email: db.read(4)?,
                role: db.read::<String, _>(5).unwrap().parse::<usize>().unwrap(),
                image: db.read(6)?,
            };
            res.push(ret);
        }
        Ok(res)
    }

    fn check_user_exsist(email: &str) -> Result<bool, err::UserErr> {
        let connection = sqlite::open("./data/cinemadb.db")?;
        let mut db = connection.prepare("SELECT Id FROM users where Email = ?;")?;
        db.bind::<&[(_, &str)]>(&[
            (1, email),
        ][..])?;
        db.next()?;
        let res = db.read::<String, _>(0).unwrap_or("0".to_string()).parse::<i32>().unwrap_or_default();
        if res != 0 {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub fn update_to_admin(&self, id: usize) -> Result<(), err::UserErr> {
        let connection = sqlite::open("./data/cinemadb.db")?;
        let mut db = connection.prepare("UPDATE users SET Name = ?, Surname = ?, Password = ?, Email = ?, Role = ?, ImageProfileFile = ? WHERE Id = ?;")?;
        //let mut db = connection.prepare("UPDATE users SET Name = ? WHERE Id = ?;")?;
        db.bind::<&[(_, &str)]>(&[
            (1, self.name.as_str()),
            (2, self.surname.to_string().as_str()),
            (3, self.password.to_string().as_str()),
            (4, self.email.to_string().as_str()),
            (5, self.role.to_string().as_str()),
            (6, self.image.to_string().as_str()),
            (7, id.to_string().as_str()),
        ][..])?;
        db.next()?;

        Ok({})
    }

    pub fn update(&self, id: usize) -> Result<(), err::UserErr> {
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
        return if changed {
            Ok(())
        } else {
            Err(err::UserErr::DbErr(sqlite::Error { code: Some(300), message: Some("".to_string()) }))
        };
    }
}

impl Workers {
    pub fn add(&self) -> Result<(), err::WorkersErr> {
        let connection = sqlite::open("./data/cinemadb.db")?;
        let mut db = connection.prepare("INSERT INTO workers ('Name', 'Surname', 'Role') VALUES (?, ?, ?);")?;
        db.bind::<&[(_, &str)]>(&[
            (1, self.name.as_str()),
            (2, self.surname.as_str()),
            (3, &self.role.to_string())
        ][..])?;
        db.next()?;
        Ok(())
    }
    pub fn get_worker_by_id(id_worker: usize) -> Result<Workers, err::WorkersErr> {
        let connection = sqlite::open("./data/cinemadb.db")?;
        let mut db = connection.prepare("SELECT * FROM workers where Id = ?;")?;
        db.bind::<&[(_, &str)]>(&[
            (1, id_worker.to_string().as_str()),
        ][..])?;
        db.next()?;
        Ok(Workers {
            name: db.read(1)?,
            surname: db.read(2)?,
            role: db.read::<String, _>(3).unwrap().parse::<i32>().unwrap(),
        })
    }
}

impl WorkersForContent {
    pub fn return_director_by_id_content(id_content: usize) -> Result<Vec<usize>, err::WorkersForContentErr> {
        let connection = sqlite::open("./data/cinemadb.db")?;
        let mut db = connection.prepare("SELECT IdWorkers FROM workers_for_content where IdContent = ?;")?;
        db.bind::<&[(_, &str)]>(&[
            (1, id_content.to_string().as_str()),
        ][..])?;
        let mut res: Vec<usize> = Vec::new();
        while let State::Row = db.next()? {
            res.push(db.read::<String, _>(0).unwrap().parse::<usize>().unwrap());
        }
        let mut result: Vec<usize> = Vec::new();
        loop {
            match res.pop() {
                Some(i) => {
                    match Workers::get_worker_by_id(i) {
                        Ok(w) => {
                            if w.role == 4 {
                                result.push(i);
                            }
                        }
                        Err(_) => break,
                    }
                }
                None => break
            }
        }
        Ok(result)
    }

    pub fn return_actor_by_id_content(id_content: usize) -> Result<Vec<usize>, err::WorkersForContentErr> {
        let connection = sqlite::open("./data/cinemadb.db")?;
        let mut db = connection.prepare("SELECT IdWorkers FROM workers_for_content where IdContent = ?;")?;
        db.bind::<&[(_, &str)]>(&[
            (1, id_content.to_string().as_str()),
        ][..])?;
        let mut res: Vec<usize> = Vec::new();
        while let State::Row = db.next()? {
            println!("fdffgf");
            res.push(db.read::<String, _>(0).unwrap().parse::<usize>().unwrap());
        }
        let mut result: Vec<usize> = Vec::new();
        loop {
            match res.pop() {
                Some(i) => {
                    match Workers::get_worker_by_id(i) {
                        Ok(w) => {
                            if w.role == 3 {
                                result.push(i);
                            }
                        }
                        Err(_) => break,
                    }
                }
                None => break
            }
        }
        Ok(result)
    }
}

impl Subscribe {
    pub fn add(&self) -> Result<(), err::SubscribeErr> {
        let connection = sqlite::open("./data/cinemadb.db")?;
        let mut db = connection.prepare("INSERT INTO subscribe ('Name', 'Count_month', 'Title', 'Description','Discount', 'Level', 'Price') VALUES (?, ?, ?, ?, ?, ?, ?);")?;
        db.bind::<&[(_, &str)]>(&[
            (1, self.name.as_str()),
            (2, &self.count_month.to_string()),
            (3, self.title.as_str()),
            (4, self.description.as_str()),
            (5, &self.discount.to_string()),
            (6, self.level.to_string().as_str()),
            (7, self.price.to_string().as_str())
        ][..])?;
        db.next()?;
        Ok(())
    }

    pub fn delete(id: usize) -> Result<(), err::SubscribeErr> {
        let connection = sqlite::open("./data/cinemadb.db")?;
        let mut db = connection.prepare("DELETE FROM subscribe WHERE Id = (?);")?;
        db.bind::<&[(_, &str)]>(&[
            (1, id.to_string().as_str()),
        ][..])?;
        db.next()?;
        Ok(())
    }

    pub fn update(&self, id: usize) -> Result<(), err::UserErr> {
        let connection = sqlite::open("./data/cinemadb.db")?;
        let mut db = connection.prepare("UPDATE subscribe SET Name = ?, Count_month = ?, Title = ?, Description = ?, Level = ?, Price = ? WHERE Id = ?;")?;
        //let mut db = connection.prepare("UPDATE users SET Name = ? WHERE Id = ?;")?;
        db.bind::<&[(_, &str)]>(&[
            (1, self.name.as_str()),
            (2, self.count_month.to_string().as_str()),
            (3, self.title.to_string().as_str()),
            (4, self.description.to_string().as_str()),
            (5, self.level.to_string().as_str()),
            (6, self.price.to_string().as_str()),
            (7, id.to_string().as_str()),
        ][..])?;
        db.next()?;

        Ok({})
    }

    pub fn all() -> Result<Vec<ReturnedSubscribes>, err::SubscribeErr> {
        let connection = sqlite::open("./data/cinemadb.db")?;
        let mut res: Vec<ReturnedSubscribes> = Vec::new();
        let mut db = connection.prepare("SELECT * FROM subscribe;")?;
        while let State::Row = db.next()? {
            let ret = ReturnedSubscribes {
                id: db.read::<String, _>(0).unwrap().parse::<usize>().unwrap(),
                name: db.read(1)?,
                count_month: db.read::<String, _>(2).unwrap().parse::<i32>().unwrap(),
                title: db.read(3)?,
                description: db.read(4)?,
                discount: db.read::<String, _>(5).unwrap().parse::<i32>().unwrap(),
                price: db.read::<String, _>(7).unwrap().parse::<usize>().unwrap(),
            };
            res.push(ret);
        }
        Ok(res)
    }

    pub fn count_month(id: usize) -> Result<usize, err::SubscribeErr> {
        let connection = sqlite::open("./data/cinemadb.db")?;
        let res: usize;
        let mut db = connection.prepare("SELECT Count_month FROM subscribe where Id = ?;")?;
        db.bind::<&[(_, &str)]>(&[
            (1, id.to_string().as_str()),
        ][..])?;
        db.next()?;
        res = db.read::<String, _>(0).unwrap().parse::<usize>().unwrap();
        if res == 0 {
            return Err(err::SubscribeErr::DbErr(sqlite::Error::from(sqlite::Error { code: Some(12), message: Some("rep".to_string()) })));
        }
        Ok(res)
    }
}

impl SubscribeAndUser {
    pub fn link(&self) -> Result<(), err::SubscribeAndUserErr> {
        let connection = sqlite::open("./data/cinemadb.db")?;
        match SubscribeAndUser::check_exist_link(self.id_users) {
            Ok(b) => {
                if b {
                    let mut db = connection.prepare("UPDATE subscribe_and_user SET IdSubscribe = ?, DataEnd = ? WHERE IdUsers = ?;")?;
                    db.bind::<&[(_, &str)]>(&[
                        (1, self.id_subscribe.to_string().as_str()),
                        (2, self.data_end.to_string().as_str()),
                        (3, self.id_users.to_string().as_str()),
                    ][..])?;
                    db.next()?;
                } else {
                    let mut db = connection.prepare("INSERT INTO subscribe_and_user ('IdSubscribe', 'IdUsers', 'DataEnd') VALUES (?, ?, ?);")?;
                    db.bind::<&[(_, &str)]>(&[
                        (1, self.id_subscribe.to_string().as_str()),
                        (2, self.id_users.to_string().as_str()),
                        (3, self.data_end.as_str()),
                    ][..])?;
                    db.next()?;
                }
                Ok(())
            }
            Err(_) => todo!(),
        }
    }

    pub fn check_exist_link(id_user: usize) -> Result<bool, err::SubscribeAndUserErr> {
        let connection = sqlite::open("./data/cinemadb.db")?;
        let mut db = connection.prepare("SELECT Id FROM subscribe_and_user WHERE IdUsers = ?;")?;
        db.bind::<&[(_, &str)]>(&[
            (1, id_user.to_string().as_str()),
        ][..])?;
        db.next()?;
        let result = match db.read::<i64, _>(0) {
            Ok(u) => u as usize,
            Err(_) => 0usize,
        };
        if result == 0 {
            Ok(false)
        } else {
            Ok(true)
        }
    }

    pub fn delete_link(id_user: usize) -> Result<(), err::SubscribeAndUserErr> {
        let connection = sqlite::open("./data/cinemadb.db")?;
        match SubscribeAndUser::check_exist_link(id_user) {
            Ok(b) => {
                if b {
                    let mut db = connection.prepare("DELETE FROM subscribe_and_user WHERE IdUsers = ?;")?;
                    db.bind::<&[(_, &str)]>(&[
                        (1, id_user.to_string().as_str()),
                    ][..])?;
                    db.next()?;
                    Ok(())
                } else {
                    Ok(())
                }
            }
            Err(e) => Err(e),
        }
    }

    pub fn get_user_link(id_user: usize) -> Result<ReturnedSubscribe, err::SubscribeAndUserErr> {
        let connection = sqlite::open("./data/cinemadb.db")?;
        let mut db = connection.prepare("SELECT * FROM subscribe_and_user WHERE IdUsers = ?;")?;
        db.bind::<&[(_, &str)]>(&[
            (1, id_user.to_string().as_str()),
        ][..])?;
        db.next()?;
        let mut db1 = connection.prepare("SELECT * FROM subscribe WHERE Id = ?;")?;
        let id: String = db.read(1)?;
        db1.bind::<&[(_, &str)]>(&[
            (1, id.as_str()),
        ][..])?;
        db1.next()?;
        return Ok(ReturnedSubscribe {
            id: db1.read::<String, _>(0).unwrap().parse::<usize>().unwrap(),
            name: db1.read(1)?,
            dead_line: db.read(3)?,
            title: db1.read(3)?,
            description: db1.read(4)?,
            discount: db1.read::<String, _>(5).unwrap().parse::<i32>().unwrap(),
            level: db1.read::<String, _>(6).unwrap().parse::<usize>().unwrap(),
            price: db1.read::<String, _>(7).unwrap().parse::<usize>().unwrap(),
        });
    }
}

impl Role {
    pub fn link(&self) -> Result<(), err::RoleErr> {
        let connection = sqlite::open("./data/cinemadb.db")?;
        let mut db = connection.prepare("INSERT INTO role ('Name') VALUES (?);")?;
        db.bind::<&[(_, &str)]>(&[
            (1, self.name.as_str()),
        ][..])?;
        db.next()?;
        Ok(())
    }
}

impl History {
    pub fn add(&self) -> Result<(), err::HistoryErr> {
        let connection = sqlite::open("./data/cinemadb.db")?;
        match Content::check_content_exist(self.id_content) {
            Ok(b) => {
                if b {
                    Ok(())
                } else {
                    let mut db = connection.prepare("INSERT INTO history ('IdUser', 'IdContent') VALUES (?, ?);")?;
                    db.bind::<&[(_, &str)]>(&[
                        (1, self.id_user.to_string().as_str()),
                        (2, self.id_content.to_string().as_str()),
                    ][..])?;
                    db.next()?;
                    Ok(())
                }
            }
            Err(_) => return Err(err::HistoryErr::DbErr(sqlite::Error { code: Some(12), message: Some("rep".to_string()) })),
        }
    }

    pub fn get_history_by_user(id: usize) -> Result<Vec<Content>, err::HistoryErr> {
        let connection = sqlite::open("./data/cinemadb.db")?;
        let mut db = connection.prepare(format!("SELECT IdContent FROM history where IdUser = {};", id))?;
        let mut content = Vec::new();
        while let State::Row = db.next()?
        {
            match Content::return_content_by_id(db.read::<String, _>(0).unwrap().parse::<usize>().unwrap()) {
                Ok(o) => match o {
                    Some(c) => content.push(c),
                    None => break,
                },
                Err(_) => break,
            }
        };
        Ok(content)
    }
}


impl File {

    pub fn add(&self) -> Result<(), err::FileErr> {
        let connection = sqlite::open("./data/cinemadb.db")?;
        let mut db = connection.prepare("INSERT INTO file ('IdContent', 'Path') VALUES (?, ?);")?;
        db.bind::<&[(_, &str)]>(&[
            (1, self.id_content.to_string().as_str()),
            (2, self.path.as_str()),
        ][..])?;
        db.next()?;
        Ok(())
    }
    pub fn all() -> Result<Vec<ReturnedFile>, err::UserErr> {
        let connection = sqlite::open("./data/cinemadb.db")?;
        let mut res: Vec<ReturnedFile> = Vec::new();
        let mut db = connection.prepare("SELECT * FROM file;")?;
        while let State::Row = db.next()? {
            let ret = ReturnedFile {
                id: db.read::<String, _>(0).unwrap().parse::<usize>().unwrap(),
                idContent: db.read::<String, _>(1).unwrap().parse::<usize>().unwrap(),
                path: db.read(2)?,
            };
            res.push(ret);
        }
        Ok(res)
    }
    pub fn delete(id: usize) -> Result<(), err::FileErr> {
        let connection = sqlite::open("./data/cinemadb.db")?;
        let mut db = connection.prepare("DELETE FROM file where Id = (?);")?;
        db.bind::<&[(_, &str)]>(&[
            (1, id.to_string().as_str()),
        ][..])?;
        db.next()?;
        Ok(())
    }

    pub fn return_path(id: usize) -> Result<String, err::FileErr> {
        let connection = sqlite::open("./data/cinemadb.db")?;
        let mut db = connection.prepare("SELECT * FROM file WHERE IdContent = ?;")?;
        db.bind::<&[(_, &str)]>(&[
            (1, id.to_string().as_str()),
        ][..])?;
        db.next()?;
        let result = db.read::<String, _>(2).unwrap_or_else(|_| "".to_string());
        Ok(result)
    }
}

impl ContentForPreferences {
    pub fn add(&self) -> Result<(), err::ContentForPreferencesErr> {
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
    pub fn add(&self) -> Result<(), err::ContentErr> {
        let connection = sqlite::open("./data/cinemadb.db")?;
        let mut db = connection.prepare("INSERT INTO content ('Name', 'Description', 'DescriptionDetails', 'ImagePath', 'LevelSubscribe', 'IdMood') VALUES (?, ?, ?, ?, ?, ?);")?;
        db.bind::<&[(_, &str)]>(&[
            (1, self.name.as_str()),
            (2, self.description.as_str()),
            (3, self.description_details.as_str()),
            (4, self.image_path.as_str()),
            (5, self.level.to_string().as_str()),
            (6, self.id_mood.to_string().as_str()),
        ][..])?;
        db.next()?;
        Ok(())
    }

    pub fn delete(id: usize) -> Result<(), err::ContentErr> {
        let connection = sqlite::open("./data/cinemadb.db")?;
        let mut db = connection.prepare("DELETE FROM content WHERE Id = (?);")?;
        db.bind::<&[(_, &str)]>(&[
            (1, id.to_string().as_str()),
        ][..])?;
        db.next()?;
        Ok(())
    }
    pub fn all() -> Result<Vec<Content>, err::ContentErr> {
        let connection = sqlite::open("./data/cinemadb.db")?;
        let mut db = connection.prepare("SELECT * FROM content;")?;
        let mut content = Vec::new();
        while let State::Row = db.next()?
        {
            content.push(Content {
                id: db.read::<String, _>(0).unwrap().parse::<usize>().unwrap(),
                name: db.read::<String, _>(1).unwrap(),
                description: db.read::<String, _>(2).unwrap(),
                description_details: db.read::<String, _>(3).unwrap(),
                image_path: db.read::<String, _>(4).unwrap(),
                level: db.read::<String, _>(5).unwrap().parse::<usize>().unwrap(),
                id_mood: db.read::<String, _>(6).unwrap().parse::<usize>().unwrap(),
            });
        }
        Ok(content)
    }

    pub fn return_level_subscribe_id(id: usize) -> Result<usize, err::ContentErr> {
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

    pub fn return_content_id(name: String) -> Result<usize, err::ContentErr> {
        let connection = sqlite::open("./data/cinemadb.db")?;
        let mut db = connection.prepare("SELECT * FROM content WHERE Name = ?;")?;
        db.bind::<&[(_, &str)]>(&[
            (1, name.as_str()),
        ][..])?;
        db.next()?;
        Ok(db.read::<String, _>(0)?.parse::<usize>().ok().unwrap_or(0))
    }
    pub fn return_contents_id(name: String) -> Result<Vec<usize>, err::ContentErr> {
        let connection = sqlite::open("./data/cinemadb.db")?;
        let mut db = connection.prepare(format!("SELECT * FROM content WHERE Name like '%{}%';", name))?;
        let mut result = Vec::new();
        while let State::Row = db.next()? {
            result.push(db.read::<String, _>(0)?.parse::<usize>().ok().unwrap_or(0));
        };
        Ok(result)
    }

    pub fn return_content_by_id(id: usize) -> Result<Option<Content>, err::ContentErr> {
        let connection = sqlite::open("./data/cinemadb.db")?;
        let mut db = connection.prepare("SELECT * FROM content WHERE Id = ?;")?;
        db.bind::<&[(_, &str)]>(&[
            (1, id.to_string().as_str()),
        ][..])?;
        db.next()?;
        let content = Content {
            id: db.read::<String, _>(0).unwrap().parse::<usize>().unwrap(),
            name: db.read::<String, _>(1).unwrap(),
            description: db.read::<String, _>(2).unwrap(),
            description_details: db.read::<String, _>(3).unwrap(),
            image_path: db.read::<String, _>(4).unwrap(),
            level: db.read::<String, _>(5).unwrap().parse::<usize>().unwrap(),
            id_mood: db.read::<String, _>(6).unwrap().parse::<usize>().unwrap(),
        };
        Ok(Some(content))
    }

    pub fn check_content_exist(id: usize) -> Result<bool, err::ContentErr> {
        let connection = sqlite::open("./data/cinemadb.db")?;
        let mut db = connection.prepare("SELECT * FROM history WHERE IdContent = ?;")?;
        db.bind::<&[(_, &str)]>(&[
            (1, id.to_string().as_str()),
        ][..])?;
        db.next()?;
        return match db.read::<String, _>(0).unwrap_or("0".to_string()).parse::<usize>().unwrap_or(0) {
            0 => Ok(false),
            _ => Ok(true),
        };
    }

    pub fn check_video_path(id: usize) -> Result<String, err::ContentErr> {
        match File::return_path(id) {
            Ok(s) => Ok(s),
            Err(_) => Err(err::ContentErr::DbErr(sqlite::Error { code: Some(12), message: Some("rep".to_string()) })),
        }
    }
}

impl Codepromo {
    pub fn add(&self) -> Result<(), err::CodepromoErr> {
        let connection = sqlite::open("./data/cinemadb.db")?;
        let mut db = connection.prepare("INSERT INTO codepromo ('IdSubscribe', 'Description', 'Days') VALUES (?, ?, ?);")?;
        db.bind::<&[(_, &str)]>(&[
            (1, self.id_subscribe.to_string().as_str()),
            (2, self.description.as_str()),
            (3, self.days.to_string().as_str())
        ][..])?;
        db.next()?;
        Ok(())
    }

    pub fn all() -> Result<Vec<ReturnedCodepromo>, err::UserErr> {
        let connection = sqlite::open("./data/cinemadb.db")?;
        let mut res: Vec<ReturnedCodepromo> = Vec::new();
        let mut db = connection.prepare("SELECT * FROM codepromo;")?;
        while let State::Row = db.next()? {
            let ret = ReturnedCodepromo {
                id: db.read::<String, _>(0).unwrap().parse::<usize>().unwrap(),
                description: db.read(1)?,
                idSubscribe: db.read::<String, _>(2).unwrap().parse::<usize>().unwrap(),
                days: db.read::<String, _>(3).unwrap().parse::<usize>().unwrap(),
            };
            res.push(ret);
        }
        Ok(res)
    }

    pub fn delete(id: &str) -> Result<(), err::CodepromoErr> {
        let connection = sqlite::open("./data/cinemadb.db")?;
        let mut db = connection.prepare("DELETE FROM codepromo WHERE Description = ?;")?;
        db.bind::<&[(_, &str)]>(&[
            (1, id.to_string().as_str()),
        ][..])?;
        db.next()?;
        Ok(())
    }

    pub fn check_for_description(description: &str) -> Result<(usize, usize), err::CodepromoErr> {
        let connection = sqlite::open("./data/cinemadb.db")?;
        let mut db = connection.prepare("SELECT * FROM codepromo WHERE Description = ?;")?;
        db.bind::<&[(_, &str)]>(&[
            (1, description),
        ][..])?;
        db.next()?;

        return Ok((db.read::<i64, _>(2).unwrap() as usize, db.read::<i64, _>(3).unwrap() as usize));
    }
}