## 💻 Tecnologías

[![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org)
[![Linux](https://img.shields.io/badge/Linux-FCC624?style=for-the-badge&logo=linux&logoColor=black)](https://www.linux.org)

---

<div align="center">
<h1>⚒️ Anvil - Instalador de AppImages</h1>
  <img src="assets/anvil-logo.png" alt="Anvil Logo" width="300" height="300">

### 🌍 Elige Idioma
[**English**](README.md) • [**Español**](#español)
</div>

---


# ⚒️ Anvil - Instalador de AppImages para Linux

## 🚀 Descripción

Anvil es una herramienta de línea de comandos escrita en Rust que instala automáticamente AppImages en tu sistema Linux. Olvídate de extraer iconos manualmente o crear accesos directos – Anvil hace todo por ti.

## ✨ Características

- 📦 **Organización Automática** - Mueve los AppImages a una carpeta dedicada (`~/.Applications_AppImage` por defecto)
- 🔧 **Gestión de Permisos** - Establece automáticamente permisos de ejecución (chmod +x)
- 🎨 **Extracción Inteligente de Iconos** - Extrae el icono de mejor calidad del AppImage (prioriza PNG por tamaño de archivo)
- 🖥️ **Integración con el Escritorio** - Crea accesos directos `.desktop` en `~/.local/share/applications/`
- 🎯 **Personalizable** - Establece nombres, iconos y categorías personalizadas
- 🎭 **Modo Verbose** - Mira exactamente qué pasa bajo el capó con `--verbose`
- 🎨 **Salida Atractiva** - Mensajes con colores para errores, advertencias y éxitos

## 📦 Instalación

### Desde el Código Fuente
```bash
git clone https://github.com/Jorge-Guedes/anvil.git
cd anvil
cargo build --release
sudo cp target/release/anvil /usr/local/bin/
```

### Usando Cargo
```bash
cargo install --git https://github.com/Jorge-Guedes/anvil.git
```

## 🚀 Uso

```bash
# Uso básico
anvil --source ~/Descargas/MiApp.AppImage

# Con nombre y categorías personalizadas
anvil --source ~/Descargas/MiApp.AppImage --name "MiApp" --categories "Development;IDE;"

# Usando un icono personalizado
anvil --source ~/Descargas/MiApp.AppImage --icon ~/iconos/miapp.png

# Ver progreso detallado
anvil --source ~/Descargas/MiApp.AppImage --verbose

# Ver ayuda
anvil --help
```

## ⚙️ Opciones

| Opción | Descripción | Por Defecto |
|--------|-------------|-------------|
| `-s, --source` | Ruta al archivo AppImage | **Requerido** |
| `-d, --destination` | Directorio destino dentro de HOME | `.Applications_AppImage` |
| `-n, --name` | Nombre personalizado para la aplicación | (del nombre del archivo) |
| `-i, --icon` | Ruta a un archivo de icono personalizado | (extraído del AppImage) |
| `-c, --categories` | Categorías para el acceso directo | `Utility` |
| `-v, --verbose` | Muestra mensajes de progreso detallados | `false` |
| `-h, --help` | Muestra la ayuda | - |
| `-V, --version` | Muestra la versión | - |

## 📁 Qué Hace Anvil

1. **Valida** el archivo AppImage
2. **Crea** una carpeta dedicada en `~/.Applications_AppImage/NombreApp/`
3. **Mueve** el AppImage a su nuevo hogar
4. **Establece** permisos de ejecución (755)
5. **Extrae** el mejor icono (si no se proporcionó uno personalizado)
6. **Crea** un acceso directo `.desktop` en `~/.local/share/applications/`
7. **Actualiza** la base de datos de aplicaciones

## 🔧 Ejemplo

```bash
$ anvil --source ~/Descargas/Flameshot-13.3.0.x86_64.appimage --name Flameshot --verbose

INFO: Valid format (appimage)
INFO: Base folder ready
INFO: App folder created: /home/user/.Applications_AppImage/Flameshot
INFO: Moving Flameshot-13.3.0.x86_64.appimage to /home/user/.Applications_AppImage/Flameshot/Flameshot-13.3.0.x86_64.appimage
INFO: File moved successfully
INFO: Execute permissions set
INFO: Extracting AppImage contents...
INFO: Found 7 icons
INFO: Best icon: "squashfs-root/usr/share/icons/hicolor/128x128/apps/flameshot.png"
INFO: Copying icon from squashfs-root/usr/share/icons/hicolor/128x128/apps/flameshot.png to /home/user/.Applications_AppImage/Flameshot/flameshot.png
INFO: Icon copied successfully
INFO: Icon extracted successfully
INFO: Desktop entries directory ready
INFO: Desktop entry created at /home/user/.local/share/applications/Flameshot.desktop
INFO: Application database updated successfully
SUCCESS: Flameshot installed successfully
```

## 🏗️ Compilar desde el Código Fuente

```bash
git clone https://github.com/Jorge-Guedes/anvil.git
cd anvil

cargo build

cargo build --release

cargo test
```

## 📄 Licencia

Este proyecto está bajo la Licencia MIT - ver el archivo [LICENSE](LICENSE) para más detalles.

