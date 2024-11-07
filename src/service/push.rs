// use async_trait::async_trait;

// #[derive(Debug, Clone)]
// pub struct PushNotification {
//     pub title: String,         // 通知标题
//     pub body: String,          // 通知内容
//     pub user_id: String,       // 目标用户ID
//     pub payload: Option<String>, // 可选的附加数据（JSON格式）
// }

// #[async_trait]
// pub trait PushNotificationService {
//     // 向单个用户发送通知
//     async fn send_notification(&self, notification: PushNotification) -> Result<(), String>;

//     // 向多个用户批量发送通知
//     async fn send_bulk_notifications(&self, notifications: Vec<PushNotification>) -> Result<(), String>;

//     // 注册用户的设备token
//     async fn register_device(&self, user_id: &str, device_token: &str) -> Result<(), String>;

//     // 注销用户设备token
//     async fn unregister_device(&self, user_id: &str, device_token: &str) -> Result<(), String>;
// }