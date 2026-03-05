use rand::RngExt;
use rand::distr::Alphanumeric;
use std::path::{Path, PathBuf};
use std::io;
use std::process::{Stdio, Command};

/// Generates a minimal LaTeX document as a `String` that embeds the given LaTeX-compatible
/// mathematical expression (`input`) inside an inline math environment (`$...$`).
pub fn latex_template(input: &str) -> String {
    format!(
        r#"\documentclass{{minimal}}
\usepackage{{amsmath}}
\usepackage{{amssymb}}
\begin{{document}}
${}$
\end{{document}}"#,
        input
    )
}

/// Generates a unique temporary file name as a `PathBuf`, using the prefix
/// `"pictex_"` followed by a random sequence of alphanumeric characters.
pub fn generate_temp_file_name() -> PathBuf {
    PathBuf::from(format!(
        "pictex_{}",
        rand::rng()
            .sample_iter(&Alphanumeric)
            .take(10)
            .map(char::from)
            .collect::<String>()
    ))
}

pub fn convert_tex_to_dvi(input: &Path, output_dir: &Path, quiet: bool) -> io::Result<()> {
    let success = Command::new("pdflatex")
        .args([
            "-halt-on-error",
            "-interaction=nonstopmode",
            "-output-format=dvi"
        ])
        .arg(input)
        .current_dir(output_dir)
        .stdout(if quiet { Stdio::null() } else { Stdio::inherit() })
        .status()?
        .success();

    if !success {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!("LaTeX compilation failed ({})", success),
        ));
    }

    Ok(())
}

pub fn convert_dvi_to_png(input: &Path, output: &Path, dpi: u32, quiet: bool) -> io::Result<()> {
    let success = Command::new("dvipng")
        .args([
            "--width*", "--height*", "--depth*",
            "-D", dpi.to_string().as_str(),
            "-T", "tight",
            "-o"
        ])
        .arg(output)
        .arg(input)
        .stdout(if quiet { Stdio::null() } else { Stdio::inherit() })
        .status()?
        .success();

    if !success {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!("Image rendering failed ({})", success),
        ));
    }

    Ok(())
}