use config::{Config, File};
use msgtrans::channel::{QuicServerChannel, TcpServerChannel, WebSocketServerChannel};
use msgtrans::context::Context as MsgContext;
use msgtrans::packet::Packet;
use msgtrans::server::MessageTransportServer;
use seahorse::{App, Context, Flag, FlagType};
use serde::Deserialize;
use std::path::Path;
use std::sync::Arc;
use tokio;
use tracing::{error, info, Level};
use tracing_subscriber;

#[derive(Debug, Deserialize, Default)]
struct ServerConfig {
    quic: Option<QuicConfig>,           // 可选
    tcp: Option<TcpConfig>,             // 可选
    websocket: Option<WebSocketConfig>, // 可选
}

#[derive(Debug, Deserialize)]
struct QuicConfig {
    address: String,
    port: u16,
    cert_path: String,
    key_path: String,
}

#[derive(Debug, Deserialize)]
struct TcpConfig {
    address: String,
    port: u16,
}

#[derive(Debug, Deserialize)]
struct WebSocketConfig {
    address: String,
    port: u16,
    path: String,
}

#[tokio::main]
async fn main() {
    // 初始化日志系统
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    let args = std::env::args().collect::<Vec<String>>();
    let app = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .description("PrivChat Server")
        .usage("privchat-server [-c <path>]") // 提供帮助信息
        .flag(
            Flag::new("config", FlagType::String)
                .description("Specify config file path")
                .alias("c"),
        )
        .action(|c: &Context| {
            // 因为 .action() 不支持异步，这里将 run_server 包装成一个 tokio::spawn 任务
            let config_path = c
                .string_flag("config")
                .unwrap_or("config/config.ini".to_string());
            tokio::spawn(async move {
                run_server(config_path).await;
            });
        });

    app.run(args);

    // 等待 Ctrl+C 退出
    tokio::signal::ctrl_c()
        .await
        .expect("Failed to listen for Ctrl+C");
    info!("Shutting down...");
}

// 将 run_server 改为异步函数
async fn run_server(config_path: String) {
    if !Path::new(&config_path).exists() {
        error!("Config file not found: {}", config_path);
        std::process::exit(1);
    }

    let settings = load_config(&config_path).unwrap_or_else(|err| {
        error!("Error loading config: {:?}", err);
        std::process::exit(1);
    });

    if settings.tcp.is_none() && settings.websocket.is_none() && settings.quic.is_none() {
        error!("At least one channel (quic/tcp/websocket) must be configured.");
        std::process::exit(1);
    }

    let mut server = MessageTransportServer::new();

    // 如果配置了 TCP，添加 TCP 通道
    if let Some(tcp_config) = settings.tcp {
        server
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

    // WebSocket 和 QUIC 配置保持不变
    if let Some(websocket_config) = settings.websocket {
        server
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

    if let Some(quic_config) = settings.quic {
        server
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

    // 设置处理器
    server
        .set_message_handler(|context: Arc<MsgContext>, packet: Packet| {
            info!(
                "Received packet from session {}: message ID: {}, payload: {:?}",
                context.session().id(),
                packet.header.message_id,
                packet.payload,
            );
            tokio::spawn({
                let session = Arc::clone(&context.session());
                async move {
                    if let Err(e) = session.send(packet).await {
                        error!("Failed to send packet: {:?}", e);
                    }
                }
            });
        })
        .await;

    server.set_connect_handler(|context: Arc<MsgContext>| {
        info!("New connection, session ID: {}", context.session().id());
    });

    server.set_disconnect_handler(|context: Arc<MsgContext>| {
        info!("Disconnected, session ID: {}", context.session().id());
    });

    server.set_error_handler(|error| {
        error!("Error: {:?}", error);
    });

    // 启动服务器
    server.start().await;
    info!("Server is running...");
}

fn load_config(config_path: &str) -> Result<ServerConfig, config::ConfigError> {
    Config::builder()
        .add_source(File::with_name(config_path)) // 支持动态配置文件路径
        .build()?
        .try_deserialize::<ServerConfig>()
}
