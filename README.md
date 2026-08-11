# App_wallpaper_Changer
> Preview de la app

![alt text](/App_wallpaper_Changer/HyprDjxs4n/public/image.png)

> Nota el framework es tauri con leptos

para ejecutar el widget copiar el siguiente comandoca
```bash
cargo tauri dev
```

# Arquitectura

```plaintext
HyprDjxs4n/
│
├── index.html <-- El contenedor base donde Leptos inyectará la UI.
├── Trunk.toml           
│
├── src/                 
│   ├── components/        
│   │   ├── gallery.rs      
│   │   ├── preview.rs      
│   │   └── settings.rs     
│   ├── app.rs              
│   └── main.rs             
│
├── src-tauri/              
│   ├── tauri.conf.json     
│   ├── src/
│   │   ├── commands/       
│   │   │   ├── mod.rs
│   │   │   └── apply.rs    
│   │   ├── core/           
│   │   │   ├── scanner.rs  
│   │   │   └── engine.rs   
│   │   └── main.rs         
```

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


Make sure you have installed the prerequisites for your OS: https://tauri.app/start/prerequisites/, then run:
  cd HyprDjxs4n
  cargo tauri android init

For Desktop development, run:
  cargo tauri dev

For Android development, run:
  cargo tauri android dev



