use firerust::FirebaseClient;
use serde_json::Value;
fn main() {
}

#[allow(dead_code)]
fn get_users(){
    let firebase = FirebaseClient::new("https://voting-509cd-default-rtdb.europe-west1.firebasedatabase.app/").unwrap();
    let users = firebase.reference("/User");
    println!("{:?}", users.get::<Value>());
}

#[allow(dead_code, unused)]
fn get_user(user:&str){
    
}

#[allow(dead_code)]
fn get_polls(){
    
}

#[allow(dead_code, unused)]
fn get_poll(id:&u32){
    
}

#[allow(dead_code, unused)]
fn vote_for_poll(id_poll:&u32, id_question:&u32){
    
}
