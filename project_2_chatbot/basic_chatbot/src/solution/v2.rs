use kalosm::language::*;

#[allow(dead_code)]
pub struct ChatbotV2 {
    chat_session: Chat<Llama> //stores the chat session 
}

impl ChatbotV2 {
    #[allow(dead_code)]
    pub async fn new(model: Llama) -> ChatbotV2 {
        let chat_session = model.chat().with_system_prompt("The assistant will act like a pirate");
        return ChatbotV2 { chat_session };
    }

    #[allow(dead_code)]
    pub async fn chat_with_user(&mut self, message: String) -> String {
        let response:String = self.chat_session.add_message(message).await.unwrap().to_string();
        return response;
        //return String::from("Hello, I am not a bot (yet)!");
    }
}