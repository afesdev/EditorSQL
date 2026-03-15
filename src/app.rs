#![allow(non_snake_case)]

use dioxus::prelude::*;

use crate::api;
use crate::state::{ChatMessage, ChatRole, ConnectionForm, ConnectionState};

static CSS: Asset = asset!("/assets/styles.css");

pub fn App() -> Element {
    let mut connection_form = use_signal(ConnectionForm::default);
    let mut connection_state = use_signal(|| ConnectionState::Disconnected);
    let mut server_databases = use_signal(Vec::<String>::new);
    
    let mut chat_messages = use_signal(Vec::<ChatMessage>::new);
    let mut chat_input = use_signal(String::new);
    let mut last_result = use_signal(|| Option::<api::QueryResult>::None);
    let mut last_generated_sql = use_signal(|| Option::<String>::None);
    let mut connection_modal_open = use_signal(|| false);
    let mut chat_panel_open = use_signal(|| true);
    let mut sql_editor_content = use_signal(String::new);
    let mut tree_expanded = use_signal(|| true);
    
    let mut test_message = use_signal(|| Option::<String>::None);
    let mut is_testing = use_signal(|| false);

    let on_open_modal = move |_| {
        test_message.set(None);
        connection_modal_open.set(true);
    };
    let on_close_modal = move |_| connection_modal_open.set(false);

    let on_test_connection = move |_| {
        let form = connection_form.read().clone();
        if form.server.trim().is_empty() {
            test_message.set(Some("Error: El servidor es obligatorio.".to_string()));
            return;
        }
        is_testing.set(true);
        test_message.set(None);
        
        let api_params = api::ConnectParams {
            name: form.name,
            server: form.server,
            port: form.port,
            database: form.database,
            user: form.user,
            password: form.password,
            integrated_security: form.integrated_security,
            encrypt: form.encrypt,
            trust_server_certificate: form.trust_server_certificate,
        };

        spawn(async move {
            match api::test_connection(api_params).await {
                Ok(res) => {
                    if res.success {
                        server_databases.set(res.databases);
                        test_message.set(Some(format!("✓ {}", res.message)));
                    } else {
                        test_message.set(Some(format!("✗ {}", res.message)));
                    }
                }
                Err(e) => {
                    test_message.set(Some(format!("Error: {}", e)));
                }
            }
            is_testing.set(false);
        });
    };

    let on_connect_click = move |_| {
        let form = connection_form.read().clone();
        if form.server.trim().is_empty() {
            connection_state.set(ConnectionState::Error("Servidor obligatorio.".to_string()));
            return;
        }
        connection_state.set(ConnectionState::Connecting);
        
        let api_params = api::ConnectParams {
            name: form.name,
            server: form.server,
            port: form.port,
            database: form.database,
            user: form.user,
            password: form.password,
            integrated_security: form.integrated_security,
            encrypt: form.encrypt,
            trust_server_certificate: form.trust_server_certificate,
        };

        spawn(async move {
            match api::connect_to_sql_server(api_params).await {
                Ok(()) => {
                    connection_state.set(ConnectionState::Connected);
                    connection_modal_open.set(false);
                }
                Err(e) => {
                    connection_state.set(ConnectionState::Error(e));
                }
            }
        });
    };

    let on_send_message = move |_| {
        let content = chat_input.read().trim().to_string();
        if content.is_empty() {
            return;
        }
        chat_messages.write().push(ChatMessage::new(ChatRole::User, content.clone()));
        chat_input.set(String::new());
        last_generated_sql.set(None);
        let prompt = content;
        spawn(async move {
            match api::send_chat_message(prompt).await {
                Ok(resp) => {
                    let text = format!("{}\n\nSQL generado:\n```sql\n{}\n```", resp.answer, resp.sql);
                    chat_messages.write().push(ChatMessage::new(ChatRole::Assistant, text));
                    last_generated_sql.set(Some(resp.sql.clone()));
                    sql_editor_content.set(resp.sql);
                }
                Err(e) => {
                    chat_messages.write().push(ChatMessage::new(ChatRole::Assistant, format!("Error: {}", e)));
                }
            }
        });
    };

    let on_run_sql = move |_| {
        let sql = sql_editor_content.read().trim().to_string();
        if sql.is_empty() {
            return;
        }
        spawn(async move {
            if let Ok(result) = api::execute_sql(sql).await {
                last_result.set(Some(result));
            }
        });
    };

    let on_toggle_chat = move |_| chat_panel_open.toggle();
    let on_toggle_tree = move |_| tree_expanded.toggle();

    rsx! {
        link { rel: "stylesheet", href: CSS }
        main { class: "app-root",
            header { class: "app-header",
                h1 { class: "app-title", "EditorSQL" }
                div { class: "header-actions",
                    span { class: "connection-badge {connection_state.read().badge_class()}", "{connection_state.read().label()}" }
                }
            }

            div { class: "workspace-row",
                div { class: "left-icon-bar",
                    button {
                        class: "icon-btn active",
                        title: "Bases de datos",
                        onclick: move |_| {},
                        span { class: "icon-db", "◉" }
                    }
                }

                div { class: "left-tree-panel",
                    div { class: "tree-panel-header",
                        span { class: "tree-panel-title", "Database" }
                        button { class: "icon-btn small", title: "Nueva conexión", onclick: on_open_modal, "+" }
                        button { class: "icon-btn small", title: "Actualizar", "↻" }
                        input { class: "tree-search", r#type: "text", placeholder: "Buscar..." }
                    }
                    div { class: "tree-content",
                        div {
                            class: "tree-node root",
                            onclick: on_toggle_tree,
                            span { class: "tree-chevron",
                                if *tree_expanded.read() { "▼" } else { "▶" }
                            }
                            span { "Conexiones" }
                        }
                        if *tree_expanded.read() {
                            div { class: "tree-children",
                                match &*connection_state.read() {
                                    ConnectionState::Connected => rsx! {
                                        div { class: "tree-node expanded",
                                            span { class: "tree-chevron", "▼" }
                                            span { "{connection_form.read().name}" }
                                        }
                                        div { class: "tree-children",
                                            for db in server_databases.read().iter() {
                                                div { class: "tree-node leaf",
                                                    span { class: "tree-chevron empty" }
                                                    span { "{db}" }
                                                }
                                            }
                                        }
                                    },
                                    _ => rsx! {
                                        div { class: "tree-node leaf muted",
                                            span { class: "tree-chevron empty" }
                                            span { "Sin conexión" }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                div { class: "center-workspace",
                    div { class: "center-tabs",
                        span { class: "tab active", "Consulta 1" }
                        button { class: "icon-btn small", "+" }
                    }
                    div { class: "sql-editor-wrap",
                        textarea {
                            class: "sql-editor",
                            placeholder: "Escribe tu consulta SQL aquí...",
                            value: "{sql_editor_content}",
                            oninput: move |ev| sql_editor_content.set(ev.value())
                        }
                    }
                    div { class: "center-toolbar",
                        button { class: "primary-button", onclick: on_run_sql, "▶ Ejecutar" }
                        {
                            let sql_opt = last_generated_sql.read();
                            if let Some(sql) = sql_opt.as_ref() {
                                let sql = sql.clone();
                                rsx! {
                                    button {
                                        class: "secondary-button",
                                        onclick: move |_| sql_editor_content.set(sql.clone()),
                                        "Usar SQL del chat"
                                    }
                                }
                            } else {
                                rsx! {}
                            }
                        }
                    }
                    div { class: "results-wrap",
                        div { class: "results-header", "Resultados" }
                        match last_result.read().as_ref() {
                            Some(result) => render_result_table(result),
                            None => render_results_placeholder()
                        }
                    }
                    div { class: "status-bar",
                        if let Some(r) = last_result.read().as_ref() {
                            span { "✓ " }
                            span { "{r.rows.len()} filas" }
                        } else {
                            span { "Listo." }
                        }
                    }
                }

                div { class: "right-area",
                    div { class: "right-icon-bar",
                        button {
                            class: if *chat_panel_open.read() { "icon-btn active" } else { "icon-btn" },
                            title: "Chat",
                            onclick: on_toggle_chat,
                            span { class: "icon-chat", "💬" }
                        }
                    }
                    if *chat_panel_open.read() {
                        div { class: "chat-panel",
                            div { class: "chat-panel-header",
                                span { "Chat" }
                                button { class: "icon-btn small", onclick: on_toggle_chat, "✕" }
                            }
                            div { class: "chat-messages",
                                for msg in chat_messages.read().iter() {
                                    div { class: "{chat_message_class(&msg.role)}",
                                        span { class: "message-role", "{msg.role_label()}" }
                                        p { "{msg.content}" }
                                    }
                                }
                            }
                            div { class: "chat-input-row",
                                textarea {
                                    class: "chat-input",
                                    rows: 2,
                                    placeholder: "Pregunta sobre tus datos...",
                                    value: "{chat_input}",
                                    oninput: move |ev| chat_input.set(ev.value())
                                }
                                button { class: "primary-button", onclick: on_send_message, "Enviar" }
                            }
                        }
                    }
                }
            }
        }

        if *connection_modal_open.read() {
            div { class: "modal-overlay", onclick: on_close_modal,
                div { class: "modal-content large", onclick: move |ev| ev.stop_propagation(),
                    div { class: "modal-header",
                        h2 { "Nueva conexión" }
                        button { class: "icon-btn", onclick: on_close_modal, "✕" }
                    }
                    div { class: "modal-body scrollable",
                        div { class: "form-row",
                            div { class: "form-group flex-2",
                                label { "Nombre de la conexión" }
                                input {
                                    r#type: "text",
                                    value: "{connection_form.read().name}",
                                    oninput: move |ev| connection_form.write().name = ev.value()
                                }
                            }
                        }
                        div { class: "form-row",
                            div { class: "form-group flex-2",
                                label { "Servidor" }
                                input {
                                    r#type: "text",
                                    value: "{connection_form.read().server}",
                                    placeholder: "localhost, 127.0.0.1",
                                    oninput: move |ev| connection_form.write().server = ev.value()
                                }
                            }
                            div { class: "form-group flex-1",
                                label { "Puerto" }
                                input {
                                    r#type: "text",
                                    value: "{connection_form.read().port}",
                                    oninput: move |ev| connection_form.write().port = ev.value()
                                }
                            }
                        }
                        
                        div { class: "form-group checkbox-group",
                            input {
                                r#type: "checkbox",
                                id: "integrated_security",
                                checked: connection_form.read().integrated_security,
                                onchange: move |_| {
                                    let mut form = connection_form.write();
                                    form.integrated_security = !form.integrated_security;
                                }
                            }
                            label { r#for: "integrated_security", "Seguridad Integrada (Windows Auth)" }
                        }

                        if !connection_form.read().integrated_security {
                            div { class: "form-row animate-fade",
                                div { class: "form-group",
                                    label { "Usuario" }
                                    input {
                                        r#type: "text",
                                        value: "{connection_form.read().user}",
                                        oninput: move |ev| connection_form.write().user = ev.value()
                                    }
                                }
                                div { class: "form-group",
                                    label { "Contraseña" }
                                    input {
                                        r#type: "password",
                                        value: "{connection_form.read().password}",
                                        oninput: move |ev| connection_form.write().password = ev.value()
                                    }
                                }
                            }
                        }

                        div { class: "form-row",
                            div { class: "form-group",
                                label { "Base de datos (Opcional)" }
                                select {
                                    class: "select-input",
                                    value: "{connection_form.read().database}",
                                    onchange: move |ev| connection_form.write().database = ev.value(),
                                    option { value: "", "Seleccionar base de datos..." }
                                    for db in server_databases.read().iter() {
                                        option { value: "{db}", "{db}" }
                                    }
                                }
                                // También permitimos escribirla manualmente si no se ha testeado
                                input {
                                    class: "mt-1",
                                    r#type: "text",
                                    placeholder: "O escribe el nombre directamente...",
                                    value: "{connection_form.read().database}",
                                    oninput: move |ev| connection_form.write().database = ev.value()
                                }
                            }
                        }

                        div { class: "form-row options-row",
                            div { class: "form-group checkbox-group",
                                input {
                                    r#type: "checkbox",
                                    id: "encrypt",
                                    checked: connection_form.read().encrypt,
                                    onchange: move |_| {
                                    let mut form = connection_form.write();
                                    form.encrypt = !form.encrypt;
                                }
                                }
                                label { r#for: "encrypt", "Cifrar conexión" }
                            }
                            div { class: "form-group checkbox-group",
                                input {
                                    r#type: "checkbox",
                                    id: "trust_server",
                                    checked: connection_form.read().trust_server_certificate,
                                    onchange: move |_| {
                                    let mut form = connection_form.write();
                                    form.trust_server_certificate = !form.trust_server_certificate;
                                }
                                }
                                label { r#for: "trust_server", "Confiar en certificado de servidor" }
                            }
                        }

                        if let Some(msg) = test_message.read().as_ref() {
                            p { class: if msg.starts_with("✓") { "test-success" } else { "test-error" }, "{msg}" }
                        }
                        
                        if let ConnectionState::Error(msg) = &*connection_state.read() {
                            p { class: "error-text", "{msg}" }
                        }
                    }
                    div { class: "modal-footer justify-between",
                        button { 
                            class: "secondary-button", 
                            onclick: on_test_connection,
                            disabled: *is_testing.read(),
                            if *is_testing.read() { "Probando..." } else { "Probar Conexión" }
                        }
                        div { class: "footer-actions",
                            button { class: "secondary-button", onclick: on_close_modal, "Cancelar" }
                            button { class: "primary-button", onclick: on_connect_click, "Conectar" }
                        }
                    }
                }
            }
        }
    }
}

fn render_results_placeholder() -> Element {
    rsx! {
        p { class: "placeholder-text", "Aquí verás los resultados de tus consultas." }
    }
}

fn render_connection_error(msg: &str) -> Element {
    rsx! {
        p { class: "error-text", "{msg}" }
    }
}

fn chat_message_class(role: &ChatRole) -> &'static str {
    match role {
        ChatRole::User => "message user",
        ChatRole::Assistant => "message assistant",
    }
}

fn render_result_table(result: &api::QueryResult) -> Element {
    rsx! {
        table { class: "result-table",
            thead {
                tr {
                    for col in result.columns.iter() {
                        th { "{col}" }
                    }
                }
            }
            tbody {
                for row in result.rows.iter() {
                    tr {
                        for cell in row.iter() {
                            td { "{cell}" }
                        }
                    }
                }
            }
        }
    }
}
