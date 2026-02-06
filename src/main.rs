use clap::Parser;
use pictex;
use std::path::PathBuf;
use std::{fs, process};
use which::which;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct CLI {
    /// LaTeX math expression
    #[arg(value_name = "expression")]
    input: String,

    /// specify the output file for the image
    #[arg(short, long, value_name = "file")]
    output: Option<PathBuf>,

    /// specify the output directory for the generated files
    #[arg(short = 'D', long, value_name = "dir", default_value = "/tmp")]
    output_directory: PathBuf,

    /// set the output resolution
    #[arg(short, long, value_name = "num", default_value_t = 500)]
    dpi: u32,

    /// Suppress normal output; errors are still displayed.
    #[arg(short, action)]
    quiet: bool,
}

fn main() {
    if which("pdflatex").is_err() || which("dvipng").is_err() {
        eprintln!("program needs pdflatex und dvipng in order to work.");
        process::exit(1);
    }

    let cli = CLI::parse();

    if cli.input.is_empty() {
        eprintln!("Cannot parse empty string");
        process::exit(1);
    }

    let png_path =
        pictex::generate_png_from_math_expr(&cli.input, cli.dpi, &cli.output_directory, cli.quiet)
            .unwrap_or_else(|e| {
                eprintln!("Failed to create png file: {}", e);
                process::exit(1);
            });

    let output_file_path = match cli.output {
        None => png_path,
        Some(file) => {
            fs::copy(&png_path, &file).unwrap_or_else(|e| {
                eprintln!(
                    "Could not copy {} to {}:\n{}",
                    png_path.display(),
                    file.display(),
                    e
                );
                process::exit(1);
            });

            file
        }
    };

    if !cli.quiet {
        println!("generated PNG file into {}", output_file_path.display())
    }
}
