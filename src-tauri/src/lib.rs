use serde::{Deserialize, Serialize};
use tiberius::{Client, Config, AuthMethod};
use tokio::net::TcpStream;
use tokio_util::compat::{TokioAsyncWriteCompatExt, Compat};
use std::sync::Arc;
use tokio::sync::Mutex;
use futures_util::stream::StreamExt;

/// Estado de la base de datos compartido en Tauri.
pub struct DbState {
    pub client: Arc<Mutex<Option<Client<Compat<TcpStream>>>>>,
}

#[derive(Debug, Serialize)]
pub struct QueryResult {
    pub columns: Vec<String>,
    pub rows: Vec<Vec<String>>,
}

#[derive(Debug, Serialize)]
pub struct GeneratedSqlAndAnswer {
    pub answer: String,
    pub sql: String,
}

#[derive(Debug, Deserialize, Clone)]
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

#[derive(Debug, Serialize)]
pub struct TestConnectionResult {
    pub success: bool,
    pub databases: Vec<String>,
    pub message: String,
}

/// Helper para crear la configuración de Tiberius
fn create_config(params: &ConnectParams) -> Config {
    let mut config = Config::new();
    config.host(&params.server);
    if let Ok(port) = params.port.parse::<u16>() {
        config.port(port);
    }
    
    if !params.database.trim().is_empty() {
        config.database(&params.database);
    }

    if params.integrated_security {
        // En esta configuración básica sin feature GSSAPI/WinAuth, 
        // desactivamos Integrated para que compile.
        // Se recomienda usar usuario/password por ahora.
        config.authentication(AuthMethod::sql_server("", ""));
    } else {
        config.authentication(AuthMethod::sql_server(&params.user, &params.password));
    }

    if params.encrypt {
        config.encryption(tiberius::EncryptionLevel::Required);
    } else {
        config.encryption(tiberius::EncryptionLevel::NotSupported);
    }

    if params.trust_server_certificate {
        config.trust_cert();
    }

    config
}

async fn get_client(params: &ConnectParams) -> Result<Client<Compat<TcpStream>>, String> {
    let config = create_config(params);
    let tcp = TcpStream::connect(config.get_addr()).await
        .map_err(|e| format!("Error de red (verifica IP/Puerto): {}", e))?;
    tcp.set_nodelay(true).map_err(|e| e.to_string())?;

    let client = Client::connect(config, tcp.compat_write()).await
        .map_err(|e| format!("Error de conexión SQL (verifica usuario/password): {}", e))?;
    
    Ok(client)
}

#[tauri::command]
async fn test_connection(params: ConnectParams) -> Result<TestConnectionResult, String> {
    if params.integrated_security {
        return Err("La autenticación integrada no está soportada en esta compilación. Por favor usa Usuario y Contraseña.".to_string());
    }

    let mut client = get_client(&params).await?;
    
    let mut stream = client.query("SELECT name FROM sys.databases WHERE database_id > 4", &[]).await
        .map_err(|e| format!("Error consultando bases de datos: {}", e))?;
    
    let mut databases = Vec::new();
    databases.push("master".to_string());

    while let Some(item) = stream.next().await {
        match item {
            Ok(tiberius::QueryItem::Row(row)) => {
                if let Some(name) = row.get::<&str, _>(0) {
                    databases.push(name.to_string());
                }
            }
            Err(e) => return Err(format!("Error en fila: {}", e)),
            _ => {}
        }
    }

    Ok(TestConnectionResult {
        success: true,
        databases,
        message: "Conexión exitosa.".to_string(),
    })
}

#[tauri::command]
async fn connect_to_sql_server(
    params: ConnectParams, 
    state: tauri::State<'_, DbState>
) -> Result<(), String> {
    let client = get_client(&params).await?;
    let mut client_lock = state.client.lock().await;
    *client_lock = Some(client);
    Ok(())
}

#[tauri::command]
async fn execute_sql(
    sql: String, 
    state: tauri::State<'_, DbState>
) -> Result<QueryResult, String> {
    let mut client_lock = state.client.lock().await;
    let client = client_lock.as_mut().ok_or("No hay una conexión activa.")?;

    let mut stream = client.query(sql, &[]).await
        .map_err(|e| format!("Error ejecutando consulta: {}", e))?;

    let mut columns = Vec::new();
    let mut rows = Vec::new();
    let mut first_row = true;

    while let Some(item) = stream.next().await {
        match item {
            Ok(tiberius::QueryItem::Row(row)) => {
                if first_row {
                    for col in row.columns() {
                        columns.push(col.name().to_string());
                    }
                    first_row = false;
                }
                
                let mut row_data = Vec::new();
                for i in 0..columns.len() {
                    // Soporte básico de tipos para el MVP
                    let val: String = match row.try_get::<&str, _>(i) {
                        Ok(Some(s)) => s.to_string(),
                        _ => match row.try_get::<i32, _>(i) {
                            Ok(Some(n)) => n.to_string(),
                            _ => match row.try_get::<f64, _>(i) {
                                Ok(Some(f)) => f.to_string(),
                                _ => "NULL".to_string(),
                            }
                        }
                    };
                    row_data.push(val);
                }
                rows.push(row_data);
            }
            Err(e) => return Err(format!("Error en resultado: {}", e)),
            _ => {}
        }
    }

    Ok(QueryResult { columns, rows })
}

#[tauri::command]
fn send_chat_message(prompt: String) -> Result<GeneratedSqlAndAnswer, String> {
    Ok(GeneratedSqlAndAnswer {
        answer: "Simulación: Analizando esquema real...".to_string(),
        sql: "SELECT TOP (10) * FROM sys.tables;".to_string(),
    })
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(DbState {
            client: Arc::new(Mutex::new(None)),
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            test_connection,
            connect_to_sql_server,
            send_chat_message,
            execute_sql
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
