# App_wallpaper_Changer
```bash

┌─[djxs4n@DjOs] - [~/Documentos/Programacion/hypaprDjxs4n/App_wallpaper_Changer] - [667]
└─[$] sh <(curl https://create.tauri.app/sh)                                                                                                           
  % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
                                 Dload  Upload   Total   Spent    Left  Speed
100 16121  100 16121    0     0  59312      0 --:--:-- --:--:-- --:--:-- 59268
info: downloading create-tauri-app
✔ Project name · HyprDjxs4n
✔ Package name · hyprdjxs4n
✔ Identifier · com.djxs4n.hyprdjxs4n
✔ Choose which language to use for your frontend · Rust - (cargo)
✔ Choose your UI template · Leptos - (https://leptos.dev/)

Template created!

Your system is missing dependencies (or they do not exist in $PATH):

╭───────────────┬───────────────────────────────────────────────────────────────────╮
│ Rust          │ Visit https://www.rust-lang.org/learn/get-started#installing-rust │
├───────────────┼───────────────────────────────────────────────────────────────────┤
│ Tauri CLI     │ Run `cargo install tauri-cli --version ^2.0.0 --locked`           │
├───────────────┼───────────────────────────────────────────────────────────────────┤
│ Trunk         │ Run `cargo install trunk --locked`                                │
├───────────────┼───────────────────────────────────────────────────────────────────┤
│ wasm32 target │ Run `rustup target add wasm32-unknown-unknown`                    │
╰───────────────┴───────────────────────────────────────────────────────────────────╯
```

> Nota el framework es tauri con leptos

para ejecutar el widget copiar el siguiente comando
```bash
cargo tauri dev
```

Make sure you have installed the prerequisites for your OS: https://tauri.app/start/prerequisites/, then run:
  cd HyprDjxs4n
  cargo tauri android init

For Desktop development, run:
  cargo tauri dev

For Android development, run:
  cargo tauri android dev

# Arquitectura

```plaintext
HyprDjxs4n/
│
├── index.html <-- El contenedor base donde Leptos inyectará la UI.
├── Trunk.toml              <-- Configuración de empaquetado del frontend.
│
├── src/                    <-- FRONTEND (UI con Leptos)
│   ├── components/         <-- NUEVA CARPETA: Aquí irá la UI modular
│   │   ├── gallery.rs      <-- La cuadrícula/hexágonos de los wallpapers
│   │   ├── preview.rs      <-- Reproductor de video/imagen seleccionada
│   │   └── settings.rs     <-- Menú flotante de configuración
│   ├── app.rs              <-- Enrutamiento y estado global de la UI
│   └── main.rs             <-- Punto de entrada de WebAssembly
│
├── src-tauri/              <-- ⚙️ BACKEND (Lógica del Sistema Operativo)
│   ├── tauri.conf.json     <-- Permisos, tamaño de ventana y comandos IPC.
│   ├── src/
│   │   ├── commands/       <-- NUEVA CARPETA: Funciones llamadas desde la UI
│   │   │   ├── mod.rs
│   │   │   └── apply.rs    <-- Funciones #[tauri::command] para aplicar fondos
│   │   ├── core/           <-- NUEVA CARPETA: Lógica pura del sistema
│   │   │   ├── scanner.rs  <-- Lee el disco buscando imágenes/videos
│   │   │   └── engine.rs   <-- Ejecuta los procesos de sww y mpvpaper
│   │   └── main.rs         <-- Inicialización de Tauri y registro de comandos
```
