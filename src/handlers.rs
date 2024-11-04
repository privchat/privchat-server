use std::sync::Arc;
use crate::gate_server::GateServer;
use msgtrans::context::Context;
use msgtrans::packet::Packet;
use tracing::{info, error};

pub async fn handle_message(gate_server: &GateServer, context: Arc<Context>, packet: Packet) {
    info!(
        "Received packet from session {}: message ID: {}, payload: {:?}",
        context.session().id(),
        packet.header.message_id,
        packet.payload,
    );

    // 先将 connection_manager 存储在一个局部变量中
    let conn_mgr = gate_server.connection_manager();
    let conn_mgr = conn_mgr.lock().await;
    // 进一步消息处理逻辑...
}

pub async fn handle_connect(gate_server: &GateServer, context: Arc<Context>) {
    info!("New connection, session ID: {}", context.session().id());
    
    let conn_mgr = gate_server.connection_manager();
    let conn_mgr = conn_mgr.lock().await;
    conn_mgr.add_connection(context.session().id(), context);
}

pub async fn handle_disconnect(gate_server: &GateServer, context: Arc<Context>) {
    info!("Disconnected, session ID: {}", context.session().id());
    
    let conn_mgr = gate_server.connection_manager();
    let conn_mgr = conn_mgr.lock().await;
    conn_mgr.remove_connection(context.session().id());
}
