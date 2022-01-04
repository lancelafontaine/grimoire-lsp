pub type Result<T> = anyhow::Result<T>;

pub mod errors;
pub mod init;
pub mod logger;
pub mod lsp;
pub mod project_root;
pub mod server_capabilities;
