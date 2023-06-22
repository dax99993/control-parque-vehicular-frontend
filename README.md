# Frontend para servicio de control parque vehicular.


## Requisitos
- Toolchain de Rust
- Trunk

## Instalacion

Instalar toolchain de Rust
```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Clonar el repositorio
```sh
git clone https://github.com/dax99993/control-parque-vehicular-frontend
cd control-parque-vehicular-frontend
```

### Instalar Trunk y objetivo de compilacion WASM
Esto creara la base de datos en Postgres, un Cache para blacklist de tokens en Redis y un servidor para correos con docker y con la configuracion para desarrollar en local,
```sh
cargo install --locked trunk
rustup target add wasm32-unknown-unknown
```

### Configuracion
De momento el frontend esta forzado a establecer conexion con la API en "http://localhost:8000"
El archivo que contiene esta informacion se encuentra en src/services/request.rs


### Ejecucion
Finalmente para ejecutar el frontend se debe estar en la carpeta principal del proyecto de frontend
donde se encuentra el archivo Cargo.toml y ejecutar
```sh
trunk serve --port 3000
```
