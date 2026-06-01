use kalosm::language::*;
use fix::fixed_load_session;
use crate::solution::file_library;

pub struct ChatbotV4 {
    model: Llama, //only stores model to allow fast assigning of model type to new chat sessions 
    //does not need to store chat_sessions anymore since it can load from file 
}

impl ChatbotV4 {
    //initializes new chatbot with model 
    pub fn new(model: Llama) -> ChatbotV4 {
        return ChatbotV4 {
            model: model,
        };
    }

    
    pub async fn chat_with_user(&mut self, username: String, message: String) -> String {
        //creates filename according to each username > aka. a path for file 
        let filename = &format!("{}.txt", username);
        
        //load user's existing session from file_library code using fn load_chat_session_from_file  
        let loaded = file_library::load_chat_session_from_file(filename); //returns an Option > will unwrap later 

        //if there is no loaded session, create new chat session with prompt
        let mut chat_session: Chat<Llama> = if loaded.is_none() {
            self.model.chat().with_system_prompt("The assistant will act like a pirate")
        } else { //if there is loaded session, convert back to chat_session using fn fixed_load_session and unwrap it 
            fixed_load_session( self.model.chat().with_system_prompt("The assistant will act like a pirate"), loaded.unwrap() )
        };

        //send message and get a response
        let response = chat_session.add_message(message).await.unwrap().to_string();
        
        //save updated session back to file (not mut b/c just saving it)
        let updated_session = chat_session.session().unwrap();
        file_library::save_chat_session_to_file(filename, &updated_session);

        return response;
    }
    //allows returning users to see history, same as V3 but loads from file instead of HashMap
    pub fn get_history(&self, username: String) -> Vec<String> {
        let filename = &format!("{}.txt", username); //generates filename path for user
        
        match file_library::load_chat_session_from_file(&filename) { //match filename to file_library load session function
            None => {
                return Vec::new();
            },
            Some(session) => {
                return session
                .history()
                .iter()
                .filter(|message| message.role() != MessageType::SystemPrompt)
                .map(|message| message.content().to_string())
                .collect();
                //iterates through entire history, filters system prompts and extracts just collection of text 
            }

        }
    }
}