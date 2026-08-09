use crate::text::helpers;

/// Supported letter-case styles.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CaseStyle {
    Upper,
    Lower,
    Title,
    Snake,
    Kebab,
    Camel,
    Pascal,
    Constant,
}

/// Parse a case-style name into a [`CaseStyle`].
pub fn parse_case(s: &str) -> Result<CaseStyle, String> {
    match s.to_lowercase().as_str() {
        "upper" => Ok(CaseStyle::Upper),
        "lower" => Ok(CaseStyle::Lower),
        "title" => Ok(CaseStyle::Title),
        "snake" => Ok(CaseStyle::Snake),
        "kebab" => Ok(CaseStyle::Kebab),
        "camel" => Ok(CaseStyle::Camel),
        "pascal" => Ok(CaseStyle::Pascal),
        "constant" => Ok(CaseStyle::Constant),
        other => Err(format!(
            "Unknown case '{other}'. Valid: upper, lower, title, snake, kebab, camel, pascal, constant"
        )),
    }
}

/// Convert text to a target case.
///
/// Text comes from `text` (literal), or from the file/stdin given by `input`.
pub fn case(
    input: Option<&str>,
    text: Option<&str>,
    style: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let style = parse_case(style)?;

    let content = match text {
        Some(t) => t.to_string(),
        None => String::from_utf8(helpers::read_input(input)?)?,
    };

    println!("{}", convert_case(&content, &style));
    Ok(())
}

/// Split text into lowercase words on non-alphanumeric boundaries,
/// also splitting camelCase / PascalCase transitions.
fn words(s: &str) -> Vec<String> {
    s.split(|c: char| !c.is_alphanumeric())
        .filter(|w| !w.is_empty())
        .flat_map(split_camel)
        .map(|w| w.to_lowercase())
        .collect()
}

/// Split a single word at lowercase→uppercase boundaries: `helloWorld` →
/// `["hello", "World"]`.
fn split_camel(word: &str) -> Vec<String> {
    let mut parts = Vec::new();
    let mut current = String::new();
    for c in word.chars() {
        if c.is_uppercase() && !current.is_empty() {
            parts.push(std::mem::take(&mut current));
        }
        current.push(c);
    }
    if !current.is_empty() {
        parts.push(current);
    }
    parts
}

fn capitalize(word: &str) -> String {
    let mut chars = word.chars();
    match chars.next() {
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
        None => String::new(),
    }
}

fn convert_case(s: &str, style: &CaseStyle) -> String {
    match style {
        CaseStyle::Upper => s.to_uppercase(),
        CaseStyle::Lower => s.to_lowercase(),
        CaseStyle::Title => words(s)
            .iter()
            .map(|w| capitalize(w))
            .collect::<Vec<_>>()
            .join(" "),
        CaseStyle::Snake => words(s).join("_"),
        CaseStyle::Kebab => words(s).join("-"),
        CaseStyle::Camel => {
            let ws = words(s);
            let mut out = String::new();
            for (i, w) in ws.iter().enumerate() {
                if i == 0 {
                    out.push_str(w);
                } else {
                    out.push_str(&capitalize(w));
                }
            }
            out
        }
        CaseStyle::Pascal => words(s).iter().map(|w| capitalize(w)).collect::<String>(),
        CaseStyle::Constant => words(s).join("_").to_uppercase(),
    }
}
