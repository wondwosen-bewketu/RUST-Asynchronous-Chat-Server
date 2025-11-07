#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_chat_state_management() {
        use crate::modules::chat::server::ChatState;
        use uuid::Uuid;

        let chat_state = ChatState::new();
        let user_id = Uuid::new_v4();
        let username = "test_user".to_string();
        let room_name = "test_room".to_string();

        // Add user to room
        assert!(chat_state.add_user_to_room(user_id, username.clone(), room_name.clone()).is_ok());

        // Check user was added
        let user = chat_state.get_user(user_id);
        assert!(user.is_some());
        let user = user.unwrap();
        assert_eq!(user.user_id, user_id);
        assert_eq!(user.username, username);
        assert_eq!(user.room_name, room_name);

        // Remove user
        let removed_user = chat_state.remove_user(user_id);
        assert!(removed_user.is_some());

        // Check user was removed
        let user = chat_state.get_user(user_id);
        assert!(user.is_none());
    }
}