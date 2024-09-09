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
        .action(run_server);

    app.run(args);

    // 等待 Ctrl+C 退出
    tokio::signal::ctrl_c()
        .await
        .expect("Failed to listen for Ctrl+C");
    info!("Shutting down...");
}

fn run_server(c: &Context) {
    // 处理 `-c` 或 `--config` 参数，使用默认配置文件路径 "config.ini" 作为备选
    let config_path = c
        .string_flag("config")
        .unwrap_or("config/config.ini".to_string());

    if !Path::new(&config_path).exists() {
        error!("Config file not found: {}", config_path);
        std::process::exit(1);
    }

    // 加载配置文件
    let settings = load_config(&config_path).unwrap_or_else(|err| {
        error!("Error loading config: {:?}", err);
        std::process::exit(1); // 遇到严重错误时退出
    });

    // 检查至少有一个通道配置
    if settings.tcp.is_none() && settings.websocket.is_none() && settings.quic.is_none() {
        error!("At least one channel (quic/tcp/websocket) must be configured.");
        std::process::exit(1);
    }

    // 创建服务器实例
    let mut server = MessageTransportServer::new();

    // 如果配置了 TCP，添加 TCP 通道
    if let Some(tcp_config) = settings.tcp {
        server
            .add_channel(TcpServerChannel::new(
                &tcp_config.address.clone(),
                tcp_config.port,
            ));
        info!(
            "TCP channel added at {}:{}",
            tcp_config.address, tcp_config.port
        );
    }

    // 如果配置了 WebSocket，添加 WebSocket 通道
    if let Some(websocket_config) = settings.websocket {
        server
            .add_channel(WebSocketServerChannel::new(
                &websocket_config.address.clone(),
                websocket_config.port,
                &websocket_config.path.clone(),
            ));
        info!(
            "WebSocket channel added at {}:{}{}",
            websocket_config.address, websocket_config.port, websocket_config.path
        );
    }

    // 如果配置了 QUIC，添加 QUIC 通道
    if let Some(quic_config) = settings.quic {
        server
            .add_channel(QuicServerChannel::new(
                &quic_config.address.clone(),
                quic_config.port,
                &quic_config.cert_path.clone(),
                &quic_config.key_path.clone(),
            ));
        info!(
            "QUIC channel added at {}:{}",
            quic_config.address, quic_config.port
        );
    }

    // 设置消息处理器
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
        });

    // 设置连接处理器
    server.set_connect_handler(|context: Arc<MsgContext>| {
        info!("New connection, session ID: {}", context.session().id());
    });

    // 设置断开连接处理器
    server.set_disconnect_handler(|context: Arc<MsgContext>| {
        info!("Disconnected, session ID: {}", context.session().id());
    });

    // 设置错误处理器
    server.set_error_handler(|error| {
        error!("Error: {:?}", error);
    });

    // 启动服务器
    server.start();
    info!("Server is running...");
}

fn load_config(config_path: &str) -> Result<ServerConfig, config::ConfigError> {
    Config::builder()
        .add_source(File::with_name(config_path)) // 支持动态配置文件路径
        .build()?
        .try_deserialize::<ServerConfig>()
}
