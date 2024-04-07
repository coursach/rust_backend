use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::users)]
#[allow(non_snake_case)]
pub struct Users{
    pub Id: i32,
    pub Name: String,
    pub Surname: String,
    pub Password: String,
    pub Email: String,
    pub Role: i32,
}