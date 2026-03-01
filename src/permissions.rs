use colored::Colorize;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

pub fn set_executable_permissions(file_path: &Path) -> bool {
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
