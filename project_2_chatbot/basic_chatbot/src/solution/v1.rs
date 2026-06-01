use kalosm::language::*;

#[allow(dead_code)]
pub struct ChatbotV1 {
    model: Llama, //assigns Llama model to ChatbotV1 so that every instance after uses Llama model 
}

impl ChatbotV1 {
    #[allow(dead_code)]
    
    //creates a new instance and assigns it the llama model 
    pub fn new(model: Llama) -> ChatbotV1 {
        return ChatbotV1 { model: model };
    }

    #[allow(dead_code)]
    //takes in itself (has to be mutable b/c you're adding messages) and message, returns a string response
    pub async fn chat_with_user(&mut self, message: String) -> String {
        //creates var chat_session that stores everything inside
        let mut chat_session: Chat<Llama> = self.model 
            .chat() //self.model.chat() takes model of self and creates new chat session
            .with_system_prompt("The assistant will act like a pirate"); //responds according to this prompt
        
        //the blank chat_session that has system prompt now responds to user message with fn add_message and unwraps the Result into string
        let response:String = chat_session.add_message(message).await.unwrap().to_string();
        return response;
        //after each message, returns to top of this function and creates new chat session variable 
    }
}