use self::state::AppState;
use crate::configure::AppConfig;
use crate::error::AppResult;
use crate::router::create_router_app;
pub mod state;
pub mod worker;
use tower_http::cors::{Any, CorsLayer};
use tracing::info;
pub struct AppServer {
    pub state: AppState,
    tcp: tokio::net::TcpListener,
}
impl AppServer {
    pub async fn new(mut config: AppConfig) -> AppResult<Self> {
        let tcp = tokio::net::TcpListener::bind(config.server.get_socket_addr()?).await?;
        let addr = tcp.local_addr()?;
        info!("The server is listening on: {addr}");
        config.server.port = addr.port();
        let state = AppState::new(config).await?;
        Ok(Self { state, tcp })
    }

    pub async fn run(self) -> AppResult<()> {
        // 配置 CorsLayer 允许所有跨域请求
        let cors = CorsLayer::new()
            .allow_origin(Any) // 允许所有来源
            .allow_methods(Any) // 允许所有方法
            .allow_headers(Any); // 允许所有头
        let router = create_router_app(self.state).layer(cors);
        axum::serve(self.tcp, router).await?;
        Ok(())
    }
}
