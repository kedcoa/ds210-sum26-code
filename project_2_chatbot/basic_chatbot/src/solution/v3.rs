use kalosm::language::*;
use std::{collections::HashMap};

#[allow(dead_code)]
pub struct ChatbotV3 {
    model : Llama, //stores the model for convenience b/c need to create new chat sessions for every new user
    chat_sessions: HashMap <String, Chat<Llama>> //stores all chat sessions in HashMap to map username to their own chat session  
}

impl ChatbotV3 {
    #[allow(dead_code)]
    //initialize new chatbot with empty hashmap of chat session and model > ready for storage
    pub fn new(model: Llama) -> ChatbotV3 {
        return ChatbotV3 {
            model,
            chat_sessions: HashMap::new() 
        };
    }


    #[allow(dead_code)]
    pub async fn chat_with_user(&mut self, username: String, message: String) -> String {
        
        // if HashMap does not contain username > create a new session first 
        if !self.chat_sessions.contains_key(&username) {
            //temporary var session that creates new chat session with prompt
            let session = self.model.chat().with_system_prompt("The assistant will act like a pirate");
            //insert new session and username into HashMap
            self.chat_sessions.insert(username.clone(), session); //clone b/c need copy of username 
        }
        
        // maps username to chat session (old or new) and uses add_message
        let chat_session = self.chat_sessions.get_mut(&username).unwrap(); //get_mut b/c need to change chat session with new messages
        return chat_session.add_message(message).await.unwrap().to_string();
    }

    #[allow(dead_code)]
    //when user relogin, they can see their history
    pub fn get_history(&self, username: String) -> Vec<String> {
        // check if user has a session
        if let Some(chat_session) = self.chat_sessions.get(&username) { //there's some chat session for user 
            let raw_session = chat_session.session().unwrap(); //unwrap session messages into string vector
            // return chat history that loops through entire history, filters out system prompts, extracts just the text and collects them into vector of strings to return
            return raw_session.history().iter().filter(|message|message.role() != MessageType::SystemPrompt).map(|message| message.content().to_string()).collect();
            //filter ensures that when testing number of messages, only counts user and chatbot 
        }
        //if user has no session, create empty vector 
        return Vec::new(); 
    } 

}