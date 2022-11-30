use firebase_rs::*;
use actix_web::{get, post, web, App, HttpServer, Responder, Result};
use std::collections::HashMap;
mod model;

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


#[get("/users")]
async fn get_users()-> Result<impl Responder>{
    let firebase = Firebase::new("https://voting-509cd-default-rtdb.europe-west1.firebasedatabase.app/").unwrap();
    let users = firebase.at("/User");
    let parsed = users.get::<HashMap<String,  model::User>>().await;
    
    Ok(web::Json(parsed.unwrap()))
}

#[get("/user/{username}")]
async fn get_user(user: web::Path<String>)-> Result<impl Responder>{
    let firebase = Firebase::new("https://voting-509cd-default-rtdb.europe-west1.firebasedatabase.app/").unwrap();
    let users = firebase.at("/User");
    let parsed = users.get::<HashMap<String, model::User>>().await.unwrap();
    let user_selected = &parsed[&user.to_string()];

    Ok(web::Json(user_selected.to_owned()))}

#[get("/polls")]
async fn get_polls()-> Result<impl Responder>{
    let firebase = Firebase::new("https://voting-509cd-default-rtdb.europe-west1.firebasedatabase.app/").unwrap();
    let polls = firebase.at("/Poll");
    let parsed = polls.get::<HashMap<String, model::Poll>>().await;

    Ok(web::Json(parsed.unwrap()))
}

#[get("/poll/{poll}")]
async fn get_poll(poll: web::Path<String>)-> Result<impl Responder>{
    let firebase = Firebase::new("https://voting-509cd-default-rtdb.europe-west1.firebasedatabase.app/").unwrap();
    let polls = firebase.at("/Poll");
    let parsed = polls.get::<HashMap<String, model::Poll>>().await.unwrap();
    let poll_selected = &parsed[&poll.to_string()];

    Ok(web::Json(poll_selected.to_owned()))
}

#[post("/vote/{poll}/{question}")]
async fn vote_for_poll(vote: web::Path<(String, String)>)-> Result<impl Responder>{
    let firebase = Firebase::new("https://voting-509cd-default-rtdb.europe-west1.firebasedatabase.app/").unwrap();
    let to_update = firebase.at("/Poll").at(vote.0.as_str());
    let mut parsed = to_update.get::<model::Poll>().await.unwrap();
    parsed.Questions.entry(vote.1.to_string()).and_modify(|k| *k += 1);
    let _res = to_update.update(&parsed).await;

    Ok("updated")
}
