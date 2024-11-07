use std::sync::Arc;
use crate::server::ChatServer;
use msgtrans::context::Context;
use msgtrans::packet::Packet;
use tracing::{info, error};
use crate::service::chat::ChatService;

pub async fn handle_message(chat_server: &ChatServer, context: Arc<Context>, packet: Packet) {
    info!(
        "Received packet from session {}: message ID: {}, payload: {:?}",
        context.session().id(),
        packet.header.message_id,
        packet.payload,
    );

    // 先将 connection_manager 存储在一个局部变量中
    let conn_mgr = chat_server.connection_manager();
    let conn_mgr = conn_mgr.lock().await;
    // 进一步消息处理逻辑...
}

pub async fn handle_connect(chat_server: &ChatServer, context: Arc<Context>) {
    info!("New connection, session ID: {}", context.session().id());
    
    let conn_mgr = chat_server.connection_manager();
    let conn_mgr = conn_mgr.lock().await;
    conn_mgr.add_connection(context.session().id(), context);

    let chat_service = chat_server.container().get::<ChatService>().unwrap();
    chat_service.send_private_message(1, 2, "abc").await;
}

pub async fn handle_disconnect(chat_server: &ChatServer, context: Arc<Context>) {
    info!("Disconnected, session ID: {}", context.session().id());
    
    let conn_mgr = chat_server.connection_manager();
    let conn_mgr = conn_mgr.lock().await;
    conn_mgr.remove_connection(context.session().id());
}
