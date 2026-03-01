use colored::Colorize;
use std::ffi::OsStr;
use std::path::Path;

pub fn check_extension(source: &str, verbose: bool) -> bool {
    let extension = Path::new(source).extension().and_then(|s| s.to_str());
    match extension {
        Some(ext) if ext.to_lowercase() == "appimage" => {
            if verbose {
                println!("{} Valid format ({})", "INFO:".cyan().bold(), ext.yellow());
            }
            true
        }
        _ => {
            println!("{} {}", "ERROR:".red().bold(), "File must be an AppImage");
            false
        }
    }
}

pub fn move_appimage(source: &str, destination: &Path, file_name: &OsStr, verbose: bool) -> bool {
    if verbose {
        println!(
            "{} Moving {} to {}",
            "INFO:".cyan(),
            file_name.to_string_lossy().purple().bold(),
            destination.display().to_string().purple().bold()
        );
    }

    if let Err(e) = std::fs::rename(source, destination) {
        eprintln!(
            "{} Could not move file: {}. \n{} Try using a path on the same disk.",
            "ERROR:".red().bold(),
            e,
            "TIP:".yellow()
        );
        return false;
    }

    if verbose {
        println!("{} File moved successfully", "INFO:".cyan());
    }
    true
}
