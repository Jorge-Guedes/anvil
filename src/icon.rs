use colored::Colorize;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

pub fn copy_icon(icon_source: &str, app_dir: &Path, verbose: bool) -> Option<PathBuf> {
    let icon_path = Path::new(icon_source);
    let icon_name = match icon_path.file_name() {
        Some(name) => name,
        None => {
            eprintln!(
                "{} {}",
                "ERROR:".red().bold(),
                "Could not get icon filename"
            );
            return None;
        }
    };

    let icon_dest = app_dir.join(icon_name);

    if verbose {
        println!(
            "{} Copying icon from {} to {}",
            "INFO:".cyan(),
            icon_source,
            icon_dest.display()
        );
    }

    if let Err(e) = std::fs::copy(icon_source, &icon_dest) {
        eprintln!("{} Could not copy icon: {}", "ERROR:".red().bold(), e);
        return None;
    }

    if verbose {
        println!("{} Icon copied successfully", "INFO:".cyan());
    }
    Some(icon_dest)
}

pub fn find_icons_in_dir(dir: &Path) -> Vec<PathBuf> {
    let mut icons = Vec::new();

    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();

                if path.is_dir() {
                    let sub_icons = find_icons_in_dir(&path);
                    icons.extend(sub_icons);
                } else if path.is_file() {
                    if let Some(ext) = path.extension() {
                        if let Some(ext_str) = ext.to_str() {
                            match ext_str.to_lowercase().as_str() {
                                "png" | "svg" | "xpm" | "ico" => {
                                    icons.push(path);
                                }
                                _ => {}
                            }
                        }
                    }
                }
            }
        }
    }

    icons
}

pub fn select_best_icon(icons: Vec<PathBuf>) -> Option<PathBuf> {
    let mut png_icons = Vec::new();
    let mut other_icons = Vec::new();

    for icon in icons {
        if let Some(ext) = icon.extension() {
            if let Some(ext_str) = ext.to_str() {
                if ext_str.to_lowercase() == "png" {
                    png_icons.push(icon);
                } else {
                    other_icons.push(icon);
                }
            } else {
                other_icons.push(icon);
            }
        } else {
            other_icons.push(icon);
        }
    }

    fn heaviest_icon(icons: Vec<PathBuf>) -> Option<PathBuf> {
        let mut best = None;
        let mut max_size = 0;

        for icon in icons {
            if let Ok(metadata) = fs::metadata(&icon) {
                let size = metadata.len();
                if size > max_size {
                    max_size = size;
                    best = Some(icon);
                }
            }
        }
        best
    }

    if !png_icons.is_empty() {
        if let Some(best_png) = heaviest_icon(png_icons) {
            return Some(best_png);
        }
    }

    if !other_icons.is_empty() {
        if let Some(best_other) = heaviest_icon(other_icons) {
            return Some(best_other);
        }
    }

    None
}

pub fn extract_icon_from_appimage(
    appimage_path: &Path,
    app_dir: &Path,
    verbose: bool,
) -> Option<PathBuf> {
    if verbose {
        println!("{} Extracting AppImage contents...", "INFO:".cyan());
    }

    match Command::new(appimage_path)
        .arg("--appimage-extract")
        .status()
    {
        Ok(status) if status.success() => {
            let icons = find_icons_in_dir(Path::new("squashfs-root"));

            if icons.is_empty() {
                eprintln!("{} No icons found", "WARN:".yellow().bold());
                let _ = fs::remove_dir_all("squashfs-root");
                return None;
            }

            if verbose {
                println!("{} Found {} icons", "INFO:".cyan(), icons.len());
            }

            if let Some(best_icon) = select_best_icon(icons) {
                if verbose {
                    println!("{} Best icon: {:?}", "INFO:".cyan(), best_icon);
                }

                if let Some(icon_str) = best_icon.to_str() {
                    let result = copy_icon(icon_str, app_dir, verbose);
                    let _ = fs::remove_dir_all("squashfs-root");
                    return result;
                } else {
                    eprintln!("{} Invalid icon path", "WARN:".yellow().bold());
                }
            }

            let _ = fs::remove_dir_all("squashfs-root");
            None
        }
        Ok(status) => {
            eprintln!(
                "{} {} {}",
                "ERROR:".red().bold(),
                "Command failed with code:",
                status
            );
            None
        }
        Err(e) => {
            eprintln!("{} Could not execute: {}", "ERROR:".red().bold(), e);
            None
        }
    }
}
