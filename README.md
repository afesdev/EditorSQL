# EditorSQL 🚀

**EditorSQL** es un editor de bases de datos moderno y minimalista diseñado para SQL Server, que integra un asistente de Inteligencia Artificial para simplificar la escritura de consultas y la gestión de datos. Construido íntegramente en **Rust**, ofrece un rendimiento excepcional y una experiencia de usuario fluida mediante una arquitectura de escritorio nativa.

## 🎯 Objetivo
El objetivo de EditorSQL es democratizar el acceso a la manipulación de datos, permitiendo que tanto expertos como principiantes puedan interactuar con bases de datos SQL Server utilizando lenguaje natural, a la vez que proporciona herramientas robustas para la ejecución manual de scripts.

## ✨ Características Principales
- **🤖 Asistente IA Integrado:** Chat interactivo que traduce lenguaje natural a consultas SQL precisas y listas para ejecutar.
- **🔌 Gestión de Conexiones:** Soporte completo para SQL Server, incluyendo:
  - Seguridad Integrada (Windows Authentication).
  - Autenticación SQL estándar.
  - Configuración de cifrado y certificados de servidor.
  - Test de conexión con pre-carga de bases de datos disponibles.
- **📝 Editor de Consultas:** Un área de trabajo limpia para escribir, editar y ejecutar scripts SQL manualmente.
- **📊 Visualización de Resultados:** Tablas de datos optimizadas para visualizar la información retornada por el servidor.
- **🌲 Explorador de Bases de Datos:** Panel lateral para navegar por las bases de datos del servidor conectado.
- **🎨 Interfaz Moderna:** Diseño oscuro (Dark Mode) enfocado en la productividad y la claridad visual.

## 🛠️ Tecnologías Utilizadas
Este proyecto utiliza el stack más moderno y seguro del ecosistema Rust:
- **[Tauri](https://tauri.app/):** Framework para la construcción de aplicaciones de escritorio ligeras y seguras.
- **[Dioxus](https://dioxuslabs.com/):** Framework de interfaz de usuario tipo React escrito en Rust para una reactividad eficiente.
- **[Rust](https://www.rust-lang.org/):** Lenguaje de programación base que garantiza seguridad de memoria y alto rendimiento.
- **CSS3:** Estilos personalizados para una interfaz refinada y profesional.

## 📂 Estructura del Proyecto
```text
├── src/                # Frontend (Dioxus)
│   ├── api.rs          # Cliente para comunicación con el backend (Tauri Commands)
│   ├── app.rs          # Componentes de la interfaz de usuario y lógica de vista
│   ├── main.rs         # Punto de entrada de la aplicación UI
│   └── state.rs        # Gestión del estado global de la aplicación
├── src-tauri/          # Backend (Rust Nativo)
│   ├── src/            # Lógica de conexión a BD, ejecución de SQL e integración de IA
│   └── tauri.conf.json # Configuración del ecosistema Tauri
├── assets/             # Recursos estáticos (CSS, Imágenes)
└── Cargo.toml          # Dependencias y configuración de Rust
```

## 🚀 Desarrollo Actual
Actualmente, el proyecto se encuentra en una fase funcional estable que incluye:
- [x] Interfaz de usuario base con paneles colapsables.
- [x] Sistema de conexión parametrizado para SQL Server.
- [x] Ejecución de consultas SQL con visualización en tablas.
- [x] Integración de flujo de chat con generación automática de SQL.
- [x] Navegación básica del árbol de bases de datos.

## 🛠️ Instalación y Configuración

### Requisitos previos
- Rust instalado (`rustup`)
- Node.js (opcional, para herramientas de bundling adicionales)
- Dependencias de sistema de Tauri (ver [guía oficial](https://tauri.app/v1/guides/getting-started/prerequisites))

### Pasos
1. Clonar el repositorio:
   ```bash
   git clone https://github.com/tu-usuario/EditorSQL.git
   cd EditorSQL
   ```
2. Instalar el CLI de Dioxus:
   ```bash
   cargo install dioxus-cli
   ```
3. Ejecutar en modo desarrollo:
   ```bash
   dx serve
   # O para la versión de escritorio con Tauri
   cargo tauri dev
   ```

---
Desarrollado con ❤️ por [Andrés Espitia]
