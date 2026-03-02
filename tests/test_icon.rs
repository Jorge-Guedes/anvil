use anvil::icon::{copy_icon, find_icons_in_dir, select_best_icon};
use std::fs;
use tempfile::TempDir;

#[test]
fn test_copy_icon_success() {
    let temp_dir = TempDir::new().unwrap();
    let app_dir = temp_dir.path();

    let icon_source = temp_dir.path().join("test-icon.png");
    fs::write(&icon_source, "contenido del icono").unwrap();

    let result = copy_icon(icon_source.to_str().unwrap(), app_dir, false);

    assert!(result.is_some(), "Debería copiar el icono correctamente");
    let icon_dest = result.unwrap();
    assert!(icon_dest.exists(), "El icono debería existir en el destino");
    assert_eq!(icon_dest.file_name().unwrap(), "test-icon.png");
}

#[test]
fn test_copy_icon_with_verbose() {
    let temp_dir = TempDir::new().unwrap();
    let app_dir = temp_dir.path();

    let icon_source = temp_dir.path().join("test-icon.png");
    fs::write(&icon_source, "contenido").unwrap();

    let result = copy_icon(icon_source.to_str().unwrap(), app_dir, true);
    assert!(result.is_some(), "Debería funcionar con verbose");
}

#[test]
fn test_copy_icon_invalid_source() {
    let temp_dir = TempDir::new().unwrap();
    let app_dir = temp_dir.path();

    let result = copy_icon("/ruta/no/existe/icono.png", app_dir, false);
    assert!(result.is_none(), "Debería fallar si el origen no existe");
}

#[test]
fn test_copy_icon_no_filename() {
    let temp_dir = TempDir::new().unwrap();
    let app_dir = temp_dir.path();

    let result = copy_icon("/ruta/", app_dir, false);
    assert!(
        result.is_none(),
        "Debería fallar si no hay nombre de archivo"
    );
}

#[test]
fn test_find_icons_in_dir_empty() {
    let temp_dir = TempDir::new().unwrap();

    let icons = find_icons_in_dir(temp_dir.path());
    assert!(icons.is_empty(), "Directorio vacío no debería tener iconos");
}

#[test]
fn test_find_icons_in_dir_with_icons() {
    let temp_dir = TempDir::new().unwrap();
    let dir_path = temp_dir.path();

    fs::write(dir_path.join("icon1.png"), "png").unwrap();
    fs::write(dir_path.join("icon2.svg"), "svg").unwrap();
    fs::write(dir_path.join("icon3.xpm"), "xpm").unwrap();
    fs::write(dir_path.join("icon4.ico"), "ico").unwrap();
    fs::write(dir_path.join("not-an-icon.txt"), "texto").unwrap();

    let sub_dir = dir_path.join("subdir");
    fs::create_dir(&sub_dir).unwrap();
    fs::write(sub_dir.join("icon5.png"), "png").unwrap();
    fs::write(sub_dir.join("icon6.svg"), "svg").unwrap();

    let icons = find_icons_in_dir(dir_path);

    assert_eq!(icons.len(), 6, "Debería encontrar 6 iconos");

    assert!(icons.iter().any(|p| p.ends_with("icon1.png")));
    assert!(icons.iter().any(|p| p.ends_with("icon5.png")));

    assert!(icons.iter().any(|p| p.ends_with("icon2.svg")));
    assert!(icons.iter().any(|p| p.ends_with("icon6.svg")));

    assert!(icons.iter().any(|p| p.ends_with("icon3.xpm")));
    assert!(icons.iter().any(|p| p.ends_with("icon4.ico")));

    assert!(!icons.iter().any(|p| p.ends_with("not-an-icon.txt")));
}

#[test]
fn test_select_best_icon_prefer_png() {
    let temp_dir = TempDir::new().unwrap();

    let png_path = temp_dir.path().join("icon.png");
    let svg_path = temp_dir.path().join("icon.svg");
    let xpm_path = temp_dir.path().join("icon.xpm");

    fs::write(&png_path, "a".repeat(100)).unwrap();
    fs::write(&svg_path, "a".repeat(200)).unwrap();
    fs::write(&xpm_path, "a".repeat(50)).unwrap();

    let icons = vec![svg_path.clone(), png_path.clone(), xpm_path.clone()];
    let best = select_best_icon(icons);

    assert!(best.is_some());
    assert!(best.unwrap().ends_with("icon.png"));
}

#[test]
fn test_select_best_icon_by_weight() {
    let temp_dir = TempDir::new().unwrap();

    let small = temp_dir.path().join("16x16.png");
    let medium = temp_dir.path().join("32x32.png");
    let large = temp_dir.path().join("128x128.png");

    fs::write(&small, "a".repeat(500)).unwrap();
    fs::write(&medium, "a".repeat(2000)).unwrap();
    fs::write(&large, "a".repeat(5000)).unwrap();

    let icons = vec![small, medium.clone(), large];
    let best = select_best_icon(icons);

    assert!(best.is_some());
    assert!(
        best.unwrap().ends_with("128x128.png"),
        "Debería elegir el más pesado"
    );
}

#[test]
fn test_select_best_icon_empty() {
    let icons = Vec::new();
    let best = select_best_icon(icons);
    assert!(best.is_none(), "Lista vacía debería devolver None");
}

#[test]
fn test_select_best_icon_only_other_formats() {
    let temp_dir = TempDir::new().unwrap();

    let svg1 = temp_dir.path().join("icon1.svg");
    let svg2 = temp_dir.path().join("icon2.svg");
    let xpm = temp_dir.path().join("icon.xpm");

    fs::write(&svg1, "a".repeat(100)).unwrap();
    fs::write(&svg2, "a".repeat(300)).unwrap();
    fs::write(&xpm, "a".repeat(200)).unwrap();

    let icons = vec![xpm, svg1, svg2.clone()];
    let best = select_best_icon(icons);

    assert!(best.is_some());
    assert!(
        best.unwrap().ends_with("icon2.svg"),
        "Debería elegir el SVG más pesado"
    );
}
