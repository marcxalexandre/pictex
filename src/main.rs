use clap::Parser;
use std::path::PathBuf;
use std::{fs, io, process};
use which::which;

mod utility;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct CLI {
    /// LaTeX math expression
    #[arg(value_name = "expression")]
    input: String,

    /// specify the output file for the image
    #[arg(short, long, value_name = "file")]
    output: Option<PathBuf>,

    /// set the output resolution
    #[arg(short, long, value_name = "num", default_value_t = 500)]
    dpi: u32,

    /// print detailed progress information
    #[arg(short, long, action)]
    verbose: bool,
}

fn main() -> io::Result<()> {
    if which("pdflatex").is_err() || which("dvipng").is_err() {
        eprintln!("program needs pdflatex und dvipng in order to work.");
        process::exit(1);
    }

    let cli = CLI::parse();
    let output_dir = std::env::temp_dir();
    assert!(output_dir.exists() && output_dir.is_dir());

    let temp_fn = utility::generate_temp_file_name();
    let tex_path = output_dir.join(temp_fn.with_extension("tex"));
    let dvi_path = output_dir.join(temp_fn.with_extension("dvi"));
    let png_path = match cli.output {
        Some(output) => output,
        None => temp_fn.with_extension("png")
    };

    // create tex file
    fs::write(tex_path.as_path(), utility::latex_template(&cli.input))?;

    utility::convert_tex_to_dvi(&tex_path, &output_dir, !cli.verbose)?;

    utility::convert_dvi_to_png(&dvi_path, &png_path, cli.dpi, !cli.verbose)?;

    println!("generated PNG file into {}", png_path.display());

    Ok(())
}
