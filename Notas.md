## Cosas a recordar

- 1 que puedo darme el acomodo global y modulas para que no me genere problemas

- 2 recordar las preferencias entre sesion segun el wallpaper activo y que estado tiene antes de contiunar

- 3 matener vivos esos procesos ubna vez se reinicie la computadora oh se abra nuevamente

Cosas por implementar DJ del futuro:
para poder mantener vivo los procesos que lance como el mpvpaper, oh el daemon, necesito poder dalres cuello con kill para poder cambiar entre un wallpaper estatico a uno vivo usando el este segun que requiera

# Estatus actual:
- Completado
- Etapa A — Esqueleto de módulos:
- src-tauri/src/scanner.rs: Wallpaper (ahora pub), get_wallpaper_dir, scan_dir (movidos de lib.rs sin cambios de lógica).
- src-tauri/src/commands.rs: get_wallpapers + apply_wallpaper (stub) + struct ApplyResult { success, engine, message } — contrato nuevo con la UI.
- lib.rs reducido a orquestación (mod + generate_handler).
- Fix OUT_DIR (rust-analyzer): falso positivo de tauri::generate_context!(); resuelto desactivando el diagnóstico macro-error en .vscode/settings.json.
- Etapa B — Persistencia + estado:
- src-tauri/src/config.rs: Config + TransitionConfig con defaults grow center 1.5s (fps 60, step 90); load/save en ~/.config/hyprdjxs4n/config.json con #[serde(default)] (tolerante a campos faltantes).
- src-tauri/src/state.rs: AppState { inner: Mutex<Inner> } con Engine { None, Swww, Mpvpaper }, current_wallpaper, mpv_child, daemon_child.
- lib.rs: mod config/state + .manage(AppState::new()) (carga config al arrancar).
- cargo check limpio (solo 5 warnings esperados de dead_code — campos aún no usados por el motor).

