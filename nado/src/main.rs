
//#[macro_use] extern crate rocket;              // like document

use diesel::prelude::*;
use rust_backend_cinema_service::establish_connection;
use rust_backend_cinema_service::schema;
use rust_backend_cinema_service::schema::users::dsl;
use serde_json::json;
fn main() {
    let mut conn = establish_connection();
    let my_document = match dsl::users   
            .filter(dsl::Id.eq(my_query.id))
            .select(Document::as_select())
            .first::<Document>(&mut conn)
        {
            Ok(record) => record,
            Err(err) => {
                error!(?err, "Error: ");

                return Response {
                    dataname: data,
                    data: "[]".to_string(),
                    error: format!("{}", err),
                };
            }
        };
}

/*#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}


*/