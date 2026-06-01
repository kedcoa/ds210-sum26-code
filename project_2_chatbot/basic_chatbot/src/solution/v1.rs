use kalosm::language::*;

#[allow(dead_code)]
pub struct ChatbotV1 {
    model: Llama,
}

impl ChatbotV1 {
    #[allow(dead_code)]
    pub fn new(model: Llama) -> ChatbotV1 {
        return ChatbotV1 { model: model };
    }

    #[allow(dead_code)]
    pub async fn chat_with_user(&mut self, message: String) -> String {
        //creates a new chat session and stores into chat_session
        let mut chat_session: Chat<Llama> = self.model
            .chat()
            .with_system_prompt("The assistant will act like a pirate");

        //passes user message to function add_message
        //stores the response by unwrapping it into a string and storing into response variable
        let response:String = chat_session.add_message(message).await.unwrap().to_string();
        return response;

        // You need to add your code here
        // You must find a way to add the given message to the chat_session!
        // consider https://docs.rs/kalosm/0.4.0/kalosm/language/struct.Chat.html#method.add_message
        // Hint: make sure you transform/extract the response message as a **String**.
        //return String::from("Hello, I am not a bot (yet)!");
    }
}