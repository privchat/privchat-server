// use async_trait::async_trait;

// #[async_trait]
// pub trait StickerService {
//     // 获取默认的 emoji 列表
//     async fn get_default_emojis(&self) -> Vec<String>;

//     // 获取某个表情包（可能是收费的）中的表情列表
//     async fn get_sticker_pack(&self, pack_id: &str) -> Option<Vec<String>>;

//     // 用户添加/购买表情包
//     async fn add_sticker_pack_for_user(&self, user_id: &str, pack_id: &str);

//     // 获取用户所有表情包（包括默认表情包和已购买的表情包）
//     async fn get_user_sticker_packs(&self, user_id: &str) -> Vec<String>;

//     // 用户删除表情包
//     async fn remove_sticker_pack_for_user(&self, user_id: &str, pack_id: &str);

//     // 查询表情包是否收费
//     async fn is_sticker_pack_premium(&self, pack_id: &str) -> bool;
// }