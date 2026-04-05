pub mod entity;
pub mod input;
pub mod r#loop;
pub mod renderer;
pub mod window;
pub mod physics;

#[derive(Debug, thiserror::Error)]
pub enum EngineError {
    #[error("renderer init failed: {0}")]
    RendererInit(String),
    #[error("window creation failed: {0}")]
    Window(#[from] winit::error::OsError),
    #[error("event loop error: {0}")]
    EventLoop(#[from] winit::error::EventLoopError),
    #[error("asset not found: {path}")]
    AssetNotFound { path: String },
}
