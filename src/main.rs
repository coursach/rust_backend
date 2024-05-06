
#[macro_use] extern crate rocket; 


use rocket::fs::NamedFile;
use rocket::http::{Header, Status};
use rocket::serde::json::Json;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::{Request, Response};

use models::*;
use transmitted_models::*;
use claims::Claims;

use user_function::*;
use admin_function::*;

mod function;
pub mod models;
mod transmitted_models;
mod claims;
mod user_function;
mod admin_function;

pub struct Cors;

#[rocket::async_trait]
impl Fairing for Cors {
    fn info(&self) -> Info {
        Info {
            name: "Cross-Origin-Resource-Sharing Fairing",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "http://localhost:8000"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, PATCH, PUT, DELETE, HEAD, OPTIONS, GET",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

#[get("/subscribe")]
fn get_all_subscribe() ->Result<Json<Vec<ReturnedSubscribes>>, Status>{
    match Subscribe::all() {
        Ok(v) => Ok(Json(v)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[post("/login", data="<login_data>")]
fn login(login_data: Json<LoginRequest>) ->Result<Json<TransmittedToken>, Status>{
    match Users::login(login_data.email.clone(), login_data.password.clone()) {
        Ok(u) => {
            match u.0 {
                true => {
                    let claim = Claims::from_name(&format!("{}:{}:{}",u.1 , login_data.email, login_data.password));
                    let token_string;
                    match claim.into_token(){
                        Ok(s) => token_string = s,
                        Err(_) => return Err(Status::Unauthorized),
                    };
                    let transmitted_token = TransmittedToken{
                        token: token_string,
                    };
                    Ok(Json(transmitted_token))
                },
                false => Err(Status::Unauthorized),
            }
        },
        Err(_) => Err(Status::Unauthorized),
    }
}
/*
#[get("/user_id")]
fn user_id(cookies: &CookieJar<'_>){
    let cookie = Cookie::new("id", "test");
    //let now = OffsetDateTime::now_utc();
    cookies.add_private(cookie);
    println!("{:?}",cookies.get_private("id"));
}
*/


//use ws::{Message, Stream, WebSocket};
/* 
#[get("/echo?channel")]
fn echo_channel(ws: ws::WebSocket) -> ws::Channel<'static> {
    use rocket::futures::{SinkExt, StreamExt};

    ws.channel(move |mut stream| Box::pin(async move {
        while let Some(message) = stream.next().await {
            let _ = stream.send(message?).await;
        }

        Ok(())
    }))
}

#[get("/echo?stream")]
fn echo_stream(ws: ws::WebSocket) -> ws::Stream!['static] {
    ws::Stream! { ws =>
        for await message in ws {
            yield message?;
        }
    }
}
*/
/* 
#[get("/echo")]
async fn echo_compose() -> Option<NamedFile>{
    NamedFile::open("data/video/mem.mp4").await.ok()
}
*/
//use std::net::SocketAddr;
//use std::fs::File;
//use rocket::response::stream::{TextStream, ReaderStream};
//use rocket::Shutdown;
//use std::io::Read;
//use rocket::futures::stream::{repeat, StreamExt};
/* 
#[get("/echo")]
fn echo_compose() -> TextStream![String]{
    TextStream!{
    }
}*/
/*
#[get("/echo")]
fn echo_stream(ws: ws::WebSocket) -> ws::Stream!['static] {
    ws::Stream! { ws =>
        for await message in ws {
            match message{
                Ok(_) => {
                    let mut file = File::open("data/image/eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJuYW1lIjoiMzplbWFpbDoxMjM0NSIsImV4cCI6MTcxMzUzNzI2OX0.aliUvUNy5Jb369up7BwIUl6GoZmVxQuHMd1sM3hdWws.jpeg").unwrap();
                    println!("dasdasdasd");
                    let mut buf = Vec::new();
                    let _ = file.read_to_end(&mut buf);
                    yield Message::Binary(buf);
                },
                Err(_) => yield Message::Text("sadasd".to_string()),   
            }
            yield message?;
        }
    }
}*/

#[options("/<_..>")]
fn everything() -> Status{
    Status::Ok
}

#[get("/images/<name>")]
async fn get_image(name: &str) -> Result<NamedFile, Status> {
    match NamedFile::open(format!("data/image/{}", name.to_string())).await.ok(){
        Some(v) => Ok(v),
        None => Err(Status::NotFound),
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
    .attach(Cors)
    .mount("/", routes![get_all_subscribe, login, get_image, everything, all_content_movie])
    .mount("/user/update", routes![update_profile, update_image_profile_jpeg, update_image_profile_png])
    .mount("/user/link", routes![link_subscibe_to_user])
    .mount("/user/unlink", routes![unlink_subscibe_to_user])
    .mount("/user/get", routes![get_subscibe_to_profile, get_user_profile, get_subscibe_to_promocode, get_content_from_token])
    .mount("/find", routes![find_content])
    .mount("/registration", routes![registration_user])
    .mount("/admin/update", routes![update_subscibe, update_user])
    .mount("/admin/add", routes![add_subscibe, add_user])
}


