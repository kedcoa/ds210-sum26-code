use std::time::Duration;

use experiments::*;
use rocket::local::asynchronous::Client;

async fn send_and_check(client: &Client) -> (String, Duration) {
    let (reply, _) = send_chat_message(client, "Sophie", "hello, I am Sophie!").await;
    let (messages, time) = get_history(client, "Sophie").await;
    assert_eq!(messages.len(), 2);
    assert_eq!(messages[0], "hello, I am Sophie!");
    assert_eq!(messages[1], reply);
    (reply, time)
}

async fn run_v3() -> Duration {
    send_and_check(&client_v3().await).await.1
}

async fn run_v4() -> Duration {
    send_and_check(&client_v4().await).await.1
}

async fn run_v5() -> (Duration, Duration) {
    let client = client_v5().await;
    let (reply, time_hot) = send_and_check(&client).await;

    // Evict Sophie from the cache by inserting two other users.
    let _ = send_chat_message(&client, "User1", "hello, I am user1!").await;
    let _ = send_chat_message(&client, "User2", "hello, I am user2!").await;

    // Retrieve history from v5 (cold).
    let (messages, time_cold) = get_history(&client, "Sophie").await;
    assert_eq!(messages.len(), 2);
    assert_eq!(messages[0], "hello, I am Sophie!");
    assert_eq!(messages[1], reply);

    (time_hot, time_cold)
}

#[tokio::main]
async fn main() {
    cleanup_session_files();
    let v3_time = run_v3().await;
    cleanup_session_files();
    let v4_time = run_v4().await;
    cleanup_session_files();
    let (v5_time_hot, v5_time_cold) = run_v5().await;
    cleanup_session_files();

    println!("----- Experiment Results -----");
    println!(" v3:       {:?}", v3_time);
    println!(" v4:       {:?}", v4_time);
    println!(" v5 (hot): {:?}", v5_time_hot);
    println!(" v5 (cold):{:?}", v5_time_cold);
    println!("----- End of Experiment Results -----");
}
