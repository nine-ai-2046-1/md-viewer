/// sanitize.rs - Filename sanitization for URL-safe HTML output
///
/// Converts markdown filenames to URL-safe HTML filenames.
/// Rules: lowercase, spaces→hyphens, keep [a-z0-9-], collapse hyphens, trim hyphens.

/// Sanitize a markdown filename stem into a URL-safe slug.
///
/// # Arguments
/// * `name` - The raw filename stem (without extension)
///
/// # Returns
/// URL-safe lowercase slug containing only [a-z0-9-]
pub fn sanitize_filename(name: &str) -> String {
    // Step 1: lowercase
    let lower = name.to_lowercase();

    // Step 2: replace spaces and underscores with hyphens
    let hyphenated: String = lower
        .chars()
        .map(|c| if c == ' ' || c == '_' { '-' } else { c })
        .collect();

    // Step 3: keep only a-z, 0-9, hyphens
    let filtered: String = hyphenated
        .chars()
        .filter(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || *c == '-')
        .collect();

    // Step 4: collapse consecutive hyphens into one
    let mut collapsed = String::with_capacity(filtered.len());
    let mut prev_hyphen = false;
    for c in filtered.chars() {
        if c == '-' {
            if !prev_hyphen {
                collapsed.push(c);
            }
            prev_hyphen = true;
        } else {
            collapsed.push(c);
            prev_hyphen = false;
        }
    }

    // Step 5: trim leading/trailing hyphens
    let trimmed = collapsed.trim_matches('-');

    // Fallback: if result is empty, use "untitled"
    if trimmed.is_empty() {
        "untitled".to_string()
    } else {
        trimmed.to_string()
    }
}

/// Convert a full markdown file path to an output HTML filename (without directory).
///
/// # Arguments
/// * `md_path` - Full path or just filename of the markdown file
///
/// # Returns
/// Sanitized HTML filename, e.g. "my-notes.html"
pub fn md_path_to_html_name(md_path: &std::path::Path) -> String {
    let stem = md_path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("untitled");
    format!("{}.html", sanitize_filename(stem))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_lowercase() {
        assert_eq!(sanitize_filename("MyNotes"), "mynotes");
        assert_eq!(sanitize_filename("README"), "readme");
    }

    #[test]
    fn test_space_to_hyphen() {
        assert_eq!(sanitize_filename("My Notes"), "my-notes");
        assert_eq!(sanitize_filename("API Reference"), "api-reference");
    }

    #[test]
    fn test_underscore_to_hyphen() {
        assert_eq!(sanitize_filename("hello_world"), "hello-world");
    }

    #[test]
    fn test_special_char_removal() {
        assert_eq!(sanitize_filename("config (1)"), "config-1");
        assert_eq!(sanitize_filename("API Reference (v2)"), "api-reference-v2");
        assert_eq!(sanitize_filename("foo!@#bar"), "foobar");
    }

    #[test]
    fn test_collapse_hyphens() {
        assert_eq!(sanitize_filename("a--b"), "a-b");
        assert_eq!(sanitize_filename("a - b"), "a-b");
    }

    #[test]
    fn test_trim_hyphens() {
        assert_eq!(sanitize_filename("-hello-"), "hello");
        assert_eq!(sanitize_filename("---"), "untitled");
    }

    #[test]
    fn test_empty_fallback() {
        assert_eq!(sanitize_filename(""), "untitled");
        assert_eq!(sanitize_filename("!!!"), "untitled");
    }

    #[test]
    fn test_digits_preserved() {
        assert_eq!(sanitize_filename("part1"), "part1");
        assert_eq!(sanitize_filename("v2 notes"), "v2-notes");
    }

    #[test]
    fn test_md_path_to_html_name() {
        assert_eq!(md_path_to_html_name(Path::new("My Notes.md")), "my-notes.html");
        assert_eq!(md_path_to_html_name(Path::new("/docs/API Reference.md")), "api-reference.html");
        assert_eq!(md_path_to_html_name(Path::new("README.md")), "readme.html");
    }

    #[test]
    fn test_yaml_path_to_html_name() {
        // .yaml and .yml extensions are stripped the same way as .md
        assert_eq!(md_path_to_html_name(Path::new("config.yaml")), "config.html");
        assert_eq!(md_path_to_html_name(Path::new("app-settings.yml")), "app-settings.html");
        assert_eq!(md_path_to_html_name(Path::new("/conf/My Config.yaml")), "my-config.html");
    }
}
