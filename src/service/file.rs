// use async_trait::async_trait;

// #[async_trait]
// pub trait FileService {
//     async fn upload_file(&self, user_id: &str, file_data: &[u8], file_type: &str) -> String;
//     async fn download_file(&self, file_id: &str) -> Option<Vec<u8>>;
//     async fn delete_file(&self, user_id: &str, file_id: &str);
//     async fn share_file(&self, from_user_id: &str, to_user_id: &str, file_id: &str);
// }