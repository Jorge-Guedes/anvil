use anvil_appimage::permissions::set_executable_permissions;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use tempfile::TempDir;

#[test]
fn test_set_executable_permissions_success() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.sh");
    fs::write(&file_path, "#!/bin/bash\necho test").unwrap();

    let mut perms = fs::metadata(&file_path).unwrap().permissions();
    perms.set_mode(0o644);
    fs::set_permissions(&file_path, perms).unwrap();

    let result = set_executable_permissions(&file_path, false);
    assert!(result, "Debería establecer permisos correctamente");

    let metadata = fs::metadata(&file_path).unwrap();
    let perms = metadata.permissions();
    assert_eq!(perms.mode() & 0o777, 0o755, "Los permisos deberían ser 755");
}

#[test]
fn test_set_executable_permissions_with_verbose() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.sh");
    fs::write(&file_path, "#!/bin/bash\necho test").unwrap();

    let result = set_executable_permissions(&file_path, true);
    assert!(result, "Debería funcionar con verbose true");

    let metadata = fs::metadata(&file_path).unwrap();
    let perms = metadata.permissions();
    assert_eq!(perms.mode() & 0o777, 0o755);
}

#[test]
fn test_set_executable_permissions_file_not_exists() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("no-existe.sh");

    let result = set_executable_permissions(&file_path, false);
    assert!(!result, "Debería fallar si el archivo no existe");
}

#[test]
fn test_set_executable_permissions_already_executable() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.sh");
    fs::write(&file_path, "#!/bin/bash\necho test").unwrap();

    // Establecer permisos 755 inicialmente
    let mut perms = fs::metadata(&file_path).unwrap().permissions();
    perms.set_mode(0o755);
    fs::set_permissions(&file_path, perms).unwrap();

    let result = set_executable_permissions(&file_path, false);
    assert!(result, "Debería funcionar aunque ya sea ejecutable");

    let metadata = fs::metadata(&file_path).unwrap();
    let perms = metadata.permissions();
    assert_eq!(perms.mode() & 0o777, 0o755);
}

#[test]
fn test_set_executable_permissions_symlink() {
    let temp_dir = TempDir::new().unwrap();
    let real_file = temp_dir.path().join("real.sh");
    fs::write(&real_file, "#!/bin/bash\necho test").unwrap();

    let symlink = temp_dir.path().join("link.sh");
    #[cfg(unix)]
    std::os::unix::fs::symlink(&real_file, &symlink).unwrap();

    let result = set_executable_permissions(&symlink, false);
    assert!(result, "Debería funcionar con enlaces simbólicos");

    let metadata = fs::metadata(&real_file).unwrap();
    let perms = metadata.permissions();
    assert_eq!(perms.mode() & 0o777, 0o755);
}

#[test]
fn test_set_executable_permissions_directory() {
    let temp_dir = TempDir::new().unwrap();
    let dir_path = temp_dir.path().join("test_dir");
    fs::create_dir(&dir_path).unwrap();

    let result = set_executable_permissions(&dir_path, false);
    assert!(result, "Debería funcionar también con directorios");

    let metadata = fs::metadata(&dir_path).unwrap();
    let perms = metadata.permissions();
    assert_eq!(
        perms.mode() & 0o777,
        0o755,
        "Los directorios también pueden ser 755"
    );
}
