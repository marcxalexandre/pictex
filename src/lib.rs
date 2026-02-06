use rand::Rng;
use rand::distr::Alphanumeric;
use std::path::{Path, PathBuf};
use std::{fs, io, process};

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

/// Writes a `.tex` file with `input` wrapped by [`latex_template`] to `output_dir`
/// using a temporary name from [`generate_temp_file_name`], then compiles it with
/// `pdflatex` in DVI mode. `stdout` configures compiler output handling.
///
/// Returns the `.png` file path on success, or an `io::Error` on failure.
///
/// # Requirements
/// `pdflatex` and `dvipng` must be available in the system PATH.
pub fn generate_png_from_math_expr(
    expr: &str,
    dpi: u32,
    output_dir: &Path,
    quiet: bool,
) -> io::Result<PathBuf> {
    if !output_dir.is_dir() {
        return Err(io::Error::new(
            io::ErrorKind::NotADirectory,
            format!("'{}' is not a directory", output_dir.display()),
        ));
    }

    // generate file names
    let temp_fn = generate_temp_file_name();
    let tex_path = output_dir.join(temp_fn.with_extension("tex"));
    let dvi_path = output_dir.join(temp_fn.with_extension("dvi"));
    let png_path = output_dir.join(temp_fn.with_extension("png"));

    // write to file
    fs::write(tex_path.as_path(), latex_template(expr))?;

    // convert tex to dvi
    let status = process::Command::new("pdflatex")
        .args([
            "-halt-on-error",
            "-interaction=nonstopmode",
            "-output-format=dvi",
        ])
        .arg(tex_path)
        .current_dir(output_dir)
        .stdout(if quiet {
            process::Stdio::null()
        } else {
            process::Stdio::inherit()
        })
        .status()?;

    if !status.success() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!("LaTeX compilation failed ({})", status),
        ));
    }

    // convert dvi to png
    let status = process::Command::new("dvipng")
        .args(["--width*", "--height*", "--depth*"])
        .args(["-D", dpi.to_string().as_str()])
        .args(["-T", "tight"])
        .arg(dvi_path)
        .arg("-o")
        .arg(png_path.as_path())
        .stdout(if quiet {
            process::Stdio::null()
        } else {
            process::Stdio::inherit()
        })
        .status()?;

    if !status.success() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!("Image rendering failed ({})", status),
        ));
    }

    Ok(png_path)
}
