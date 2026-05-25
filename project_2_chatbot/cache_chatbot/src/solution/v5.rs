use std::num::NonZero;
use kalosm::language::*;
use file_chatbot::solution::file_library;
use fix::fixed_load_session;
use lru::LruCache;

pub struct ChatbotV5 {
    model: Llama,
    cache: LruCache<String, Chat<Llama>>,
}

impl ChatbotV5 {
    pub fn new(model: Llama) -> ChatbotV5 {
        return ChatbotV5 {
            model: model,
            cache: LruCache::new(NonZero::new(2).unwrap()),
        };
    }

    pub async fn chat_with_user(&mut self, username: String, message: String) -> String {
        let filename = &format!("{}.txt", username);
        let cached_chat = self.cache.get_mut(&username);

        match cached_chat {
            None => {
                println!("chat_with_user: {username} is not in the cache!");
                // The cache does not have the chat. What should you do?
                return String::from("Hello, I am not a bot (yet)!");
            }
            Some(chat_session) => {
                println!("chat_with_user: {username} is in the cache! Nice!");
                // The cache has this chat. What should you do?
                return String::from("Hello, I am not a bot (yet)!");

            }
        }
    }

    pub fn get_history(&mut self, username: String) -> Vec<String> {
        let filename = &format!("{}.txt", username);
        let cached_chat = self.cache.get_mut(&username);

        match cached_chat {
            None => {
                println!("get_history: {username} is not in the cache!");
                // TODO: The cache does not have the chat. What should you do?
                // Your code goes here.
                return Vec::new();
            }
            Some(chat_session) => {
                println!("get_history: {username} is in the cache! Nice!");
                // TODO: The cache has this chat. What should you do?
                // Your code goes here.
                return Vec::new();

            }
        }
    }
}
