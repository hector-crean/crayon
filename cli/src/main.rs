use clap::{Parser, Subcommand};
use cli::typegen::generate_typescript_bindings;
use color_eyre::eyre;

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Adds files to myapp
    Typegen { path: String },
}

fn main() -> eyre::Result<()> {
    color_eyre::install()?;

    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Commands::Typegen { path } => {
            generate_typescript_bindings(path)?;
        }
    }

    Ok(())
}
