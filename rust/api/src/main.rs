use firebase_rs::*;
use serde::{Deserialize, Serialize};
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Result};
use std::collections::HashMap;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new()
        .service(get_users)
        .service(get_user)
        .service(get_polls)
        .service(get_poll)
        .service(vote_for_poll)
    )
    .bind(("localhost", 8080))?
    .run()
    .await
}

#[allow(non_snake_case)]
#[derive(Clone, Debug, Deserialize, Serialize)]
struct User {
    Email: String,
    Password: String,
    Admin: bool,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize)]
struct Poll {
    Status: String,
    Questions: HashMap<String, i32>,
}


#[get("/users")]
async fn get_users()-> Result<impl Responder>{
    let firebase = Firebase::new("https://voting-509cd-default-rtdb.europe-west1.firebasedatabase.app/").unwrap();
    let users = firebase.at("/User");
    let parsed = users.get::<HashMap<String, User>>().await;
    Ok(web::Json(parsed.unwrap()))
}

#[get("/user/{username}")]
async fn get_user(user: web::Path<String>)-> Result<impl Responder>{
    let firebase = Firebase::new("https://voting-509cd-default-rtdb.europe-west1.firebasedatabase.app/").unwrap();
    let users = firebase.at("/User");
    let parsed = users.get::<HashMap<String, User>>().await.unwrap();
    let user_selected = &parsed[&user.to_string()];
    Ok(web::Json(user_selected.to_owned()))}

#[get("/polls")]
async fn get_polls()-> Result<impl Responder>{
    let firebase = Firebase::new("https://voting-509cd-default-rtdb.europe-west1.firebasedatabase.app/").unwrap();
    let polls = firebase.at("/Poll");
    let parsed = polls.get::<HashMap<String, Poll>>().await;
    Ok(web::Json(parsed.unwrap()))
}

#[get("/poll/{poll_id}")]
async fn get_poll(poll: web::Path<i32>)-> impl Responder{
    HttpResponse::Ok().json(format!("The poll {}", poll))
}

#[post("/vote/{poll_id}/{question_id}")]
async fn vote_for_poll(vote: web::Path<(u32, u32)>)-> impl Responder{
    HttpResponse::Ok().json(format!("Vote {} for {}!", vote.1, vote.0))
}
