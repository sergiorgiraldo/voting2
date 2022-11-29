use firerust::FirebaseClient;
use serde_json::Value;
use actix_web::{get, post, web, Result, App, HttpResponse, HttpServer, Responder};

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
async fn get_users()-> impl Responder{
    let firebase = FirebaseClient::new("https://voting-509cd-default-rtdb.europe-west1.firebasedatabase.app/").unwrap();
    let users = firebase.reference("/User");
    HttpResponse::Ok().json(format!("{:?}", users.get::<Value>().unwrap()))
}

#[get("/user/{user_id}")]
async fn get_user(user: web::Path<i32>)-> impl Responder{
    HttpResponse::Ok().json(format!("The user {}", user))
}

#[get("/polls")]
async fn get_polls()-> impl Responder{
    let firebase = FirebaseClient::new("https://voting-509cd-default-rtdb.europe-west1.firebasedatabase.app/").unwrap();
    let polls = firebase.reference("/Poll");
    HttpResponse::Ok().json(format!("{:?}", polls.get::<Value>().unwrap()))
}

#[get("/poll/{poll_id}")]
async fn get_poll(poll: web::Path<i32>)-> impl Responder{
    HttpResponse::Ok().json(format!("The poll {}", poll))
}

#[post("/vote/{poll_id}/{question_id}")]
async fn vote_for_poll(vote: web::Path<(u32, u32)>)-> impl Responder{
    HttpResponse::Ok().json(format!("Vote {} for {}!", vote.1, vote.0))
}
