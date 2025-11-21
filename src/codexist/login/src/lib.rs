mod device_code_auth;
mod pkce;
mod server;

pub use device_code_auth::run_device_code_login;
pub use server::LoginServer;
pub use server::ServerOptions;
pub use server::ShutdownHandle;
pub use server::run_login_server;

// Re-export commonly used auth types and helpers from codexist-core for compatibility
pub use codexist_app_server_protocol::AuthMode;
pub use codexist_core::AuthManager;
pub use codexist_core::CodexistAuth;
pub use codexist_core::auth::AuthDotJson;
pub use codexist_core::auth::CLIENT_ID;
pub use codexist_core::auth::CODEXIST_API_KEY_ENV_VAR;
pub use codexist_core::auth::OPENAI_API_KEY_ENV_VAR;
pub use codexist_core::auth::login_with_api_key;
pub use codexist_core::auth::logout;
pub use codexist_core::auth::save_auth;
pub use codexist_core::token_data::TokenData;
