# HyprDjsx4n Administrator
# Plan de desarrollo para no volverme loco en el proceso

Bases y dependencias necesarias:

- Tauri 2 + Leptos 0.8 (Trunk/csr)
- Hyprland/Wayland
- Sww/swwdaemon
- hypaper
- mpvpaper

El backend actual solo escanea la referencia doned se guarda los wallpapers y apply wallpaper es un stub lo que significa que guarda temporalmente los archivos.

> [!CAUTION] Es necesario separar el swww daemon si se va a ejecutar los wallpaper dinamio de mvpaper

---

## Fase 0 — Bases del backend (fundamentos)
Objetivo: estructura que soporte todo lo demás.
- Reorganizar src-tauri/src/lib.rs en módulos:
- scanner.rs — escaneo de directorios (actual scan_dir/get_wallpaper_dir).
- wallpaper/ — motor de aplicación con trait/builder común: set_static(path), set_animated(path), detect_backends() (detecta vía PATH: swww, hyprpaper, mpvpaper; prioridad: mpvpaper (animado) > swww (estático) > hyprpaper).
- state.rs — estado global (Mutex<Option<Child>> para el PID de mpvpaper actual, último wallpaper aplicado).
- config.rs — ~/.config/hyprdjxs4n/config.json (setter preferido, carpeta de fondos, opciones mpv).
- apply_wallpaper real: enruta por tipo (imagen→swww img, video/gif→mpvpaper), devuelve Result { success, message } al frontend.
- Comando extra get_status: qué backend está activo, wallpaper actual, PID mpv.
Verificación: cargo check + probar en tauri dev que una imagen cambia el fondo.

---

## Fase 1 — mpvpaper (prioridad principal)
Objetivo: fondo animado real con mpv.
1. Gestión del proceso:
- Spawn: mpvpaper -f -o "no-audio loop-file=1" <output> <ruta> (guardar Child, retener stdin para que no muera). 

- Al aplicar nuevo wallpaper → matar proceso mpv anterior primero (evitar zombies/apilamiento).
1. Resolución del conflicto con swww-daemon:
- Al aplicar video → pausar/limpiar swww (swww clear o matar daemon temporalmente) para que mpvpaper sea visible.
- Al aplicar imagen → matar mpvpaper, reiniciar/continuar swww-daemon → swww img.
1. Multi-monitor: resolver salida con hyprctl monitors -j (nombre del monitor activo o * para todos).
2. Feedback: comando apply_wallpaper devuelve éxito/error (stderr del proceso) y la UI muestra toast de estado.
Verificación manual: aplicar un mp4 → se ve animado; aplicar imagen → vuelve a swww sin procesos mpv huérfanos (pgrep -a mpvpaper).

> [!note] Estructura nueva del backend (src-tauri/src/)
```
registro de comandos, estado global
scanner.rs        → escaneo de ~/Imágenes/wallpapers (funciones actuales, sin cambios)
config.rs         → ~/.config/hyprdjxs4n/config.json (serde)
state.rs          → AppState { config, active_engine, current_wallpaper, mpv_child }
engine/mod.rs     → facade: apply(path) según tipo + config
engine/swww.rs    → daemon ensure/kill + img con transiciones
engine/mpv.rs     → spawn/kill mpvpaper + resolución de monitores
```

- [ ] actividad por hacer
- [x] actividad echa

---

## Fase 2 — Filtros y ordenamiento (estilo skwd)

- Extender get_wallpapers(search, kind, sort) en backend:
- kind: ALL | IMG | VID (como skwd: filter-bar ftl).
- sort: date (más reciente), name, recent (aplicado recientemente → requiere historial en config), res.
- Búsqueda por nombre en backend (o filtro en frontend con signal).
Fase 3 — Selector UI completo estilo skwd
- Barra superior: buscador + botones de filtro (ALL/IMG/VID) + dropdown de orden + indicador de estado (aplicado/en curso).
- Grid de tarjetas (mantener hexágono actual o pasar a card tipo skwd) con badge de tipo (IMG/VID), hover con preview, click → aplicar con feedback.
- Panel de detalle/preview grande con botón "Aplicar".
- Estado activo: resaltar el wallpaper actualmente aplicado.
Fase 4 — Ampliación (posterior)
- Historial de aplicados, favoritos, carpetas múltiples (no solo ~/Pictures/wallpapers), settings UI (setter preferido, opciones mpv: auto-pause -p -a MAX, volumen/calikey), snippets de UI de settings/selector.ftl (badges, layout card/hex, columnas/filas).

 Lógica híbrida (núcleo de la Fase 1)
