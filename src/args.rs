use clap::Parser;

#[derive(Parser)]
#[command(
    author = "Jorge Guedes",
    version = "1.0.0",
    about = "Anvil - AppImage installer for Linux",
    long_about = "Anvil automatically installs AppImages by moving them to a dedicated folder, extracting icons, and creating desktop entries for seamless integration with your Linux desktop."
)]
pub struct Args {
    #[arg(short, long, help = "Path to the AppImage file")]
    pub source: String,

    #[arg(
        short,
        long,
        default_value = ".Applications_AppImage",
        help = "Destination directory under HOME (default: .Applications_AppImage)"
    )]
    pub destination: String,

    #[arg(short, long, help = "Custom name for the application")]
    pub name: Option<String>,

    #[arg(short, long, help = "Path to a custom icon file")]
    pub icon: Option<String>,

    #[arg(
        short,
        long,
        default_value = "Utility",
        help = "Desktop entry categories (e.g. 'Utility;Development;')"
    )]
    pub categories: String,

    #[arg(short, long, help = "Show detailed progress messages")]
    pub verbose: bool,
}
