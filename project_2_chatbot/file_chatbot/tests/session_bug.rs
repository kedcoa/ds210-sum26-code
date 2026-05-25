use file_chatbot::solution::v4::ChatbotV4;
use kalosm::language::{Llama, LlamaSource};

async fn build_model() -> Llama {
    Llama::builder()
        .with_source(LlamaSource::llama_3_2_1b_chat())
        .build()
        .await
        .unwrap()
}

/// Sends two messages across separate chatbot instances and checks that get_history
/// returns exactly the two exchanges in order.
///
/// This test reveals the kalosm Chat::with_session bug: the session history is fed
/// to the model twice on the second turn, so the saved history ends up with duplicated
/// entries and the assertion fails.
#[tokio::test(flavor = "multi_thread")]
async fn test_history_correct_after_two_turns() {
    let username = "bug_test_user".to_string();
    let session_file = format!("{}.txt", username);
    let _ = std::fs::remove_file(&session_file);

    let model = build_model().await;
    let message1 = "Say only the word HELLO and nothing else.".to_string();
    let message2 = "What word did you just say?".to_string();

    let mut chatbot = ChatbotV4::new(model.clone());
    let reply1 = chatbot.chat_with_user(username.clone(), message1.clone()).await;
    drop(chatbot);

    let mut chatbot2 = ChatbotV4::new(model.clone());
    let reply2 = chatbot2.chat_with_user(username.clone(), message2.clone()).await;
    let history = chatbot2.get_history(username.clone());

    let _ = std::fs::remove_file(&session_file);

    assert_eq!(history, vec![message1, reply1, message2, reply2]);
}
