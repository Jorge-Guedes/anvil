use colored::Colorize;
use std::ffi::OsStr;
use std::path::Path;

pub fn check_extension(source: &str) -> bool {
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

pub fn move_appimage(source: &str, destination: &Path, file_name: &OsStr) -> bool {
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
