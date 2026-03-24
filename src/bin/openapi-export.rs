use std::fs;
use std::path::Path;

use fhevm_relayer::http::middleware::openapi::build_openapi_doc;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let spec = build_openapi_doc();
    let yaml = serde_yaml::to_string(&spec)?;

    // Post-process the YAML to ensure long hex strings are quoted
    let processed_yaml = ensure_hex_strings_quoted(&yaml);

    let path = Path::new("openapi.yml");
    fs::write(path, &processed_yaml)?;

    println!("OpenAPI specification written to {}", path.display());
    Ok(())
}

// ── Hex string quoting ──────────────────────────────────────────────────────

/// Ensures that hexadecimal strings are properly quoted in YAML to prevent
/// them from being interpreted as numbers in exponential notation or as YAML hex literals.
fn ensure_hex_strings_quoted(yaml: &str) -> String {
    yaml.lines()
        .map(|line| {
            // Check if line contains "example:" followed by a hex string
            if let Some(example_pos) = line.find("example:") {
                let after_example = &line[example_pos + 8..].trim_start();
                if is_long_unquoted_hex(after_example) || is_short_hex_string(after_example) {
                    let indent = &line[..example_pos];
                    return format!(r#"{}example: "{}""#, indent, after_example);
                }
            }

            // Check if line is an array item (starts with "- ") followed by a hex string
            let trimmed = line.trim_start();
            if let Some(rest) = trimmed.strip_prefix("- ") {
                let after_dash = rest.trim_start();
                let should_quote = is_long_unquoted_hex(after_dash)
                    || (after_dash.starts_with("0x")
                        && !after_dash.starts_with('"')
                        && !after_dash.starts_with('\'')
                        && after_dash.len() >= 3
                        && after_dash[2..].chars().all(|c| c.is_ascii_hexdigit()));

                if should_quote {
                    let indent_len = line.len() - trimmed.len();
                    let indent = &line[..indent_len];
                    return format!(r#"{}- "{}""#, indent, after_dash);
                }
            }

            // Check for key: 0x... pattern (YAML hex literals that need quoting)
            if let Some(colon_pos) = line.find(':') {
                let after_colon = &line[colon_pos + 1..].trim_start();
                if after_colon.starts_with("0x")
                    && !after_colon.starts_with('"')
                    && !after_colon.starts_with('\'')
                {
                    let hex_end = after_colon
                        .find(|c: char| c.is_whitespace() || c == '#')
                        .unwrap_or(after_colon.len());
                    let hex_part = &after_colon[..hex_end];

                    if hex_part.len() >= 3 && hex_part[2..].chars().all(|c| c.is_ascii_hexdigit()) {
                        let before_colon = &line[..=colon_pos];
                        let after_hex = &after_colon[hex_end..];
                        return format!(r#"{} "{}"{}"#, before_colon, hex_part, after_hex);
                    }
                }
            }

            line.to_string()
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn is_long_unquoted_hex(s: &str) -> bool {
    if s.len() < 16 {
        return false;
    }
    if s.starts_with('"') || s.starts_with('\'') {
        return false;
    }
    s.chars().all(|c| c.is_ascii_hexdigit())
}

fn is_short_hex_string(s: &str) -> bool {
    if s.len() < 8 || s.len() >= 16 {
        return false;
    }
    if s.starts_with('"') || s.starts_with('\'') {
        return false;
    }
    // Must contain at least one a-f letter to be ambiguous hex.
    // Pure-digit strings (e.g. 11155111) are valid YAML integers and must not be quoted.
    s.chars().all(|c| c.is_ascii_hexdigit()) && s.chars().any(|c| c.is_ascii_alphabetic())
}
