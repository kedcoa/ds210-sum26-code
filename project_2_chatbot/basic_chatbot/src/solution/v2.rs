use kalosm::language::*;

#[allow(dead_code)]
pub struct ChatbotV2 {
    chat_session: Chat<Llama> //creates one chat session for entire chatbot so it can store all the messages
}

impl ChatbotV2 {
    #[allow(dead_code)]
    //fn new creates a new instance already with a chat session that has system prompt, so it will need async 
    pub async fn new(model: Llama) -> ChatbotV2 {
        let chat_session = model.chat().with_system_prompt("The assistant will act like a pirate");
        return ChatbotV2 { chat_session };
    }

    //response stays the same
    #[allow(dead_code)]
    pub async fn chat_with_user(&mut self, message: String) -> String {
        let response:String = self.chat_session.add_message(message).await.unwrap().to_string();
        return response;
    }
}