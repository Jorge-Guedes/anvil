use colored::Colorize;
use std::ffi::OsString;
use std::path::Path;

pub fn get_file_name(source: &str) -> Option<OsString> {
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

pub fn get_app_name(args_name: &Option<String>, source_path: &Path) -> String {
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

pub fn capitalize_name(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + chars.as_str(),
    }
}
