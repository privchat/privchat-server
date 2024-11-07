// use async_trait::async_trait;

// #[async_trait]
// pub trait HistoryService {
//     async fn get_user_chat_history(&self, user_id: &str, friend_id: &str, limit: usize, offset: usize);
//     async fn get_group_chat_history(&self, group_id: &str, limit: usize, offset: usize);
//     async fn save_user_chat_history(&self, user_id: &str, friend_id: &str, message: &str);
//     async fn save_group_chat_history(&self, group_id: &str, user_id: &str, message: &str);
// }