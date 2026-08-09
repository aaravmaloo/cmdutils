use crate::text::helpers;

/// Which counts to show. If all are `false`, everything is shown.
#[derive(Debug, Clone, Copy, Default)]
pub struct CountFlags {
    pub words: bool,
    pub lines: bool,
    pub chars: bool,
    pub bytes: bool,
}

/// Count lines, words, characters, and bytes of a file or stdin.
///
/// * `input` — file path, or `None`/`"-"` to read stdin.
/// * `flags` — which counts to print (all when none are set).
pub fn count(input: Option<&str>, flags: CountFlags) -> Result<(), Box<dyn std::error::Error>> {
    let data = helpers::read_input(input)?;

    let lines = data.iter().filter(|&&b| b == b'\n').count();
    let text = String::from_utf8_lossy(&data);
    let words = text.split_whitespace().count();
    let chars = text.chars().count();
    let bytes = data.len();

    let show_all = !(flags.words || flags.lines || flags.chars || flags.bytes);

    let mut parts = Vec::new();
    if show_all || flags.lines {
        parts.push(format!("{lines:>7} lines"));
    }
    if show_all || flags.words {
        parts.push(format!("{words:>7} words"));
    }
    if show_all || flags.chars {
        parts.push(format!("{chars:>7} chars"));
    }
    if show_all || flags.bytes {
        parts.push(format!("{bytes:>7} bytes"));
    }

    println!("{}  {}", parts.join(", "), helpers::source_name(input));
    Ok(())
}
