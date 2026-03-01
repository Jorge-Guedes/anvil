use clap::Parser;
use colored::Colorize;
use std::path::{Path, PathBuf};
use std::process::Command;

mod args;
use args::Args;
mod desktop_entry;
use desktop_entry::DesktopEntry;
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

fn main() {
    let args = Args::parse();

    if !Path::new(&args.source).exists() {
        eprintln!(
            "{} {} {} {}",
            "ERROR:".red().bold(),
            "File",
            args.source.purple().bold(),
            "not found"
        );
        return;
    }

    let Some(destination_path) = get_home_subdirectory(&args.destination) else {
        eprintln!(
            "{} {}",
            "ERROR:".red().bold(),
            "Could not find HOME directory"
        );
        return;
    };

    let Some(file_name) = get_file_name(&args.source) else {
        return;
    };

    if !check_extension(&args.source, args.verbose) {
        return;
    }

    let source_path = Path::new(&args.source);
    let app_folder_name = get_app_name(&args.name, source_path);

    let capitalized_name = capitalize_name(&app_folder_name);

    let Some(app_dir) = setup_app_directory(&destination_path, &capitalized_name, args.verbose)
    else {
        return;
    };

    let final_file_path = app_dir.join(&file_name);

    if !move_appimage(&args.source, &final_file_path, &file_name, args.verbose) {
        return;
    }

    if !set_executable_permissions(&final_file_path, args.verbose) {
        return;
    }

    let icon_path = if let Some(icon_source) = &args.icon {
        match copy_icon(icon_source, &app_dir, args.verbose) {
            Some(path) => {
                if args.verbose {
                    println!("{} Icon copied successfully", "INFO:".cyan());
                }
                path
            }
            None => return,
        }
    } else {
        match extract_icon_from_appimage(&final_file_path, &app_dir, args.verbose) {
            Some(path) => {
                if args.verbose {
                    println!("{} Icon extracted successfully", "INFO:".cyan());
                }
                path
            }
            None => {
                eprintln!(
                    "{} Could not obtain icon, continuing without it",
                    "WARN:".yellow().bold()
                );

                PathBuf::new()
            }
        }
    };

    let Some(desktop_entries_dir) = setup_desktop_entries_dir(args.verbose) else {
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
            if args.verbose {
                println!(
                    "{} Desktop entry created at {}",
                    "INFO:".cyan(),
                    desktop_file_path.display()
                );
            }
        }
        Err(e) => {
            eprintln!(
                "{} Could not create desktop entry: {}",
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
            if args.verbose {
                println!(
                    "{} Application database updated successfully",
                    "INFO:".cyan()
                );
            }
        }
        Ok(output) => {
            eprintln!(
                "{} Could not update application database (code: {})",
                "WARN:".yellow(),
                output.status
            );
            println!(
                "{} If the app doesn't appear in the menu, run: update-desktop-database {}",
                "TIP:".yellow(),
                desktop_entries_dir.display()
            );
        }
        Err(e) => {
            eprintln!(
                "{} Could not run update-desktop-database: {}",
                "WARN:".yellow(),
                e
            );
            println!(
                "{} If the app doesn't appear in the menu, run: update-desktop-database {}",
                "TIP:".yellow(),
                desktop_entries_dir.display()
            );
        }
    }

    println!(
        "{} {} installed successfully",
        "SUCCESS:".green().bold(),
        capitalized_name.bright_cyan().bold()
    );
}
