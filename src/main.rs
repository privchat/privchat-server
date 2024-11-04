mod config;
mod connection_manager;
mod gate_server;
mod handlers;

use crate::config::load_config;
use crate::gate_server::GateServer;
use tokio;
use tracing::{error, info, Level};
use seahorse::{App, Context, Flag, FlagType};

#[tokio::main]
async fn main() {
    info!("Starting server...");

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
    
    // 加载配置文件
    let config = load_config(&config_path).expect("Failed to load config");

    let gate_server = GateServer::new(config).await;

    // 启动服务器
    GateServer::start(gate_server.clone()).await;
}