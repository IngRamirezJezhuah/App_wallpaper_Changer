# Arquitectura

```plaintext
HyprDjxs4n/
├── COPYING.md
└── ARCHITECURE.md
├── Cargo.toml                  # Workspace: ui (Leptos) + src-tauri (Tauri v2)
├── Trunk.toml                  # Puerto 1420, ignore src-tauri
├── index.html                  # Solo CSS limpio + entrada WASM
├── src/                        # FRONTEND (Leptos CSR - Solo presentación)
│   ├── main.rs
│   ├── app.rs                  # Layout principal y estado global mínimo
│   ├── models.rs               # Structs espejadas del backend (WallpaperItem, MediaKind, etc.)
│   ├── api.rs                  # Invocaciones Tauri tipadas (unwrap seguro de errores)
│   ├── components/
│   │   ├── mod.rs
│   │   ├── filter_bar.rs       # Filtros: ALL | IMG | GIF | VID + indicador de motor
│   │   ├── hex_grid.rs         # Contenedor scroll horizontal (rueda vertical -> scroll X)
│   │   └── hex_card.rs         # Hexágono con thumbnail lazy, badge y estado activo
│   └── styles/
│       └── main.css
└── src-tauri/                  # BACKEND (Rust nativo - Lógica y sistema)
    ├── Cargo.toml
    ├── tauri.conf.json
    ├── capabilities/default.json
    └── src/
        ├── main.rs             # Parseo CLI (--restore) antes de abrir ventana
        ├── lib.rs              # Builder Tauri + AppState + handlers
        ├── config.rs           # Config (~/.config/hyprdjxs4n/config.json)
        ├── state.rs            # Estado runtime + persistencia último wallpaper
        ├── scanner.rs          # Escaneo + clasificación MediaKind (Image | Gif | Video)
        ├── thumbnails.rs       # Caché (~/.cache/hyprdjxs4n/thumbs/)
        ├── engine/
        │   ├── mod.rs          # Dispatcher + gestión de PIDs y kill limpio
        │   ├── swww.rs         # Backend imágenes/GIFs
        │   ├── mpvpaper.rs     # Backend videos
        │   └── waypaper.rs     # (Hito futuro) Integración opcional / hyprpaper
        └── commands.rs         # Comandos IPC expuestos al frontend
```
