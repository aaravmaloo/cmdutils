use crate::text::helpers;

/// Replace all occurrences of `find` with `replacement`.
///
/// * `input` — file path, or `None`/`"-"` to read stdin.
/// * `in_place` — rewrite the input file (requires a real file input).
/// * `output` — write the result to a file instead of stdout.
pub fn replace(
    find: &str,
    replacement: &str,
    input: Option<&str>,
    in_place: bool,
    output: Option<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    if find.is_empty() {
        return Err("Search string cannot be empty".into());
    }
    if in_place && output.is_some() {
        return Err("Cannot use both --in-place and --output".into());
    }
    // `-` means stdin, which cannot be rewritten in place.
    if in_place && input.is_none_or(|p| p == "-") {
        return Err("--in-place requires an input file".into());
    }

    let data = helpers::read_input(input)?;
    let text = String::from_utf8(data)?;
    let replaced = text.replace(find, replacement);

    if in_place {
        let path = input.unwrap();
        std::fs::write(path, replaced)?;
        println!("Replaced all occurrences of '{find}' in {path}");
    } else if let Some(out) = output {
        std::fs::write(out, replaced)?;
        println!(
            "Replaced all occurrences of '{find}' in {} → wrote {out}",
            helpers::source_name(input)
        );
    } else {
        print!("{replaced}");
    }

    Ok(())
}
