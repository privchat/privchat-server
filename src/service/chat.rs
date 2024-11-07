
use std::sync::Arc;
use crate::server::ChatServer;
use tracing::{info, error};

pub struct ChatService {
    chat_server: Arc<ChatServer>,  // 引入 ChatServer 依赖
}

impl ChatService {
    pub fn new(chat_server: Arc<ChatServer>) -> Self {
        Self { chat_server }
    }

    pub async fn send_private_message(&self, from_user_id: u64, to_user_id: u64, message: &str) {
        // 发送私信
        info!(
            "from_user_id: {} to_user_id: {}, message: {:?}",
            from_user_id,
            to_user_id,
            message,
        );
    }

    pub async fn send_group_message(&self, from_user_id: u64, group_id: u64, message: &str) {
        // 发送群组消息
    }

    pub async fn get_chat_history(&self, user_id: u64, other_user_id: u64, limit: usize, offset: usize) {
        // 获取聊天记录
    }

    pub async fn save_chat_message(&self, from_user_id: u64, to_user_id: u64, message: &str) {
        // 保存聊天消息
    }
}