Predeterminado: imágenes estáticas con swww-daemon.
Imagen (png/jpg/webp/gif) → engine/swww.rs:
1. Si mpvpaper activo → matar Child (kill + wait, reaparar) y limpiar estado.
2. ensure_daemon(): si no responde swww query, relanzar swww-daemon y esperar socket (retry ~2s).
3. swww img --transition-type ... <ruta> con la transición de la config.
Video (mp4/webm/mov/mkv/avi) → engine/mpv.rs:
1. Si swww-daemon corriendo → matarlo (PID guardado o pkill -x swww-daemon), verificar con swww query que murió (evita el warning "daemon blocks mpvpaper").
2. Spawn mpvpaper -o "no-audio loop-file=1" <output> <ruta> sin -f (guardar Child para poder matarlo; reaper con try_wait/thread para evitar zombies).
3. <output>: por defecto * o activo vía hyprctl monitors -j (config).
4. Al volver a imagen → matar mpv → relanzar daemon → swww img con transición.
gif → lo maneja swww (soporta GIF animado), solo mp4/etc van a mpvpaper.
Nota honesta: la transición de swww solo aplica imagen→imagen; al cruzar a video no hay crossfade (mpvpaper no lo ofrece) — entrada directa en layer background.

- [ ] actividad por hacer
- [x] actividad echa

---
## Fase 3 — Selector UI completo estilo skwd
- Barra superior: buscador + botones de filtro (ALL/IMG/VID) + dropdown de orden + indicador de estado (aplicado/en curso).
- Grid de tarjetas (mantener hexágono actual o pasar a card tipo skwd) con badge de tipo (IMG/VID), hover con preview, click → aplicar con feedback.
- Panel de detalle/preview grande con botón "Aplicar".
- Estado activo: resaltar el wallpaper actualmente aplicado.

Config con transiciones de swww (adaptado de sus flags)
config.json: modo, salida mpv, carpeta de fondos, y bloque transition:
type (none/simple/fade/left/wipe/wave/grow/center/random), duration, fps, step, angle, pos, bezier, wave, resize (crop/fit/stretch), filter. Se inyectan como flags de swww img solo cuando difieren de defaults.

- [ ] actividad por hacer
- [x] actividad echa

---

## Fase 4 — Ampliación (posterior)
- Historial de aplicados, favoritos, carpetas múltiples (no solo ~/Pictures/wallpapers), settings UI (setter preferido, opciones mpv: auto-pause -p -a MAX, volumen/calikey), snippets de UI de settings/selector.ftl (badges, layout card/hex, columnas/filas).

 Comandos Tauri
apply_wallpaper(path) → ya no stub: enruta al motor, devuelve ApplyResult { success, engine: "swww"|"mpv", message }.
get_state → motor activo, wallpaper actual, config (para marcar tarjeta activa).
get_config / save_config (transiciones) — UI de ajustes queda para Fase 4.

- [ ] actividad por hacer
- [x] actividad echa

---
## 5. Frontend (mínimo en esta fase)
- apply_wallpaper ya no ignora la respuesta: toast de estado (éxito/error, ej. "mpvpaper iniciado", "swww listo").
- Corrección: quitar prefijo asset:// al pasar ruta real al backend (o devolver path crudo y usar convertFileSrc en frontend).

---

## 6. Verificación por pasos
1. cargo check (workspace) → sin errores.
2. tauri dev: aplicar imagen → transición visible, swww query OK.
3. Aplicar mp4 → pgrep swww-daemon vacío, pgrep mpvpaper OK, video visible.
4. Volver a imagen → mpv muerto, daemon relanzado, transición OK.
5. Sin huérfanos: pgrep -a mpvpaper tras varios cambios = 1 máximo.