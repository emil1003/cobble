use console::style;
use std::{thread, time::Duration};

use clap::{Args, Parser, Subcommand};
use indicatif::ProgressBar;

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
    let cli = Cli::parse();
    match cli.command {
        Some(Commands::Run(file_paths)) => run_program(&file_paths.in_path),
        _ => unimplemented!(),
    }
}

fn run_program(path: &str) {
    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(Duration::from_millis(100));

    // Load source file
    pb.set_message(format!("{} Reading {}", style("[1/3]").bold().dim(), path));
    let src = match std::fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) => {
            pb.finish_with_message(format!(
                "{} {} while reading",
                style("[1/3]").bold().dim(),
                style("Error").red().bold(),
            ));
            println!("{}", e);
            return;
        }
    };
    thread::sleep(Duration::from_millis(250));

    // Compile program
    pb.set_message(format!(
        "{} Compiling {}",
        style("[2/3]").bold().dim(),
        path
    ));
    let prg = match cobble::compiler::compile_program(&src) {
        Ok(p) => p,
        Err(e) => {
            pb.finish_with_message(format!(
                "{} {} while compiling",
                style("[2/3]").bold().dim(),
                style("Error").red().bold(),
            ));
            println!("{}", e);
            return;
        }
    };
    thread::sleep(Duration::from_millis(250));

    // Interpret program
    pb.set_message(format!(
        "{} Interpreting {}",
        style("[3/3]").bold().dim(),
        path
    ));
    let (res, state) = cobble::interpreter::interpret_program(prg, None);
    if let Err(e) = res {
        pb.finish_with_message(format!(
            "{} {} while interpreting",
            style("[3/3]").bold().dim(),
            style("Error").red().bold(),
        ));
        println!("{}", e);
        return;
    }
    thread::sleep(Duration::from_millis(250));

    pb.finish_with_message(format!(
        "{} {}",
        style("[3/3]").bold().dim(),
        style("Done").green().bold()
    ));

    println!("{}", state);
}

#[test]
fn test_verify_cli() {
    use clap::CommandFactory;
    Cli::command().debug_assert();
}
