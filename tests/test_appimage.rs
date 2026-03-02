use anvil::appimage::{check_extension, move_appimage};
use std::ffi::OsStr;
use std::fs;
use std::path::Path;
use tempfile::TempDir;

#[test]
fn test_check_extension_valid() {
    // Con verbose false para no imprimir durante tests
    assert!(check_extension("test.AppImage", false));
    assert!(check_extension("test.appimage", false));
    assert!(check_extension("test.AppIMAGE", false));
    assert!(check_extension("test.AppImage", true)); // Con verbose
}

#[test]
fn test_check_extension_invalid() {
    assert!(!check_extension("test.txt", false));
    assert!(!check_extension("test.pdf", false));
    assert!(!check_extension("test.png", false));
    assert!(!check_extension("test", false));
}

#[test]
fn test_check_extension_no_extension() {
    assert!(!check_extension("test", false));
    assert!(!check_extension("", false));
    assert!(!check_extension(".", false));
}

#[test]
fn test_move_appimage_success() {
    let temp_dir = TempDir::new().unwrap();
    let source_path = temp_dir.path().join("test.AppImage");
    let content = "contenido de prueba del AppImage";

    fs::write(&source_path, content).unwrap();

    let dest_dir = TempDir::new().unwrap();
    let dest_path = dest_dir.path().join("test.AppImage");

    let result = move_appimage(
        source_path.to_str().unwrap(),
        &dest_path,
        OsStr::new("test.AppImage"),
        false,
    );

    assert!(result, "El movimiento debería ser exitoso");
    assert!(
        !source_path.exists(),
        "El archivo origen debería haber desaparecido"
    );
    assert!(dest_path.exists(), "El archivo destino debería existir");

    let moved_content = fs::read_to_string(dest_path).unwrap();
    assert_eq!(moved_content, content, "El contenido debería ser el mismo");
}

#[test]
fn test_move_appimage_with_verbose() {
    let temp_dir = TempDir::new().unwrap();
    let source_path = temp_dir.path().join("test.AppImage");
    fs::write(&source_path, "contenido").unwrap();

    let dest_dir = TempDir::new().unwrap();
    let dest_path = dest_dir.path().join("test.AppImage");

    let result = move_appimage(
        source_path.to_str().unwrap(),
        &dest_path,
        OsStr::new("test.AppImage"),
        true, // verbose true
    );

    assert!(result);
    assert!(!source_path.exists());
    assert!(dest_path.exists());
}

#[test]
fn test_move_appimage_source_not_exists() {
    let temp_dir = TempDir::new().unwrap();
    let source_path = temp_dir.path().join("no-existe.AppImage");
    let dest_dir = TempDir::new().unwrap();
    let dest_path = dest_dir.path().join("test.AppImage");

    let result = move_appimage(
        source_path.to_str().unwrap(),
        &dest_path,
        OsStr::new("test.AppImage"),
        false,
    );

    assert!(!result, "Debería fallar si el origen no existe");
    assert!(!dest_path.exists(), "No debería crearse el destino");
}

#[test]
fn test_move_appimage_dest_parent_not_exists() {
    let temp_dir = TempDir::new().unwrap();
    let source_path = temp_dir.path().join("test.AppImage");
    fs::write(&source_path, "contenido").unwrap();

    let dest_path = Path::new("/tmp/anvil-test-dir-no-existe/test.AppImage");

    let result = move_appimage(
        source_path.to_str().unwrap(),
        dest_path,
        OsStr::new("test.AppImage"),
        false,
    );

    assert!(!result, "Debería fallar si el directorio destino no existe");
    assert!(source_path.exists(), "El origen debería permanecer intacto");
}

#[test]
fn test_move_appimage_empty_source() {
    let dest_dir = TempDir::new().unwrap();
    let dest_path = dest_dir.path().join("test.AppImage");

    let result = move_appimage("", &dest_path, OsStr::new("test.AppImage"), false);

    assert!(!result, "Debería fallar con source vacío");
}
