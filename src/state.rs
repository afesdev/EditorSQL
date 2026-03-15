use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ConnectionState {
    Disconnected,
    Connecting,
    Connected,
    Error(String),
}

impl ConnectionState {
    pub fn label(&self) -> &str {
        match self {
            ConnectionState::Disconnected => "Desconectado",
            ConnectionState::Connecting => "Conectando…",
            ConnectionState::Connected => "Conectado",
            ConnectionState::Error(_) => "Error de conexión",
        }
    }

    pub fn badge_class(&self) -> &'static str {
        match self {
            ConnectionState::Disconnected => "badge-disconnected",
            ConnectionState::Connecting => "badge-connecting",
            ConnectionState::Connected => "badge-connected",
            ConnectionState::Error(_) => "badge-error",
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConnectionForm {
    pub name: String,
    pub server: String,
    pub port: String,
    pub database: String,
    pub user: String,
    pub password: String,
    pub integrated_security: bool,
    pub encrypt: bool,
    pub trust_server_certificate: bool,
}

impl Default for ConnectionForm {
    fn default() -> Self {
        Self {
            name: "Nueva Conexión".to_string(),
            server: "localhost".to_string(),
            port: "1433".to_string(),
            database: String::new(),
            user: String::new(),
            password: String::new(),
            integrated_security: true,
            encrypt: true,
            trust_server_certificate: true,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ChatRole {
    User,
    Assistant,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: ChatRole,
    pub content: String,
}

impl ChatMessage {
    pub fn new(role: ChatRole, content: String) -> Self {
        Self { role, content }
    }

    pub fn role_label(&self) -> &str {
        match self.role {
            ChatRole::User => "Tú",
            ChatRole::Assistant => "EditorSQL",
        }
    }
}
