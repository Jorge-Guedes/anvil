use clap::Parser;
use colored::Colorize;
use dirs;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(short, long)]
    source: String,

    #[arg(short, long, default_value = ".Applications_prueba")]
    destination: String,

    #[arg(short, long)]
    name: Option<String>,

    #[arg(short, long)]
    icon: Option<String>,

    #[arg(short, long, default_value = "Utility")]
    categories: String,
}

fn main() {
    let args = Args::parse();

    if !Path::new(&args.source).exists() {
        eprintln!(
            "{} {} {} {}",
            "ERROR:".red().bold(),
            "Archivo".red(),
            args.source.purple().bold(),
            "no encontrado".red()
        );
        return;
    }

    let Some(destination_path) = get_home_subdirectory(&args.destination) else {
        eprintln!(
            "{} {}",
            "Error".red().bold(),
            "No se pudo encontrar el HOME".red()
        );
        return;
    };

    let Some(file_name) = get_file_name(&args.source) else {
        return;
    };

    if !check_extension(&args.source) {
        return;
    }

    let source_path = Path::new(&args.source);
    let app_folder_name = get_app_name(&args.name, source_path);

    let capitalized_name = capitalize_name(&app_folder_name);

    let Some(app_dir) = setup_app_directory(&destination_path, &capitalized_name) else {
        return;
    };

    let final_file_path = app_dir.join(&file_name);

    if !move_appimage(&args.source, &final_file_path, &file_name) {
        return;
    }

    if !set_executable_permissions(&final_file_path) {
        return;
    }

    if let Some(icon_source) = &args.icon {
        if !copy_icon(icon_source, &app_dir) {
            return;
        }
    } else {
        if !extract_icon_from_appimage(&final_file_path, &app_dir) {
            eprintln!("{} No se pudo extraer icono", "WARN:".yellow());
            // No hacemos return, seguimos sin icono
        }
    }
}

fn get_home_subdirectory(folder_name: &str) -> Option<PathBuf> {
    match dirs::home_dir() {
        Some(path) => Some(path.join(folder_name)),
        None => None,
    }
}

fn get_file_name(source: &str) -> Option<std::ffi::OsString> {
    let path = Path::new(source);
    match path.file_name() {
        Some(name) => Some(name.to_os_string()),
        None => {
            eprintln!(
                "{} No se pudo obtener el nombre del archivo",
                "ERROR:".red()
            );
            None
        }
    }
}

fn check_extension(source: &str) -> bool {
    let extension = Path::new(source).extension().and_then(|s| s.to_str());
    match extension {
        Some(ext) if ext.to_lowercase() == "appimage" => {
            println!(
                "{} {} ({})",
                "INFO:".cyan().bold(),
                "Formato válido".cyan(),
                ext.yellow()
            );
            true
        }
        _ => {
            println!(
                "{} {}",
                "ERROR:".red().bold(),
                "El archivo debe ser un AppImage".red()
            );
            false
        }
    }
}

fn get_app_name(args_name: &Option<String>, source_path: &Path) -> String {
    if let Some(name) = args_name.as_ref() {
        name.clone()
    } else {
        source_path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("Application")
            .to_string()
    }
}

