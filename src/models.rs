use sqlite::State;

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
    pub end_see :i32,
}

pub struct Role {
    pub name : String,
}

pub struct Subscribe {
    pub name : String,
    pub count_month :i32,
    pub title : String,
    pub description : String,
    pub discount :i32,
}

pub struct SubscribeAndUser {
    pub id_subscribe :i32,
    pub id_users :i32,
    pub data_end : String,
}

pub struct Users{
    pub name: String,
    pub surname: String,
    pub password: String,
    pub email: String,
    pub role: i32,
}

pub struct Workers {
    pub name : String,
    pub surname : String,
    pub id_content :i32,
    pub role :i32,
}


impl Users {
    pub fn add(&self) -> Result<(), err::UserErr>{
        let connection = sqlite::open("./data/cinemadb.db")?;
        let mut db = connection.prepare("INSERT INTO users ('Name', 'Surname', 'Password', 'Email', 'Role') VALUES (?, ?, ?, ?, ?);")?;
        db.bind::<&[(_, &str)]>(&[
            (1, self.name.as_str()),
            (2, self.surname.as_str()),
            (3, self.password.as_str()),
            (4, self.email.as_str()),
            (5, &(self.role.to_string()))
        ][..])?;
        db.next()?;
        Ok(())
    }
    pub fn all(&self) -> Result<Vec<Users>, err::UserErr>{
        let connection = sqlite::open("./data/cinemadb.db")?;
        let mut res:Vec<Users> = Vec::new();
        let mut db = connection.prepare("SELECT * FROM users;")?;
        while let State::Row = db.next()? {
            let ret:Users = Users{
                name: db.read(1)?,
                surname: db.read(2)?,
                password: db.read(3)?,
                email: db.read(4)?,
                role: db.read::<String, _>(5).unwrap().parse::<i32>().unwrap(),
            } ;
            res.push(ret);
        }
        Ok(res)
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
}

impl SubscribeAndUser{
    pub fn link(&self)-> Result<(), err::SubscribeAndUserErr>{
        let connection = sqlite::open("./data/cinemadb.db")?;
        let mut db = connection.prepare("INSERT INTO subscribe_and_user ('IdSubscribe', 'IdUsers', 'DataEnd') VALUES (?, ?, ?);")?;
        db.bind::<&[(_, &str)]>(&[
            (1, self.id_subscribe.to_string().as_str()),
            (2, self.id_users.to_string().as_str()),
            (3, self.data_end.as_str()),
        ][..])?;
        db.next()?;
        Ok(())
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
        let mut db = connection.prepare("INSERT INTO history ('IdUser', 'IdContent', 'EndSee') VALUES (?, ?, ?);")?;
        db.bind::<&[(_, &str)]>(&[
            (1, self.id_user.to_string().as_str()),
            (2, self.id_content.to_string().as_str()),
            (3, self.end_see.to_string().as_str()),
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
}