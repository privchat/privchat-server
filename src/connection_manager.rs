use msgtrans::context::Context as MsgContext;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

pub struct ConnectionManager {
    // 管理所有连接
    pub connections: RwLock<HashMap<usize, Arc<MsgContext>>>,
    // 管理已经验证的连接，按 user_id 分组
    pub verified_connections: RwLock<HashMap<String, Vec<Arc<MsgContext>>>>,
}

impl ConnectionManager {
    pub fn new() -> Self {
        Self {
            connections: RwLock::new(HashMap::new()),
            verified_connections: RwLock::new(HashMap::new()),
        }
    }

    // 添加连接到所有连接中
    pub fn add_connection(&self, session_id: usize, context: Arc<MsgContext>) {
        self.connections.write().unwrap().insert(session_id, context);
    }

    // 将连接从未验证状态升级为已验证状态
    pub fn verify_connection(&self, user_id: String, session_id: usize) {
        let connections = self.connections.write().unwrap();
        if let Some(context) = connections.get(&session_id).cloned() {
            let mut verified = self.verified_connections.write().unwrap();
            verified.entry(user_id).or_default().push(context);
        }
    }

    // 从所有连接中移除连接
    pub fn remove_connection(&self, session_id: usize) {
        self.connections.write().unwrap().remove(&session_id);
    }

    // 根据 session_id 获取连接对象
    pub fn get_connection(&self, session_id: usize) -> Option<Arc<MsgContext>> {
        self.connections.read().unwrap().get(&session_id).cloned()
    }
}