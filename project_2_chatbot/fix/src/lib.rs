use kalosm::language::*;

/// Replays a saved session's history into a fresh chat by queuing each message
/// without triggering inference. Avoids the `Chat::with_session` bug where the
/// session history is fed to the model twice on the next turn.
pub fn fixed_load_session(mut chat: Chat<Llama>, session: LlamaChatSession) -> Chat<Llama> {
    for msg in session.history() {
        let _ = chat.add_message(msg);
    }
    return chat;
}
