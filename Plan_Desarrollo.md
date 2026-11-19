# HyprDjsx4n Administrator
# Plan de desarrollo para no volverme loco en el proceso

Bases y dependencias necesarias:

- Tauri 2 + Leptos 0.8 (Trunk/csr)
- Hyprland/Wayland
- Sww/swwdaemon
- hypaper
- mpvpaper

---

> [!NOTE] Reestructuracion total lo hago de nuez

> [!CAUTION] Es necesario separar el swww daemon si se va a ejecutar los wallpaper dinamio de mvpaper

---
## NOTA DJ DEL PASADO
DJ del pasado aqui, mira vamos a reconstruir todo 1 por lo que pesa y 2 por como se organiza el proyecto asi que te anoto las cosas que vamos a conservar para que no te olvide

### Qué se conserva y qué se descarta del proyecto actual
> Lógica que SÍ conservamos

- 1 Detección automática de carpeta por idioma del sistema (scanner.rs):
    - Uso de dirs::picture_dir() (respeta XDG_PICTURES_DIR: Pictures en inglés, Imágenes en español) + subcarpeta /wallpapers, creándola si no existe.
- 2 Configuración persistente en JSON (config.rs):
    - Archivo en ~/.config/hyprdjxs4n/config.json con #[serde(default)] para tolerar campos nuevos sin romper configs viejas.
    - Parámetros de swww (type, duration, fps, step, pos, angle, bezier, wave, resize, filter) con defaults (grow, center, 1.5s, 60fps, step 90).
    - Parámetros de mpvpaper (mpv_output, mpv_options como no-audio loop-file=1).

- 3 Identidad visual del widget (styles.css + tauri.conf.json):
    - Ventana transparente, sin bordes (decorations: false, transparent: true), anclada abajo como dock/launcher.
    - Barra superior flotante de filtros tipo píldora con blur.
    - Galería horizontal en panal hexagonal (clip-path: polygon(...) con desplazamiento vertical en nodos pares nth-child(2n)).

- 4 Requisitos pendientes de Notas.md:
    - Recordar el último wallpaper activo y su motor entre sesiones.
    - Mantener vivos swww-daemon / mpvpaper tras cerrar el widget o reiniciar el equipo.
    - Matar limpiamente el motor anterior al cambiar entre wallpaper estático/GIF y video.

---

## Fase 0 — Bases del backend (fundamentos)
Meta 
Tener el proyecto creado desde cero, sin basura ni dependencias cruzadas, compilando backend y frontend.

Tareas:

- Inicializar workspace Tauri v2 + Leptos 0.8 + Trunk.

- Configurar tauri.conf.json (ventana transparente sin bordes, protocol-asset con scope $HOME/**, CSP limpio).

- Conectar api.rs <-> commands.rs con un comando ping / get_wallpapers real (corrigiendo el empaquetado de argumentos de invoke en Tauri v2).

- Criterio de éxito: cargo check en ambos crates sin warnings y ventana transparente abriendo en Hyprland.

- [] actividad pendiente

---

## Hito 1 — Escáner real y separación de formatos (ALL | IMG | GIF | VID)
- Meta: Leer la carpeta de wallpapers del usuario según el idioma del sistema y clasificar correctamente cada archivo.
- Tareas:
- Implementar scanner.rs con MediaKind::{Image, Gif, Video}:
- Image: png, jpg, jpeg, webp, avif.
- Gif: gif.
- Video: mp4, webm, mkv, mov.
- Soportar carpeta personalizada desde config.json (fallback a XDG_PICTURES_DIR/wallpapers).
- Ordenamiento consistente (por nombre o fecha de modificación).
- Criterio de éxito: La barra superior filtra por ALL, IMG, GIF y VID, y cada hexágono muestra su etiqueta real (PNG, GIF, MP4, etc.).

- [] actividad pendiente

---

## Hito 2 — Sistema de miniaturas (Thumbnails) y panal ligero
- Meta: Eliminar el peso en RAM/GPU y poder previsualizar videos en el panal.
- Tareas:
- Implementar thumbnails.rs con caché en ~/.cache/hyprdjxs4n/thumbs/ usando hash de ruta + timestamp (mtime).
- Generar miniaturas reducidas (~280px) para imágenes/GIFs y extraer primer frame para videos (usando ffmpeg o ffmpegthumbnailer si está en el sistema, con fallback visual si no está).
- Servir la ruta de la miniatura al frontend (thumb_path) y usar loading="lazy" en <img>.
- Limpiar main.css: corregir .filter-btn.active, traducir scroll vertical del ratón a scroll horizontal en el panal y resaltar el wallpaper actualmente activo.
- Criterio de éxito: El widget abre rápido incluso con +100 wallpapers (incluidos videos) y mantiene bajo consumo de RAM.

- [] actividad pendiente

---
## Hito 3 — Motor de wallpapers en Hyprland (swww + mpvpaper)
- Meta: Aplicar wallpapers reales al hacer clic, alternando entre estático/GIF y video sin dejar procesos zombis.
- Tareas:
- Implementar engine/mod.rs con registro de PID en ~/.cache/hyprdjxs4n/active_engine.json.
- Al aplicar Image / Gif: matar mpvpaper si estaba activo, asegurar que swww-daemon esté corriendo (desacoplado con setsid/process_group(0)), ejecutar swww img con los parámetros de TransitionConfig.
- Al aplicar Video: matar mpvpaper anterior si existe, limpiar/detener proceso incompatible si aplica, lanzar mpvpaper desacoplado con mpv_output y mpv_options.
- Devolver ApplyResult real al frontend para mostrar estado o error (ej. si falta swww o mpvpaper instalado).
- Criterio de éxito: Puedes cambiar libremente MP4 -> PNG -> GIF -> MP4 desde el panal y los procesos sobreviven si cierras el widget.

- [] actividad pendiente

---

## Hito 4 — Persistencia entre sesiones y arranque con Hyprland
- Meta: Que el wallpaper activo se recuerde y se restaure automáticamente al iniciar sesión.
- Tareas:
- Guardar last_wallpaper y last_engine en estado persistente al aplicar con éxito.
- Añadir flag CLI --restore en src-tauri/src/main.rs: lee la config/estado, aplica el último wallpaper en segundo plano y termina sin levantar WebKitGTK.
- Documentar la línea recomendada para hyprland.conf (exec-once = hyprdjxs4n --restore y reglas de ventana windowrulev2 para float, pin, noborder, noblur si aplica).
- Criterio de éxito: Al reiniciar Hyprland, el último wallpaper (imagen, GIF o video) se restaura automáticamente.

- [] actividad pendiente

---
## Hito 5 — Evolución a largo plazo (Post-MVP)
Una vez estable lo anterior, el diseño modular permite ir añadiendo sin reescribir:
1. Multi-motor configurable: Soporte para hyprpaper o delegación a waypaper para usuarios que prefieran otro daemon.
2. Soporte multi-monitor: Detectar salidas con hyprctl monitors -j y elegir en qué monitor aplicar el wallpaper.
3. Panel rápido de ajustes en UI: Cambiar ruta de carpeta, tipo de transición de swww (grow, wipe, fade, duración) y opción de "cerrar widget al seleccionar".
4. Navegación por teclado: Flechas izquierda/derecha + Enter para aplicar + Esc para cerrar el widget al instante.
5. Watcher de carpeta: Detectar automáticamente nuevos wallpapers añadidos sin reiniciar la app.

- [] actividad pendiente
---

