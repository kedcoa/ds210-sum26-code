use std::num::NonZero;
use kalosm::language::*;
use file_chatbot::solution::file_library;
use fix::fixed_load_session;
use lru::LruCache;

pub struct ChatbotV5 {
    model: Llama,
    cache: LruCache<String, Chat<Llama>>, //assigns field cache with type LruCache<String, Chat<Llama>> to ChatbotV5 
}

impl ChatbotV5 {
    //intializes new chatbot with model and new cache w/ capacity of 2 
    pub fn new(model: Llama) -> ChatbotV5 {
        return ChatbotV5 {
            model: model,
            cache: LruCache::new(NonZero::new(2).unwrap()),
        };
    }

    
    pub async fn chat_with_user(&mut self, username: String, message: String) -> String {
        let filename = &format!("{}.txt", username);
        
        //retreive reference to user cache as exists or none 
        let cached_chat = self.cache.get_mut(&username); 

        //match if there is cache or not 
        match cached_chat {
            None => {
                println!("chat_with_user: {username} is not in the cache!");
                
                //load reference file session, returns some session or none 
                let loaded = file_library::load_chat_session_from_file(&filename);
                
                //if no loaded > initalize new chat session with prompt 
                let mut chat_session: Chat<Llama> = if loaded.is_none() {
                    self.model.chat().with_system_prompt("The assistant will act like a pirate")
                } else {
                    //there is loaded > convert back into chat session and unwrap  
                    fixed_load_session(
                        self.model.chat().with_system_prompt("The assistant will act like a pirate"),
                        loaded.unwrap()
                    )
                };

                //send message 
                let response = chat_session.add_message(message).await.unwrap().to_string();

                //save updated session back to file for permanent storage
                let updated_session = chat_session.session().unwrap();
                file_library::save_chat_session_to_file(&filename, &updated_session);
                drop(updated_session); // drop the session before putting into cache to stop borrowing 
                
                //insert into cache so retrieves it faster
                self.cache.put(username, chat_session);

                return response;
            }

            //if there is cache, just retrieve and update it
            Some(chat_session) => {
                println!("chat_with_user: {username} is in the cache! Nice!");

                //sends message and gets response
                let response = chat_session.add_message(message).await.unwrap().to_string();

                //saves to file for permanent storage and backup 
                let updated_session = chat_session.session().unwrap();
                file_library::save_chat_session_to_file(&filename, &updated_session);
                
                return response;
            }
        }
    }


    //loads history from cache if exists, or load from file if removed from cache 
    pub fn get_history(&mut self, username: String) -> Vec<String> {
        let filename = &format!("{}.txt", username);
        let cached_chat = self.cache.get_mut(&username);

        match cached_chat {
            None => {
                println!("get_history: {username} is not in the cache!");
                
                //not in cache, try loading from permanent file
                let loaded = file_library::load_chat_session_from_file(&filename);
                match loaded {
                    None => {
                        //this is a brand new user > have to create a new chat session and store into cache
                        let chat_session = self.model.chat().with_system_prompt("The assistant will act like a pirate");
                        self.cache.put(username, chat_session);
                        return Vec::new();
                    }
                    Some(session) => {
                        // this is returning user that doesn't have cache, but has loaded file
                        //convert back into chat sesion, filter and unwrap into vector 
                        let history = session.history().iter().filter(|msg| msg.role() != MessageType::SystemPrompt).map(|message| message.content().to_string()).collect();

                        // build a chat session and add to cache for future use
                        let chat_session = fixed_load_session(self.model.chat().with_system_prompt("The assistant will act like a pirate"), session);
                        self.cache.put(username, chat_session);

                        return history;
                    }
                }
                
            
            }
            //if there already is cache, just return history from there
            Some(chat_session) => {
                println!("get_history: {username} is in the cache! Nice!");
                
                //already in memory, so directly unwrap and return
                return chat_session.session().unwrap().history().iter().filter(|message|message.role() != MessageType::SystemPrompt).map(|message| message.content().to_string()).collect();
            }
        }
    }
}
