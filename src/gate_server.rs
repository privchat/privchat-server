use std::sync::Arc;
use tokio::sync::Mutex;
use crate::config::ServerConfig;
use crate::connection_manager::ConnectionManager;
use crate::handlers::{handle_message, handle_connect, handle_disconnect};
use msgtrans::server::MessageTransportServer;
use tracing::{error, info, Level};
use msgtrans::channel::{QuicServerChannel, TcpServerChannel, WebSocketServerChannel};

pub struct GateServer {
    server: MessageTransportServer,
    config: ServerConfig,
    connection_manager: Arc<Mutex<ConnectionManager>>,
}

impl GateServer {
    pub async fn new(config: ServerConfig) -> Arc<Mutex<Self>> {
        let server = MessageTransportServer::new();
        let config_clone = config.clone();
        if config.tcp.is_none() && config.websocket.is_none() && config.quic.is_none() {
            error!("At least one channel (quic/tcp/websocket) must be configured.");
            std::process::exit(1);
        }
    
        // 如果配置了 TCP，添加 TCP 通道
        if let Some(tcp_config) = config_clone.tcp {
            server
                .add_channel(TcpServerChannel::new(
                    &tcp_config.address.clone(),
                    tcp_config.port,
                )).await;
            info!(
                "TCP channel added at {}:{}",
                tcp_config.address, tcp_config.port
            );
        }
    
        // WebSocket 和 QUIC 配置保持不变
        if let Some(websocket_config) = config_clone.websocket {
            server
                .add_channel(WebSocketServerChannel::new(
                    &websocket_config.address.clone(),
                    websocket_config.port,
                    &websocket_config.path.clone(),
                )).await;
            info!(
                "WebSocket channel added at {}:{}{}",
                websocket_config.address, websocket_config.port, websocket_config.path
            );
        }
    
        if let Some(quic_config) = config_clone.quic {
            server
                .add_channel(QuicServerChannel::new(
                    &quic_config.address.clone(),
                    quic_config.port,
                    &quic_config.cert_path.clone(),
                    &quic_config.key_path.clone(),
                )).await;
            info!(
                "QUIC channel added at {}:{}",
                quic_config.address, quic_config.port
            );
        }

        Arc::new(Mutex::new(Self {
            server,
            config,
            connection_manager: Arc::new(Mutex::new(ConnectionManager::new())),
        }))
    }

    pub async fn start(gate_server: Arc<Mutex<Self>>) {
        let mut guard = gate_server.lock().await;
        let server = &mut guard.server;



        // 设置消息处理、连接和断开连接事件处理器
        server.set_message_handler({
            let gate_server = Arc::clone(&gate_server);
            move |context, packet| {
                let gate_server = Arc::clone(&gate_server);
                tokio::spawn(async move {
                    let gate_server = gate_server.lock().await;
                    handle_message(&*gate_server, context, packet).await;
                });
            }
        });

        server.set_connect_handler({
            let gate_server = Arc::clone(&gate_server);
            move |context| {
                let gate_server = Arc::clone(&gate_server);
                tokio::spawn(async move {
                    let gate_server = gate_server.lock().await;
                    handle_connect(&*gate_server, context).await;
                });
            }
        });

        server.set_disconnect_handler({
            let gate_server = Arc::clone(&gate_server);
            move |context| {
                let gate_server = Arc::clone(&gate_server);
                tokio::spawn(async move {
                    let gate_server = gate_server.lock().await;
                    handle_disconnect(&*gate_server, context).await;
                });
            }
        });

        drop(guard);  // 手动释放锁
        gate_server.lock().await.server.start().await;
    }

    pub fn connection_manager(&self) -> Arc<Mutex<ConnectionManager>> {
        Arc::clone(&self.connection_manager)
    }
}