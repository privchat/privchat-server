// use async_trait::async_trait;

// #[async_trait]
// pub trait ImageService {
//     async fn upload_image(&self, user_id: &str, image_data: &[u8]) -> String;
//     async fn get_image(&self, image_id: &str) -> Option<Vec<u8>>;
//     async fn delete_image(&self, user_id: &str, image_id: &str);
//     async fn share_image(&self, from_user_id: &str, to_user_id: &str, image_id: &str);
// }