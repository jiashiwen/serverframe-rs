mod common_module;
mod request_module;
mod response_module;

pub use common_module::Key;
pub use common_module::KV;
pub use request_module::CreateUser;
pub use response_module::Response;
pub use response_module::User;

/// 定义自己的 Result
pub type Result<T> = std::result::Result<T, crate::httpserver::exception::AppError>;
