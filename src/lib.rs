pub mod appimage;
pub mod args;
pub mod desktop_entry;
pub mod directory;
pub mod icon;
pub mod permissions;
pub mod utils;

pub use appimage::{check_extension, move_appimage};
pub use directory::{get_home_subdirectory, setup_app_directory, setup_desktop_entries_dir};
pub use icon::{copy_icon, extract_icon_from_appimage, find_icons_in_dir, select_best_icon};
pub use permissions::set_executable_permissions;
pub use utils::{capitalize_name, get_app_name, get_file_name};
