use clap::Parser;
use colored::Colorize;
use dirs;
use std::fs;
use std::path::{Path, PathBuf};

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
