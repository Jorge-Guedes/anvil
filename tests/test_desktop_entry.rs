use anvil_appimage::desktop_entry::DesktopEntry;
use std::fs;
use std::path::{Path, PathBuf};
use tempfile::TempDir;

#[test]
fn test_desktop_entry_generate_content() {
    let entry = DesktopEntry {
        name: "TestApp".to_string(),
        exec: PathBuf::from("/home/user/Applications/TestApp/test.AppImage"),
        icon: PathBuf::from("/home/user/Applications/TestApp/test.png"),
        categories: "Utility;Development;".to_string(),
        startup_wm_class: "TestApp".to_string(),
    };

    let content = entry.generate_content();

    assert!(content.contains("[Desktop Entry]"));
    assert!(content.contains("Type=Application"));
    assert!(content.contains("Name=TestApp"));
    assert!(content.contains("Exec=/home/user/Applications/TestApp/test.AppImage"));
    assert!(content.contains("Icon=/home/user/Applications/TestApp/test.png"));
    assert!(content.contains("Categories=Utility;Development;"));
    assert!(content.contains("Terminal=false"));
    assert!(content.contains("StartupWMClass=TestApp"));
}

#[test]
fn test_desktop_entry_generate_content_with_empty_icon() {
    let entry = DesktopEntry {
        name: "TestApp".to_string(),
        exec: PathBuf::from("/home/user/Applications/TestApp/test.AppImage"),
        icon: PathBuf::new(), // Icono vacío
        categories: "Utility".to_string(),
        startup_wm_class: "TestApp".to_string(),
    };

    let content = entry.generate_content();

    assert!(content.contains("Icon="));
    assert!(content.contains("Categories=Utility"));
}

#[test]
fn test_desktop_entry_write_to_file_success() {
    let temp_dir = TempDir::new().unwrap();
    let desktop_path = temp_dir.path().join("test.desktop");

    let entry = DesktopEntry {
        name: "TestApp".to_string(),
        exec: PathBuf::from("/home/user/Applications/TestApp/test.AppImage"),
        icon: PathBuf::from("/home/user/Applications/TestApp/test.png"),
        categories: "Utility".to_string(),
        startup_wm_class: "TestApp".to_string(),
    };

    let result = entry.write_to_file(&desktop_path);
    assert!(result.is_ok(), "Debería escribir el archivo correctamente");

    assert!(desktop_path.exists(), "El archivo .desktop debería existir");

    let content = fs::read_to_string(&desktop_path).unwrap();
    assert!(content.contains("Name=TestApp"));
    assert!(content.contains("Exec=/home/user/Applications/TestApp/test.AppImage"));
}

#[test]
fn test_desktop_entry_write_to_file_invalid_path() {
    let entry = DesktopEntry {
        name: "TestApp".to_string(),
        exec: PathBuf::from("/home/user/Applications/TestApp/test.AppImage"),
        icon: PathBuf::from("/home/user/Applications/TestApp/test.png"),
        categories: "Utility".to_string(),
        startup_wm_class: "TestApp".to_string(),
    };

    let invalid_path = Path::new("/proc/readonly/no-existe/test.desktop");

    let result = entry.write_to_file(invalid_path);
    assert!(result.is_err(), "Debería fallar con ruta inválida");
}

#[test]
fn test_desktop_entry_with_special_characters() {
    let entry = DesktopEntry {
        name: "App with spaces and ñandú".to_string(),
        exec: PathBuf::from("/home/user/Applications/My App/test.AppImage"),
        icon: PathBuf::from("/home/user/Applications/My App/icon.png"),
        categories: "Utility;Development;".to_string(),
        startup_wm_class: "AppWithSpaces".to_string(),
    };

    let content = entry.generate_content();

    assert!(content.contains("Name=App with spaces and ñandú"));
    assert!(content.contains("Exec=/home/user/Applications/My App/test.AppImage"));
    assert!(content.contains("StartupWMClass=AppWithSpaces"));
}

#[test]
fn test_desktop_entry_multiple_categories() {
    let entry = DesktopEntry {
        name: "TestApp".to_string(),
        exec: PathBuf::from("/home/user/Applications/TestApp/test.AppImage"),
        icon: PathBuf::from("/home/user/Applications/TestApp/test.png"),
        categories: "Graphics;Photography;Utility;".to_string(),
        startup_wm_class: "TestApp".to_string(),
    };

    let content = entry.generate_content();
    assert!(content.contains("Categories=Graphics;Photography;Utility;"));
}

#[test]
fn test_desktop_entry_round_trip() {
    let temp_dir = TempDir::new().unwrap();
    let desktop_path = temp_dir.path().join("test.desktop");

    let original_entry = DesktopEntry {
        name: "TestApp".to_string(),
        exec: PathBuf::from("/home/user/Applications/TestApp/test.AppImage"),
        icon: PathBuf::from("/home/user/Applications/TestApp/test.png"),
        categories: "Utility".to_string(),
        startup_wm_class: "TestApp".to_string(),
    };

    original_entry.write_to_file(&desktop_path).unwrap();

    let content = fs::read_to_string(&desktop_path).unwrap();

    assert!(content.contains(&original_entry.name));
    assert!(content.contains(&original_entry.exec.display().to_string()));
    assert!(content.contains(&original_entry.icon.display().to_string()));
    assert!(content.contains(&original_entry.categories));
    assert!(content.contains(&original_entry.startup_wm_class));
}
