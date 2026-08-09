use std::io::Read;
use std::io::Write;

/// Read the full input bytes from a file, or from stdin when `input` is
/// `None` or `"-"`.
pub fn read_input(input: Option<&str>) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    match input {
        Some(path) if path != "-" => std::fs::read(path).map_err(|e| {
            if e.kind() == std::io::ErrorKind::NotFound {
                format!("Input file not found: {path}").into()
            } else {
                Box::new(e) as Box<dyn std::error::Error>
            }
        }),
        _ => {
            let mut buf = Vec::new();
            std::io::stdin().lock().read_to_end(&mut buf)?;
            Ok(buf)
        }
    }
}

/// Human-readable source name for display in output messages.
pub fn source_name(input: Option<&str>) -> &str {
    match input {
        Some(path) if path != "-" => path,
        _ => "stdin",
    }
}

/// Write output bytes either to a file or to stdout.
pub fn write_output(
    output: Option<&str>,
    data: &[u8],
    source: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    match output {
        Some(path) => {
            std::fs::write(path, data)?;
            println!("Wrote {} bytes from {source} to {path}", data.len());
        }
        None => {
            std::io::stdout().write_all(data)?;
        }
    }
    Ok(())
}
