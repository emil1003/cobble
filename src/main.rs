use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    #[arg(short, long)]
    verbose: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Compile a given assembly file into bytecode
    #[command(alias = "b")]
    Build(FilePaths),

    /// Run a given program through the interpreter
    #[command(alias = "r")]
    Run(FilePaths),
}

#[derive(Args)]
struct FilePaths {
    /// Input file path
    in_path: String,

    /// Output file path
    #[arg(short, long)]
    output: Option<String>,
}

fn main() {
    let _cli = Cli::parse();
}

#[test]
fn test_verify_cli() {
    use clap::CommandFactory;
    Cli::command().debug_assert();
}
