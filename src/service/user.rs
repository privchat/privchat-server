use std::sync::Arc;
use tokio::sync::Mutex;
use crate::ChatServer;

pub struct UserService {
    chat_server: Arc<Mutex<ChatServer>>,  // 引入 ChatServer 依赖
}

impl UserService {
    async fn get_user_info(&self, user_id: &str) {}
    async fn get_friend_list(&self, user_id: &str) {}
    async fn add_friend(&self, user_id: &str, friend_id: &str) {}
    async fn approve_friend_request(&self, user_id: &str, friend_id: &str) {}
    async fn delete_friend(&self, user_id: &str, friend_id: &str) {}
}