## 💻 Technologies

[![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org)
[![Linux](https://img.shields.io/badge/Linux-FCC624?style=for-the-badge&logo=linux&logoColor=black)](https://www.linux.org)

---

<div align="center">
<h1>⚒️ Anvil - AppImage Installer</h1>
  <img src="assets/anvil-logo.png" alt="Anvil Logo" width="300" height="300">

### 🌍 Choose Language
[**English**](#english) • [**Español**](README.es.md)
</div>

---


# ⚒️ Anvil - AppImage Installer for Linux

## 🚀 Description

Anvil is a command-line tool written in Rust that automatically installs AppImages into your Linux system. No more manually extracting icons or creating desktop entries – Anvil does it all for you.

## ✨ Features

- 📦 **Automatic Organization** - Moves AppImages to a dedicated folder (`~/.Applications_AppImage` by default)
- 🔧 **Permission Management** - Automatically sets executable permissions (chmod +x)
- 🎨 **Smart Icon Extraction** - Extracts the best quality icon from the AppImage (PNG prioritized by file size)
- 🖥️ **Desktop Integration** - Creates proper `.desktop` entries in `~/.local/share/applications/`
- 🎯 **Customizable** - Set custom names, icons, and categories
- 🎭 **Verbose Mode** - See exactly what's happening under the hood with `--verbose`
- 🎨 **Beautiful Output** - Color-coded messages for errors, warnings, and success

## 📦 Installation

### From Source
```bash
git clone https://github.com/Jorge-Guedes/anvil.git
cd anvil
cargo build --release
sudo cp target/release/anvil /usr/local/bin/
```

### Using Cargo
```bash
cargo install --git https://github.com/Jorge-Guedes/anvil.git
```

## 🚀 Usage

```bash
# Basic usage
anvil --source ~/Downloads/MyApp.AppImage

# With custom name and categories
anvil --source ~/Downloads/MyApp.AppImage --name "MyApp" --categories "Development;IDE;"

# Using a custom icon
anvil --source ~/Downloads/MyApp.AppImage --icon ~/icons/myapp.png

# See detailed progress
anvil --source ~/Downloads/MyApp.AppImage --verbose

# Get help
anvil --help
```

## ⚙️ Options

| Option | Description | Default |
|--------|-------------|---------|
| `-s, --source` | Path to the AppImage file | **Required** |
| `-d, --destination` | Destination directory under HOME | `.Applications_AppImage` |
| `-n, --name` | Custom name for the application | (from filename) |
| `-i, --icon` | Path to a custom icon file | (extracted from AppImage) |
| `-c, --categories` | Desktop entry categories | `Utility` |
| `-v, --verbose` | Show detailed progress messages | `false` |
| `-h, --help` | Print help | - |
| `-V, --version` | Print version | - |

## 📁 What Anvil Does

1. **Validates** the AppImage file
2. **Creates** a dedicated folder in `~/.Applications_AppImage/AppName/`
3. **Moves** the AppImage to its new home
4. **Sets** executable permissions (755)
5. **Extracts** the best icon (if no custom icon provided)
6. **Creates** a `.desktop` entry in `~/.local/share/applications/`
7. **Updates** the application database

## 🔧 Example

```bash
$ anvil --source ~/Downloads/Flameshot-13.3.0.x86_64.appimage --name "Flameshot" --verbose

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

## 🏗️ Building from Source

```bash
# Clone the repository
git clone https://github.com/Jorge-Guedes/anvil.git
cd anvil

# Build in debug mode
cargo build

# Build in release mode
cargo build --release

# Run tests
cargo test
```

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

