// use async_trait::async_trait;

// #[async_trait]
// pub trait GroupService {
//     async fn create_group(&self, owner_user_id: &str, group_name: &str);
//     async fn add_member_to_group(&self, group_id: &str, user_id: &str);
//     async fn remove_member_from_group(&self, group_id: &str, user_id: &str);
//     async fn send_group_message(&self, group_id: &str, from_user_id: &str, message: &str);
//     async fn get_group_member_list(&self, group_id: &str);
// }