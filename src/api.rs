use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"], catch)]
    async fn invoke(cmd: &str, args: JsValue) -> Result<JsValue, JsValue>;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryResult {
    pub columns: Vec<String>,
    pub rows: Vec<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeneratedSqlAndAnswer {
    pub answer: String,
    pub sql: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConnectParams {
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

#[derive(Debug, Serialize, Deserialize)]
pub struct TestConnectionResult {
    pub success: bool,
    pub databases: Vec<String>,
    pub message: String,
}

pub async fn test_connection(params: ConnectParams) -> Result<TestConnectionResult, String> {
    #[derive(Serialize)]
    struct Args {
        params: ConnectParams,
    }
    let args = to_value(&Args { params }).map_err(|e| e.to_string())?;
    match invoke("test_connection", args).await {
        Ok(res) => from_value::<TestConnectionResult>(res).map_err(|e| e.to_string()),
        Err(e) => Err(from_value::<String>(e).unwrap_or_else(|_| "Error desconocido".to_string())),
    }
}

pub async fn connect_to_sql_server(params: ConnectParams) -> Result<(), String> {
    #[derive(Serialize)]
    struct Args {
        params: ConnectParams,
    }
    let args = to_value(&Args { params }).map_err(|e| e.to_string())?;
    match invoke("connect_to_sql_server", args).await {
        Ok(_) => Ok(()),
        Err(e) => Err(from_value::<String>(e).unwrap_or_else(|_| "Error de conexión".to_string())),
    }
}

pub async fn send_chat_message(prompt: String) -> Result<GeneratedSqlAndAnswer, String> {
    #[derive(Serialize)]
    struct Args {
        prompt: String,
    }
    let args = to_value(&Args { prompt }).map_err(|e| e.to_string())?;
    match invoke("send_chat_message", args).await {
        Ok(res) => from_value::<GeneratedSqlAndAnswer>(res).map_err(|e| e.to_string()),
        Err(e) => Err(from_value::<String>(e).unwrap_or_else(|_| "Error en el chat".to_string())),
    }
}

pub async fn execute_sql(sql: String) -> Result<QueryResult, String> {
    #[derive(Serialize)]
    struct Args {
        sql: String,
    }
    let args = to_value(&Args { sql }).map_err(|e| e.to_string())?;
    match invoke("execute_sql", args).await {
        Ok(res) => from_value::<QueryResult>(res).map_err(|e| e.to_string()),
        Err(e) => Err(from_value::<String>(e).unwrap_or_else(|_| "Error ejecutando SQL".to_string())),
    }
}
