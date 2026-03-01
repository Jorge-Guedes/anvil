use clap::Parser;
use colored::Colorize;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

mod args;
use args::Args;
mod utils;
use utils::{capitalize_name, get_app_name, get_file_name};
mod directory;
use directory::{get_home_subdirectory, setup_app_directory, setup_desktop_entries_dir};
mod permissions;
use permissions::set_executable_permissions;
mod appimage;
use appimage::{check_extension, move_appimage};
mod icon;
use icon::{copy_icon, extract_icon_from_appimage};

struct DesktopEntry {
    name: String,
    exec: PathBuf,
    icon: PathBuf,
    categories: String,
    startup_wm_class: String,
}
impl DesktopEntry {
    fn generate_content(&self) -> String {
        format!(
            "[Desktop Entry]\n\
            Type=Application\n\
            Name={}\n\
            Exec={}\n\
            Icon={}\n\
            Categories={}\n\
            Terminal=false\n\
            StartupWMClass={}",
            self.name,
            self.exec.display(),
            self.icon.display(),
            self.categories,
            self.startup_wm_class
        )
    }

    fn write_to_file(&self, path: &Path) -> std::io::Result<()> {
        fs::write(path, self.generate_content())
    }
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

    let icon_path = if let Some(icon_source) = &args.icon {
        match copy_icon(icon_source, &app_dir) {
            Some(path) => {
                println!("{}", "Icono copiado correctamente".green().bold());
                path
            }
            None => return,
        }
    } else {
        match extract_icon_from_appimage(&final_file_path, &app_dir) {
            Some(path) => {
                println!("{}", "Icono extraído correctamente".green().bold());
                path
            }
            None => {
                eprintln!(
                    "{} No se pudo obtener icono, se continuará sin él",
                    "WARN:".yellow()
                );
                PathBuf::new()
            }
        }
    };

    let Some(desktop_entries_dir) = setup_desktop_entries_dir() else {
        return;
    };

    let desktop_entry = DesktopEntry {
        name: capitalized_name.clone(),
        exec: final_file_path,
        icon: icon_path,
        categories: args.categories,
        startup_wm_class: capitalized_name.clone(),
    };

    let desktop_file_name = format!("{}.desktop", capitalized_name);
    let desktop_file_path = desktop_entries_dir.join(desktop_file_name);

    match desktop_entry.write_to_file(&desktop_file_path) {
        Ok(()) => {
            println!(
                "{} {}",
                "Acceso directo creado en".green().bold(),
                desktop_file_path.display().to_string().green().bold()
            );
        }
        Err(e) => {
            eprintln!(
                "{} No se pudo crear el acceso directo: {}",
                "ERROR:".red().bold(),
                e
            );
            return;
        }
    }

    let output = Command::new("update-desktop-database")
        .arg(&desktop_entries_dir)
        .output();

    match output {
        Ok(output) if output.status.success() => {
            println!(
                "{}",
                "Base de datos de aplicaciones actualizada".green().bold()
            );
        }
        Ok(output) => {
            eprintln!(
                "{} No se pudo actualizar la base de datos (código: {})",
                "WARN:".yellow(),
                output.status
            );
            println!(
                "{} Para que la aplicación aparezca en el menú, ejecuta: update-desktop-database {}",
                "TIP:".yellow(),
                desktop_entries_dir.display()
            );
        }
        Err(e) => {
            eprintln!(
                "{} No se pudo ejecutar update-desktop-database: {}",
                "WARN:".yellow(),
                e
            );
            println!(
                "{} Para que la aplicación aparezca en el menú, ejecuta: update-desktop-database {}",
                "TIP:".yellow(),
                desktop_entries_dir.display()
            );
        }
    }

    println!(
        "{} {}",
        "¡instalado correctamente!".bright_green().bold(),
        capitalized_name.bright_cyan().bold()
    );
}
