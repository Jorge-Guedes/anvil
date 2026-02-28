# 🛠 Próximos Pasos para Anvil

## Prioridad Alta
- [ ] **Permisos de Ejecución**: Implementar `std::os::unix::fs::PermissionsExt` para aplicar `chmod +x` (0o755) al archivo final.
- [ ] **Generación de .desktop**: Crear el archivo lanzador en `~/.local/share/applications/` usando el nombre capitalizado.
- [ ] **Rutas Dinámicas**: Asegurar que el campo `Exec=` en el `.desktop` apunte a la ruta absoluta de la AppImage.

## Mejoras de UX
- [ ] **Iconos**: Lógica para mover el archivo de icono (si se provee) a la carpeta de la aplicación.
- [ ] **Categorías**: Usar el argumento `--categories` para rellenar el campo `Categories=` del lanzador.
- [ ] **Limpieza de Código**: Sustituir las repeticiones de `Path::new(&args.source)` por la variable `source_path`.

## Seguridad
- [ ] **Validación de Sobrescritura**: Comprobar si `final_file_path` ya existe antes de hacer el `rename`.

---

"Recuerda que estoy desarrollando 'Anvil', un instalador de AppImages en Rust para Linux, y que nos quedamos pendientes de implementar los permisos de ejecución y el generador de archivos .desktop."

Si me dices eso, guardaré el contexto del proyecto. Mañana, cuando abras un chat nuevo, solo tendrás que decirme: "Hola, vamos a seguir con Anvil" y yo ya sabré de qué herramientas estamos hablando.

Lo que me tendrías que pasar mañana:
Para que yo trabaje con el código real, mañana lo ideal es que me pegues:

Tu main.rs actual (el que tienes ahora).

El primer punto del TODO.md que quieras atacar.