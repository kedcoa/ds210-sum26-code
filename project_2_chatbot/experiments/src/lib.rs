use std::fs;
use std::time::{Duration, Instant};

pub fn cleanup_session_files() {
    fs::remove_file("Sophie.txt").ok();
    fs::remove_file("User1.txt").ok();
    fs::remove_file("User2.txt").ok();
}
use rocket::local::asynchronous::Client;
use rocket::serde::{Deserialize, Serialize};

// Create clients for different chatbot versions.
pub async fn client_v3() -> Client {
    let webserver = basic_chatbot::stencil::webserver::create_webserver().await;
    return Client::untracked(webserver).await.unwrap();
}

pub async fn client_v4() -> Client {
    let webserver = file_chatbot::stencil::webserver::create_webserver().await;
    return Client::untracked(webserver).await.unwrap();
}

pub async fn client_v5() -> Client {
    let webserver = cache_chatbot::stencil::webserver::create_webserver().await;
    return Client::untracked(webserver).await.unwrap();
}


// Types passed to or returned by ChatBots HTTP APIs.
#[derive(Debug, Serialize, Deserialize)]
struct ChatRequest<'a> {
    username: &'a str,
    message: &'a str,
}

#[derive(Debug, Serialize, Deserialize)]
struct ChatResponse {
    message: String,
}

type HistoryResponse = Vec<String>;

// Send chat message.
pub async fn send_chat_message(client: &Client, username: &str, message: &str) -> (String, Duration) {
    let request = client
        .post("/chat")
        .json(&ChatRequest {
            username,
            message
        });

    let time = Instant::now();
    let response = request.dispatch().await;
    let body = response.into_bytes().await;
    let time_taken = time.elapsed();


    let response: ChatResponse = serde_json::from_slice(&body.unwrap()).unwrap();
    return (response.message, time_taken);
}

// get history.
pub async fn get_history(client: &Client, username: &str) -> (Vec<String>, Duration) {
    let request = client
        .post("/history")
        .json(&ChatRequest {
            username,
            message: "",
        });

    let time = Instant::now();
    let response = request.dispatch().await;
    let body = response.into_bytes().await;
    let time_taken = time.elapsed();

    let response: HistoryResponse = serde_json::from_slice(&body.unwrap()).unwrap();
    return (response, time_taken);
}