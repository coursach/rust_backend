
#[macro_use] extern crate rocket; 


use rocket::fs::NamedFile;

//use chrono::Duration;
//use rocket::tokio::time::{self, Duration};
use rocket::{http::{Cookie, CookieJar, Status}, request::FromRequest, response::stream::{Event, EventStream}, time::OffsetDateTime}; 
use serde_json::json;
use rocket_contrib::json::Json;

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

#[get("/subscribe")]
fn get_all_subscribe() ->(Status, String){
    match Subscribe::all() {
        Ok(v) =>(Status::Ok, Json(json!({"result": v})).to_string()),
        Err(e) => (Status::InternalServerError, Json(json!({"result": format!("{:?}", e)})).to_string()),
    }
}

#[post("/login", data="<login_string>", format ="json")]
fn login(login_string: String) ->(Status, String){
    match serde_json::from_str::<LoginRequest>(&login_string) {
        Ok(v)=>{
            match Users::login(v.email.clone(), v.password.clone()) {
                Ok(u) => {
                    match u.0 {
                        true => {
                            let claim = Claims::from_name(&format!("{}:{}:{}",u.1 , v.email, v.password));
                            let response = TransmittedToken{
                                token: match claim.into_token(){
                                    Ok(s) => s,
                                    Err(_) => todo!(),
                                }
                            };
                            (Status::Ok, Json(json!(response)).to_string())
                        },
                        false => (Status::Unauthorized, Json(json!({
                            "result": "нет"
                        })).to_string()),
                    }
                },
                Err(_) => (Status::Unauthorized, Json(json!({
                    "result": "null"
                })).to_string()),
            }
        }, 
        Err(e)=>(Status::Ok, Json(json!({
            "result": e.to_string()
        })).to_string()),
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


use ws::{Message, Stream, WebSocket};
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
use std::fs::File;
//use rocket::response::stream::{TextStream, ReaderStream};
//use rocket::Shutdown;
use std::io::Read;
//use rocket::futures::stream::{repeat, StreamExt};
/* 
#[get("/echo")]
fn echo_compose() -> TextStream![String]{
    TextStream!{
    }
}*/

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
}
#[get("/echo1")]
async fn echo_compose() -> Option<NamedFile>{
    NamedFile::open("data/video/mem.mp4").await.ok()
}
/* 
#[get("/stream/hi/<n>")]
fn one_hi_per_ms(mut shutdown: Shutdown, n: u8) -> TextStream![&'static str] {
    TextStream(repeat("hi").take(100))
}*/

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![get_all_subscribe, login, echo_stream, echo_compose])
    .mount("/user/update", routes![update_profile, update_image_profile_jpeg, update_image_profile_png])
    .mount("/user/link", routes![link_subscibe_to_user])
    .mount("/user/unlink", routes![unlink_subscibe_to_user])
    .mount("/user/get", routes![get_subscibe_to_profile])
    .mount("/registration", routes![registration_user])
    .mount("/admin/update", routes![update_subscibe, update_user])
    .mount("/admin/add", routes![add_subscibe, add_user])
}


