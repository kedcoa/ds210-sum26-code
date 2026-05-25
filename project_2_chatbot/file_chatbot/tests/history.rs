use file_chatbot::solution::v4::ChatbotV4;
use kalosm::language::{Llama, LlamaSource};

async fn build_model() -> Llama {
    Llama::builder()
        .with_source(LlamaSource::llama_3_2_1b_chat())
        .build()
        .await
        .unwrap()
}

/// Sends one message, drops the chatbot (simulating a process restart), reconstructs it,
/// and checks that get_history returns exactly the one exchange that took place.
#[tokio::test(flavor = "multi_thread")]
async fn test_history_persists_across_restart() {
    let username = "history_test_user".to_string();
    let session_file = format!("{}.txt", username);
    let _ = std::fs::remove_file(&session_file);

    let model = build_model().await;
    let message = "Say only the word HELLO and nothing else.".to_string();

    let mut chatbot = ChatbotV4::new(model.clone());
    let reply = chatbot.chat_with_user(username.clone(), message.clone()).await;
    drop(chatbot);

    let chatbot2 = ChatbotV4::new(model.clone());
    let history = chatbot2.get_history(username.clone());

    let _ = std::fs::remove_file(&session_file);

    assert_eq!(history, vec![message, reply]);
}
