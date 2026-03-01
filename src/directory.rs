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

pub fn setup_app_directory(base_path: &Path, app_name: &str, verbose: bool) -> Option<PathBuf> {
    if let Err(e) = std::fs::create_dir_all(base_path) {
        eprintln!(
            "{} {} {}",
            "ERROR:".red().bold(),
            "Could not create base folder:",
            e
        );
        return None;
    }

    if verbose {
        println!("{} Base folder ready", "INFO:".cyan());
    }

    let app_dir = base_path.join(app_name);

    if let Err(e) = std::fs::create_dir_all(&app_dir) {
        eprintln!(
            "{} {} {}",
            "ERROR:".red().bold(),
            "Could not create app folder:",
            e
        );
        return None;
    }

    if verbose {
        println!(
            "{} App folder created: {}",
            "INFO:".cyan(),
            app_dir.display()
        );
    }

    Some(app_dir)
}

pub fn setup_desktop_entries_dir(verbose: bool) -> Option<PathBuf> {
    let home = match dirs::home_dir() {
        Some(path) => path,
        None => {
            eprintln!(
                "{} {}",
                "ERROR:".red().bold(),
                "Could not find HOME directory"
            );
            return None;
        }
    };

    let desktop_dir = home.join(".local/share/applications");

    if let Err(e) = fs::create_dir_all(&desktop_dir) {
        eprintln!(
            "{} Could not create desktop entries directory: {}",
            "ERROR:".red().bold(),
            e
        );
        return None;
    }

    if verbose {
        println!("{} Desktop entries directory ready", "INFO:".cyan());
    }
    Some(desktop_dir)
}
