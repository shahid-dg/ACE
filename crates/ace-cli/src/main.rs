use clap::Parser;

mod commands;

#[derive(Parser, Debug)]
#[command(
    name = "ace",
    version,
    about = "Annotation quality assurance, consensus analysis, and annotator reliability"
)]
struct Cli {}

fn main() {
    let _ = Cli::parse();
}
