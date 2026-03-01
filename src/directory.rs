use colored::Colorize;
use dirs;
use std::fs;
use std::path::{Path, PathBuf};

pub fn get_home_subdirectory(folder_name: &str) -> Option<PathBuf> {
    match dirs::home_dir() {
        Some(path) => Some(path.join(folder_name)),
        None => None,
    }
}

pub fn setup_app_directory(base_path: &Path, app_name: &str) -> Option<PathBuf> {
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

pub fn setup_desktop_entries_dir() -> Option<PathBuf> {
    let home = match dirs::home_dir() {
        Some(path) => path,
        None => {
            eprintln!(
                "{} {}",
                "ERROR:".red().bold(),
                "No se pudo encontrar el directorio HOME".red()
            );
            return None;
        }
    };

    let desktop_dir = home.join(".local/share/applications");

    if let Err(e) = fs::create_dir_all(&desktop_dir) {
        eprintln!(
            "{} No se pudo crear el directorio de entradas: {}",
            "ERROR:".red().bold(),
            e
        );
        return None;
    }

    println!("{}", "Directorio de entradas listo".green().bold());
    Some(desktop_dir)
}