fn capitalize_name(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

fn setup_app_directory(base_path: &Path, app_name: &str) -> Option<PathBuf> {
    if let Err(e) = std::fs::create_dir_all(base_path) {
        eprintln!(
            "{} {} {}",
            "ERROR:".red().bold(),
            "No se pudo crear la carpeta base:".red(),
            e
        );
        return None;
    }
    println!("{}", "Carpeta base lista".green().bold());

    let app_dir = base_path.join(app_name);

    if let Err(e) = std::fs::create_dir_all(&app_dir) {
        eprintln!(
            "{} {} {}",
            "ERROR:".red().bold(),
            "No se pudo crear la carpeta de la aplicación:".red(),
            e
        );
        return None;
    }

    println!(
        "{} {}",
        "Carpeta de aplicación creada:".green().bold(),
        app_dir.display().to_string().purple()
    );

    Some(app_dir)
}

fn move_appimage(source: &str, destination: &Path, file_name: &std::ffi::OsStr) -> bool {
    println!(
        "Moviendo {} a su carpeta: {}",
        file_name.to_string_lossy().purple().bold(),
        destination.display().to_string().purple().bold()
    );

    if let Err(e) = std::fs::rename(source, destination) {
        eprintln!(
            "{} No se pudo mover el archivo: {}. \n{} intenta usar una ruta en el mismo disco.",
            "ERROR:".red().bold(),
            e,
            "TIP:".yellow()
        );
        return false;
    }

    println!("{}", "¡Archivo movido correctamente!".green().bold());
    true
}

fn set_executable_permissions(file_path: &Path) -> bool {
    match fs::metadata(file_path) {
        Ok(metadata) => {
            let mut permissions = metadata.permissions();
            permissions.set_mode(0o755);
            match fs::set_permissions(file_path, permissions) {
                Ok(()) => {
                    println!("{}", "Permisos de ejecución establecidos".green().bold());
                    true
                }
                Err(e) => {
                    eprintln!(
                        "{} {} {}",
                        "ERROR".red().bold(),
                        "No se pudieron establecer permisos:".red(),
                        e.to_string().red()
                    );
                    false
                }
            }
        }
        Err(e) => {
            eprintln!(
                "{} {} {}",
                "ERROR".red().bold(),
                "No se puede acceder al archivo".red(),
                e.to_string().red()
            );
            false
        }
    }
}

fn copy_icon(icon_source: &str, app_dir: &Path) -> bool {
    let icon_path = Path::new(icon_source);
    let icon_name = match icon_path.file_name() {
        Some(name) => name,
        None => {
            eprintln!(
                "{} {}",
                "ERROR:".red().bold(),
                "No se pudo obtener el nombre del archivo de icono".red()
            );
            return false;
        }
    };

    let icon_dest = app_dir.join(icon_name);

    println!(
        "Copiando icono {} a {}",
        icon_source.purple().bold(),
        icon_dest.display().to_string().purple().bold()
    );

    if let Err(e) = std::fs::copy(icon_source, icon_dest) {
        eprintln!(
            "{} No se pudo copiar el icono: {}",
            "ERROR:".red().bold(),
            e
        );
        return false;
    }

    println!("{}", "¡Icono copiado correctamente!".green().bold());
    true
}

fn find_icons_in_dir(dir: &Path) -> Vec<PathBuf> {
    let mut icons = Vec::new();

    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();

                if path.is_dir() {
                    let sub_icons = find_icons_in_dir(&path);
                    icons.extend(sub_icons);
                } else if path.is_file() {
                    if let Some(ext) = path.extension() {
                        if let Some(ext_str) = ext.to_str() {
                            match ext_str.to_lowercase().as_str() {
                                "png" | "svg" | "xpm" | "ico" => {
                                    icons.push(path);
                                }
                                _ => {}
                            }
                        }
                    }
                }
            }
        }
    }

    icons
}

fn select_best_icon(icons: Vec<PathBuf>) -> Option<PathBuf> {
    let mut png_icons = Vec::new();
    let mut other_icons = Vec::new();

    for icon in icons {
        if let Some(ext) = icon.extension() {
            if let Some(ext_str) = ext.to_str() {
                if ext_str.to_lowercase() == "png" {
                    png_icons.push(icon);
                } else {
                    other_icons.push(icon);
                }
            } else {
                other_icons.push(icon);
            }
        } else {
            other_icons.push(icon);
        }
    }

    fn heaviest_icon(icons: Vec<PathBuf>) -> Option<PathBuf> {
        let mut best = None;
        let mut max_size = 0;

        for icon in icons {
            if let Ok(metadata) = fs::metadata(&icon) {
                let size = metadata.len();
                if size > max_size {
                    max_size = size;
                    best = Some(icon);
                }
            }
        }
        best
    }

    if !png_icons.is_empty() {
        if let Some(best_png) = heaviest_icon(png_icons) {
            return Some(best_png);
        }
    }

    if !other_icons.is_empty() {
        if let Some(best_other) = heaviest_icon(other_icons) {
            return Some(best_other);
        }
    }

    None
}

fn extract_icon_from_appimage(appimage_path: &Path, app_dir: &Path) -> bool {
    println!("{} Extrayendo contenidos del AppImage...", "INFO:".cyan());
    let status = match Command::new(appimage_path)
        .arg("--appimage-extract")
        .status()
    {
        Ok(status) if status.success() => {
            let icons = find_icons_in_dir(Path::new("squashfs-root"));

            if icons.is_empty() {
                eprintln!("{} No se encontraron iconos", "WARN:".yellow());
                let _ = fs::remove_dir_all("squashfs-root");
                return false;
            }

            println!("{} Se encontraron {} iconos", "INFO:".cyan(), icons.len());

            if let Some(best_icon) = select_best_icon(icons) {
                println!("{} Mejor icono: {:?}", "INFO:".cyan(), best_icon);

                if let Some(icon_str) = best_icon.to_str() {
                    if !copy_icon(icon_str, app_dir) {
                        eprintln!("{} No se pudo copiar el icono extraído", "WARN:".yellow());
                    }
                } else {
                    eprintln!("{} La ruta del icono no es válida", "WARN:".yellow());
                }
            }

            let _ = fs::remove_dir_all("squashfs-root");
            true
        }
        Ok(status) => {
            eprintln!(
                "{} {} {}",
                "ERROR:".red().bold(),
                "El comando falló con código: {}".red(),
                status
            );
            return false;
        }
        Err(e) => {
            eprintln!("{} No se pudo ejecutar: {}", "ERROR:".red().bold(), e);
            return false;
        }
    };
    true
}
