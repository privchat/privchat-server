// use async_trait::async_trait;

// #[derive(Debug, Clone)]
// pub struct SharedLink {
//     pub url: String,
//     pub title: String,
//     pub icon: Option<String>,     // 链接图标（可选）
//     pub description: Option<String>, // 链接描述（可选）
// }

// #[async_trait]
// pub trait ShareService {
//     // 创建一个分享内容（例如文章、商品页面）
//     async fn create_shared_link(&self, user_id: &str, link: SharedLink) -> String;

//     // 获取用户的分享历史
//     async fn get_user_shared_links(&self, user_id: &str, limit: usize, offset: usize) -> Vec<SharedLink>;

//     // 删除某个分享链接
//     async fn delete_shared_link(&self, user_id: &str, link_id: &str);
// }