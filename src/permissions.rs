use colored::Colorize;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

pub fn set_executable_permissions(file_path: &Path, verbose: bool) -> bool {
    match fs::metadata(file_path) {
        Ok(metadata) => {
            let mut permissions = metadata.permissions();
            permissions.set_mode(0o755);
            match fs::set_permissions(file_path, permissions) {
                Ok(()) => {
                    if verbose {
                        println!("{} Execute permissions set", "INFO:".cyan());
                    }
                    true
                }
                Err(e) => {
                    eprintln!(
                        "{} {} {}",
                        "ERROR".red().bold(),
                        "Could not set permissions:",
                        e
                    );
                    false
                }
            }
        }
        Err(e) => {
            eprintln!(
                "{} {} {}",
                "ERROR".red().bold(),
                "Could not access file:",
                e
            );
            false
        }
    }
}
