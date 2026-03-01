use std::fs;
use std::path::{Path, PathBuf};

pub struct DesktopEntry {
    pub name: String,
    pub exec: PathBuf,
    pub icon: PathBuf,
    pub categories: String,
    pub startup_wm_class: String,
}

impl DesktopEntry {
    pub fn generate_content(&self) -> String {
        format!(
            "[Desktop Entry]\n\
            Type=Application\n\
            Name={}\n\
            Exec={}\n\
            Icon={}\n\
            Categories={}\n\
            Terminal=false\n\
            StartupWMClass={}",
            self.name,
            self.exec.display(),
            self.icon.display(),
            self.categories,
            self.startup_wm_class
        )
    }

    pub fn write_to_file(&self, path: &Path) -> std::io::Result<()> {
        fs::write(path, self.generate_content())
    }
}
