# Arquitectura

```plaintext
HyprDjxs4n/
│
├── index.html <-- El contenedor base donde Leptos inyectará la UI.
├── Trunk.toml           
│
├── public/                  
│   ├── walppapers/        
│   │   ├── img-1        
│   │   ├── img-2   
│   │   ├── img-3
│   │   └── img...    
│   ├── image.png
│   ├── leptos.svg              
│   └── tauri.svg
│
├── src/                 
│   ├── components/         
│   │   └── gallery.rs      
│   ├── css/
│   │   └── styles.css
│   │
│   ├── app.rs              
│   └── main.rs             
│
├── src-tauri/    
│   ├── capabilities/       
│   │   │   └── default.json
│   │   │
│   │   ├── gen/schemas/       
│   │   │   └── default.json
│   │   └── icons/         
│   │       └── default.json
│   │
│   ├── src/
│   │   ├── lib.rs  
│   │   └── main.rs   
├── tauri.conf.json       
│   ├── Debug/ 
│   └── etc...  
│      
├── cargo.toml
├── index.html
├── styles.css
├── .tomlTrunk
├── COPYING.md
└── ARCHITECURE.md
```
