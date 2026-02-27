# ANVIL 🚀

## DESCRIPCIÓN
Forge-Link es una herramienta de línea de comandos (CLI) escrita en **Rust** diseñada para usuarios de Linux (especialmente Manjaro/Arch) que buscan una integración perfecta de sus aplicaciones AppImage en el entorno de escritorio.

## ¿QUÉ HACE?
Mover y ejecutar un AppImage no es suficiente para una experiencia nativa. Forge-Link automatiza todo el proceso de "instalación":

* **Organización**: Mueve el binario a una carpeta dedicada (por defecto `~/Applications`).
* **Permisos**: Asegura que el archivo sea ejecutable de forma automática (`chmod +x`).
* **Iconografía**: Extrae el icono original del interior del AppImage y lo instala en el sistema.
* **Integración**: Genera el archivo `.desktop` necesario para que la aplicación aparezca en tu menú de inicio, buscador y dock.

## ¿POR QUÉ USARLO?
Para evitar la tediosa tarea manual de crear accesos directos y extraer recursos cada vez que descargas una nueva herramienta portable.

---

# PAUTAS DE DESARROLLO (ROADMAP)

### FASE 1: El Esqueleto (CLI y Argumentos)
* **Herramienta**: Crate `clap`.
* **Objetivo**: Que el programa entienda comandos con banderas como `--path` o `-p`.
* **Práctica**: Estructuras (`structs`), macros y el tipo `Option` para valores opcionales.

### FASE 2: El Explorador (Validación de Rutas)
* **Herramienta**: `std::path` y `dirs`.
* **Objetivo**: Traducir rutas relativas a absolutas y localizar carpetas del sistema (`~/.local/share/...`) independientemente del usuario.
* **Práctica**: Uso de `PathBuf` y gestión de errores con `Result`.

### FASE 3: El Cirujano (Extracción de Recursos)
* **Herramienta**: `std::process::Command`.
* **Objetivo**: Invocar el AppImage con `--appimage-extract` para buscar el icono y el `.desktop` original en la carpeta temporal.
* **Práctica**: Ejecución de procesos externos y manipulación de archivos temporales.

### FASE 4: El Arquitecto (Generación de Archivos)
* **Herramienta**: `std::fs` y *Raw Strings*.
* **Objetivo**: Crear el archivo `.desktop` final y mover el binario a su destino permanente.
* **Práctica**: Escritura de archivos y permisos de Unix.

### FASE 5: Limpieza y Feedback
* **Herramienta**: `colored`.
* **Objetivo**: Borrar los restos de la extracción y mostrar mensajes de éxito elegantes.