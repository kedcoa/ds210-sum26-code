use kalosm::language::*;
use std::fs;

pub fn save_chat_session_to_file(filename: &str, session: &LlamaChatSession) {
    // converts session to raw bytes using to_bytes() function
    let bytes = session.to_bytes().unwrap();
    // saves bytes to a file using function fs::write from std::fs
    fs::write(filename, bytes).unwrap();
}


pub fn load_chat_session_from_file(filename: &str) -> Option<LlamaChatSession> {
    // if loop to check if file exists, and then read it 
    if fs::exists(filename).unwrap() {
        //read bytes from file using fs::read
        let bytes = fs::read(filename).unwrap();
        //convert bytes back into LlamaChatSession
        let session = LlamaChatSession::from_bytes(&bytes).unwrap();
        return Some(session); //ensures that can return session or None 
    } else {
        //if there is no file, return None
        return None;
    }
}