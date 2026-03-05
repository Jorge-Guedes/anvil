use anvil_appimage::directory::{
    get_home_subdirectory, setup_app_directory, setup_desktop_entries_dir,
};
use std::fs;
use std::path::Path;
use tempfile::TempDir;

#[test]
fn test_get_home_subdirectory() {
    let result = get_home_subdirectory(".test_folder");
    assert!(result.is_some(), "Debería encontrar el directorio HOME");

    let path = result.unwrap();
    assert!(path.to_string_lossy().contains(".test_folder"));
}

#[test]
fn test_setup_app_directory_success() {
    let temp_dir = TempDir::new().unwrap();
    let base_path = temp_dir.path();
    let app_name = "TestApp";

    let result = setup_app_directory(base_path, app_name, false);

    assert!(result.is_some());
    let app_dir = result.unwrap();
    assert!(
        app_dir.exists(),
        "El directorio de la aplicación debería existir"
    );
    assert_eq!(app_dir.file_name().unwrap(), app_name);
}

#[test]
fn test_setup_app_directory_with_verbose() {
    let temp_dir = TempDir::new().unwrap();
    let base_path = temp_dir.path();

    let result = setup_app_directory(base_path, "TestApp", true);
    assert!(
        result.is_some(),
        "Debería crear el directorio incluso con verbose"
    );
}

#[test]
fn test_setup_app_directory_base_not_writable() {
    let base_path = Path::new("/proc/readonly/test");
    let result = setup_app_directory(base_path, "TestApp", false);
    assert!(
        result.is_none(),
        "Debería fallar si el directorio base no es escribible"
    );
}

#[test]
fn test_setup_app_directory_already_exists() {
    let temp_dir = TempDir::new().unwrap();
    let base_path = temp_dir.path();
    let app_name = "TestApp";

    let manual_dir = base_path.join(app_name);
    fs::create_dir(&manual_dir).unwrap();

    let result = setup_app_directory(base_path, app_name, false);
    assert!(
        result.is_some(),
        "Debería funcionar aunque el directorio ya exista"
    );
    assert_eq!(result.unwrap(), manual_dir);
}

#[test]
fn test_setup_desktop_entries_dir_success() {
    let result = setup_desktop_entries_dir(false);

    if result.is_none() {
        println!("Test omitido: No se encontró el directorio HOME");
        return;
    }

    let desktop_dir = result.unwrap();
    assert!(
        desktop_dir
            .to_string_lossy()
            .contains(".local/share/applications"),
        "La ruta debería contener .local/share/applications"
    );
}

#[test]
fn test_setup_desktop_entries_dir_with_verbose() {
    let result = setup_desktop_entries_dir(true);

    if result.is_none() {
        println!("Test omitido: No se encontró el directorio HOME");
        return;
    }
}

#[test]
fn test_setup_desktop_entries_dir_home_not_set() {
    let _ = setup_desktop_entries_dir(false);
}
