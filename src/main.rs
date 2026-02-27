use clap::Parser;
use colored::Colorize;
use dirs;
use std::path::Path;

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(short, long)]
    source: String,

    #[arg(short, long)]
    name: String,

    #[arg(short, long)]
    icon: Option<String>,

    #[arg(short, long, default_value = ".Applications")]
    destination: String,

    #[arg(short, long, default_value = "Utility")]
    categories: String,
}

fn main() {
    let args = Args::parse();

    if !Path::new(&args.source).exists() {
        eprintln!(
            "{} {} {} {}",
            "ERROR:".red().bold(),
            "Archivo".red(),
            args.source.purple().bold(),
            "no encontrado".red()
        );
        return;
    }
}
