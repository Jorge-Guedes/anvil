use anvil_appimage::utils::{capitalize_name, get_app_name, get_file_name};
use std::path::Path;

#[test]
fn test_capitalize_name() {
    assert_eq!(capitalize_name("flameshot"), "Flameshot");
    assert_eq!(capitalize_name("nuncius"), "Nuncius");
    assert_eq!(capitalize_name(""), "");
    assert_eq!(capitalize_name("miApp"), "MiApp");
}

#[test]
fn test_get_app_name_with_name() {
    let path = Path::new("/ruta/test.AppImage");
    let name = Some("MiApp".to_string());
    assert_eq!(get_app_name(&name, path), "MiApp");
}

#[test]
fn test_get_app_name_from_filename() {
    let path = Path::new("/ruta/test.AppImage");
    assert_eq!(get_app_name(&None, path), "test");
}

#[test]
fn test_get_app_name_fallback() {
    let path = Path::new("/");
    assert_eq!(get_app_name(&None, path), "Application");
}

#[test]
fn test_get_file_name_valid() {
    let source = "/ruta/mi-app.AppImage";
    let result = get_file_name(source);
    assert!(result.is_some());
    assert_eq!(result.unwrap().to_string_lossy(), "mi-app.AppImage");
}

#[test]
fn test_get_file_name_invalid() {
    let source = "/";
    let result = get_file_name(source);
    assert!(result.is_none());
}

#[test]
fn test_get_file_name_with_spaces() {
    let source = "/ruta/mi app.AppImage";
    let result = get_file_name(source);
    assert!(result.is_some());
    assert_eq!(result.unwrap().to_string_lossy(), "mi app.AppImage");
}
