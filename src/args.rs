use clap::Parser;

#[derive(Parser)]
#[command(author, version, about)]
pub struct Args {
    #[arg(short, long)]
    pub source: String,

    #[arg(short, long, default_value = ".Applications")]
    pub destination: String,

    #[arg(short, long)]
    pub name: Option<String>,

    #[arg(short, long)]
    pub icon: Option<String>,

    #[arg(short, long, default_value = "Utility")]
    pub categories: String,

    #[arg(short, long)]
    pub verbose: bool,
}
