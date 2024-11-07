use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};
use crate::config::ServerConfig;
use crate::manager::ConnectionManager;
use crate::handlers::{handle_message, handle_connect, handle_disconnect};
use msgtrans::server::MessageTransportServer;
use tracing::{error, info};
use msgtrans::channel::{QuicServerChannel, TcpServerChannel, WebSocketServerChannel};
use crate::service::chat::ChatService;
use busybody::*;

pub struct ChatServer {
    server: Arc<RwLock<MessageTransportServer>>, // 使用 RwLock 包装
    config: ServerConfig,
    connection_manager: Arc<Mutex<ConnectionManager>>,
    container: Arc<ServiceContainer>,
}

impl ChatServer {
    pub async fn new(config: ServerConfig) -> Arc<Self> {
        let server = Arc::new(RwLock::new(MessageTransportServer::new()));
        let config_clone = config.clone();
        if config.tcp.is_none() && config.websocket.is_none() && config.quic.is_none() {
            error!("At least one channel (quic/tcp/websocket) must be configured.");
            std::process::exit(1);
        }
    
        // 如果配置了 TCP，添加 TCP 通道
        // 获取可变写锁
        if let Some(tcp_config) = config_clone.tcp {
            let server_write = server.write().await; // 获取写锁
            server_write
                .add_channel(TcpServerChannel::new(
                    &tcp_config.address.clone(),
                    tcp_config.port,
                ))
                .await;
            info!(
                "TCP channel added at {}:{}",
                tcp_config.address, tcp_config.port
            );
        }

        // WebSocket 和 QUIC 配置类似，分别获取写锁后再调用 add_channel 方法
        if let Some(websocket_config) = config_clone.websocket {
            let server_write = server.write().await;
            server_write
                .add_channel(WebSocketServerChannel::new(
                    &websocket_config.address.clone(),
                    websocket_config.port,
                    &websocket_config.path.clone(),
                ))
                .await;
            info!(
                "WebSocket channel added at {}:{}{}",
                websocket_config.address, websocket_config.port, websocket_config.path
            );
        }

        if let Some(quic_config) = config_clone.quic {
            let server_write = server.write().await;
            server_write
                .add_channel(QuicServerChannel::new(
                    &quic_config.address.clone(),
                    quic_config.port,
                    &quic_config.cert_path.clone(),
                    &quic_config.key_path.clone(),
                ))
                .await;
            info!(
                "QUIC channel added at {}:{}",
                quic_config.address, quic_config.port
            );
        }

        let container = ServiceContainerBuilder::new().build();
        
        let chat_server = Arc::new(Self {
            server,
            config,
            connection_manager: Arc::new(Mutex::new(ConnectionManager::new())),
            container,
        });

        Self::register_containers(Arc::clone(&chat_server)).await;
        chat_server
    }

    async fn register_containers(chat_server: Arc<Self>) {
        chat_server.container.set(ChatService::new(Arc::clone(&chat_server)));
    }

    async fn register_event_handlers(chat_server: Arc<Self>) {
        let mut server = chat_server.server.write().await; // 获取可变写锁
    
        // 设置消息处理、连接和断开连接事件处理器
        server.set_message_handler({
            let chat_server = Arc::clone(&chat_server);
            move |context, packet| {
                let chat_server = Arc::clone(&chat_server);
                tokio::spawn(async move {
                    handle_message(&*chat_server, context, packet).await;
                });
            }
        }).await;
    
        server.set_connect_handler({
            let chat_server = Arc::clone(&chat_server);
            move |context| {
                let chat_server = Arc::clone(&chat_server);
                tokio::spawn(async move {
                    handle_connect(&*chat_server, context).await;
                });
            }
        });
    
        server.set_disconnect_handler({
            let chat_server = Arc::clone(&chat_server);
            move |context| {
                let chat_server = Arc::clone(&chat_server);
                tokio::spawn(async move {
                    handle_disconnect(&*chat_server, context).await;
                });
            }
        });
    }

    pub async fn start(chat_server: Arc<Self>) {
        Self::register_event_handlers(Arc::clone(&chat_server)).await;

        let server_write = chat_server.server.write().await; // 获取写锁
        server_write.start().await;
    }

    pub fn connection_manager(&self) -> Arc<Mutex<ConnectionManager>> {
        Arc::clone(&self.connection_manager)
    }

    pub fn container(&self) -> Arc<ServiceContainer> {
        self.container.clone()
    }
}