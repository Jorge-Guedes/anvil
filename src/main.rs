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

    let Some(destination_path) = get_destination_path(&args.destination) else {
        eprintln!(
            "{} {}",
            "Error".red().bold(),
            "No se pudo encontrar el HOME".red()
        );
        return;
    };

    if let Err(e) = std::fs::create_dir_all(&destination_path) {
        eprintln!(
            "{} {} {}",
            "ERROR:".red().bold(),
            "No se pudo crear la carpeta:".red(),
            e
        );
        return;
    }
    println!("{}", "Carpeta de destino lista".green().bold());

    if let Some(file_name) = Path::new(&args.source).file_name() {
        let extension = Path::new(&args.source).extension().and_then(|s| s.to_str());
        match extension {
            Some(ext) if ext.to_lowercase() == "appimage" => {
                println!(
                    "{} {} ({})",
                    "INFO:".cyan().bold(),
                    "Formato válido".cyan(),
                    ext.yellow()
                );
            }
            _ => {
                println!(
                    "{} {}",
                    "ERROR:".red().bold(),
                    "El archivo debe ser un AppImage".red()
                );
                return;
            }
        }

        let source_path = Path::new(&args.source);
        let app_folder_name = if let Some(ref name) = args.name {
            name.clone()
        } else {
            let Some(stem) = source_path.file_stem().and_then(|s| s.to_str()) else {
                eprintln!(
                    "{} {}",
                    "ERROR:".red().bold(),
                    "No se pudo extraer un nombre válido".red()
                );
                return;
            };
            stem.to_string()
        };

        let mut chars = app_folder_name.chars();
        let capitalized_name = match chars.next() {
            None => String::new(),
            Some(f) => f.to_uppercase().collect::<String>() + chars.as_str(),
        };

        let app_dir = destination_path.join(&capitalized_name);

        if let Err(e) = std::fs::create_dir_all(&app_dir) {
            eprintln!(
                "{} {} {}",
                "ERROR:".red().bold(),
                "No se puede crear la carpeta".red(),
                e
            );
            return;
        }

        let final_file_path = app_dir.join(file_name);

        println!(
            "Moviendo {} a su carpeta: {}",
            file_name.to_string_lossy().purple().bold(),
            final_file_path.display().to_string().purple().bold()
        );

        if let Err(e) = std::fs::rename(&args.source, &final_file_path) {
            eprintln!(
                "{} No se pudo mover el archivo: {}. \n{} intenta usar una ruta en el mismo disco.",
                "ERROR:".red().bold(),
                e,
                "TIP:".yellow()
            );
            return;
        }

        println!("{}", "¡Archivo movido correctamente!".green().bold());
    }
}

fn get_destination_path(folder_name: &str) -> Option<PathBuf> {
    match dirs::home_dir() {
        Some(path) => Some(path.join(folder_name)),
        None => None,
    }
}
