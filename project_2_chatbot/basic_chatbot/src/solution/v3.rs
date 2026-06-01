use kalosm::language::*;
use std::{collections::HashMap};

#[allow(dead_code)]
pub struct ChatbotV3 {
    model : Llama, //stores the model
    chat_sessions: HashMap <String, Chat<Llama>> //takes in usernames as keys and each chat session as values 
    //maps username > their chat session 
}

impl ChatbotV3 {
    #[allow(dead_code)]
    pub fn new(model: Llama) -> ChatbotV3 {
        return ChatbotV3 {
            model,
            chat_sessions: HashMap::new() //creates a new empty hashmap to store chat sessions for each user
        };
    }

    #[allow(dead_code)]
    pub async fn chat_with_user(&mut self, username: String, message: String) -> String {
        // check if user already has a session
        if !self.chat_sessions.contains_key(&username) {
            let session = self.model.chat().with_system_prompt("The assistant will act like a pirate");
            self.chat_sessions.insert(username.clone(), session); 
            //this creates new chat session, clones it and inserts into chat_sessions hashmap 
        }
        
        // retrieves chat session and chatbot sends the message
        let chat_session = self.chat_sessions.get_mut(&username).unwrap();
        return chat_session.add_message(message).await.unwrap().to_string();
    }

    #[allow(dead_code)]
    pub fn get_history(&self, username: String) -> Vec<String> {
        // check if user has a session using if loop
        if let Some(chat_session) = self.chat_sessions.get(&username) {
           
            //get raw session data from chat 
            let raw_session = chat_session.session().unwrap();

            //get list of messages from session
            let messages = raw_session.history();

            //convert messages into string and return as vector
            let mut history_strings = Vec::new();
            for message in messages.iter() {
                let text = message.content().to_string();
                history_strings.push(text);
            }
            return history_strings;
        } 
        return Vec::new(); //if user has no past session, return empty history vec

    }
}