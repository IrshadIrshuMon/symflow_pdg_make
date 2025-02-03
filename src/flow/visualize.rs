//! Visualization of flow graphs.

use std::collections::HashMap;
use std::io::{Result, Write};

pub const BR: &str = "<br align=\"left\"/>";


/// Write the preamble of the graphviz file.
pub fn write_header<W: Write>(mut f: W, title: &str, fontsize: u32) -> Result<()> {
    writeln!(f, "digraph Flow {{")?;
    write!(f, "graph [label=\"{}\", labelloc=\"t\", fontsize={}, ", title, fontsize)?;
    writeln!(f, "fontname=\"Source Code Pro\"]")?;
    writeln!(f, "node [fontname=\"Source Code Pro\"]")?;
    writeln!(f, "edge [fontname=\"Source Code Pro\"]")
}

/// Write condition edges.
pub fn write_edges<W: Write, F, T>(
    mut f: W,
    edges: &HashMap<(usize, usize), T>,
    writer: F
) -> Result<()> where F: Fn(&mut W, ((usize, usize), &T)) -> Result<()> {
    // Export the edges, but sort them first to make the graphviz output
    // deterministic eventhough the hash map cannot be traversed in order.
    let mut edges = edges.iter().collect::<Vec<_>>();
    edges.sort_by_key(|edge| edge.0);
    for (&(start, end), edge) in edges {
        write!(f, "b{} -> b{} [", start, end)?;
        writer(&mut f, ((start, end), edge))?;
        writeln!(f, "]")?;
    }
    Ok(())
}

/// Write the closing of the file.
pub fn write_footer<W: Write>(mut f: W) -> Result<()> {
    writeln!(f, "}}")
}


#[cfg(test)]
pub mod test {
    use std::fs::{self, File};
    use std::process::Command;
    use super::*;

    /// Compile the file with graphviz.
    pub fn compile<F>(dir: &str, filename: &str, writer: F) -> std::result::Result<(), std::io::Error>
where
    F: FnOnce(File) -> std::result::Result<(), std::io::Error>,
{
    use std::path::{Path, PathBuf};

    // Ensure output directory exists
    let output_dir = Path::new("target/out").join(dir);
    fs::create_dir_all(&output_dir)?;

    // Create temporary file for graph description
    let temp_path = PathBuf::from("target/graph.dot");
    let temp_file = File::create(&temp_path)?;
    writer(temp_file)?;

    // Define output `.dot` file path
    let output_path = output_dir.join(format!("{}.dot", filename));

    // Graphviz command to process and output to `.dot` file
    let cmd = format!(
        "ccomps -x {} | dot | gvpack -n | neato -Tdot -o {}",
        temp_path.display(),
        output_path.display()
    );

    let output = Command::new("bash")
        .arg("-c")
        .arg(cmd)
        .output()?;

    if !output.status.success() {
        eprintln!("Graphviz command failed: {:?}", String::from_utf8_lossy(&output.stderr));
        return Err(std::io::Error::new(std::io::ErrorKind::Other, "Graphviz command failed"));
    }

    std::io::stdout().write_all(&output.stdout)?;
    std::io::stderr().write_all(&output.stderr)?;

    Ok(())
}

}
