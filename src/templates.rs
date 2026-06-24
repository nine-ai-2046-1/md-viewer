/// templates.rs - HTML template loading and variable rendering
///
/// Provides simple {{KEY}} substitution for HTML templates embedded in the binary.
/// Templates: single.html (one file), multi.html (folder mode).

use std::collections::HashMap;

/// Render a template string by replacing all `{{KEY}}` placeholders.
///
/// Unknown keys are left as-is (no error).
pub fn render(template: &str, vars: &HashMap<&str, &str>) -> String {
    let mut result = template.to_string();
    for (key, value) in vars {
        let placeholder = format!("{{{{{}}}}}", key);
        result = result.replace(&placeholder, value);
    }
    result
}

/// Render a template with conditional blocks.
///
/// Supports:
/// - `{{#IF_INDEX}}...{{/IF_INDEX}}` — include only for index pages
/// - `{{#IF_DOC}}...{{/IF_DOC}}`    — include only for document pages
///
/// Then applies standard `{{KEY}}` substitution.
pub fn render_with_conditionals(
    template: &str,
    vars: &HashMap<&str, &str>,
    is_index: bool,
) -> String {
    let processed = apply_conditionals(template, is_index);
    render(&processed, vars)
}

/// Process conditional blocks: keep content inside matching blocks, remove others.
fn apply_conditionals(template: &str, is_index: bool) -> String {
    let mut result = template.to_string();

    if is_index {
        // Keep IF_INDEX content, remove IF_DOC content
        result = strip_block(&result, "DOC");
        result = unwrap_block(&result, "INDEX");
    } else {
        // Keep IF_DOC content, remove IF_INDEX content
        result = strip_block(&result, "INDEX");
        result = unwrap_block(&result, "DOC");
    }

    result
}

/// Remove a conditional block (including its content).
fn strip_block(template: &str, tag: &str) -> String {
    let open = format!("{{{{#IF_{}}}}}", tag);
    let close = format!("{{{{/IF_{}}}}}", tag);

    let mut result = String::new();
    let mut rest = template;

    loop {
        match rest.find(&open) {
            None => { result.push_str(rest); break; }
            Some(start) => {
                result.push_str(&rest[..start]);
                match rest[start..].find(&close) {
                    None => break, // Malformed template
                    Some(end_offset) => {
                        let end = start + end_offset + close.len();
                        rest = &rest[end..];
                    }
                }
            }
        }
    }

    result
}

/// Unwrap a conditional block (keep content, remove tags).
fn unwrap_block(template: &str, tag: &str) -> String {
    let open = format!("{{{{#IF_{}}}}}", tag);
    let close = format!("{{{{/IF_{}}}}}", tag);
    template
        .replace(&open, "")
        .replace(&close, "")
}

/// Build a HTML table of contents from a list of (level, id, text) heading tuples.
///
/// # Arguments
/// * `headings` - Vec of (level 1-6, id, display text)
///
/// # Returns
/// HTML `<ul>` list with nested structure and anchor links
pub fn build_toc_html(headings: &[(u32, String, String)]) -> String {
    if headings.is_empty() {
        return String::new();
    }

    let mut html = String::new();
    // Stack tracks the heading levels for which a <ul> is currently open.
    let mut stack: Vec<u32> = Vec::new();

    for (level, id, text) in headings {
        let level = *level;

        // Close any open levels deeper than current level.
        while stack.last().copied().unwrap_or(0) > level {
            stack.pop();
            html.push_str("</li></ul>");
        }

        if stack.last().copied().unwrap_or(0) == level {
            // Sibling heading: close the previous <li> at same level.
            html.push_str("</li>");
        } else {
            // Deeper heading (stack top < level, or stack empty): open new <ul>.
            html.push_str("<ul class=\"toc-list\">");
            stack.push(level);
        }

        html.push_str(&format!(
            "<li class=\"toc-item\"><a class=\"toc-link\" href=\"#{}\">{}</a>",
            escape_html_attr(id),
            escape_html(text)
        ));
    }

    // Close all remaining open levels.
    for _ in &stack {
        html.push_str("</li></ul>");
    }

    html
}

/// Build file cards HTML for the index page.
///
/// # Arguments
/// * `files` - Vec of (display title, url path, relative md path)
pub fn build_file_cards(files: &[(String, String, String)]) -> String {
    files
        .iter()
        .map(|(title, path, rel_path)| {
            format!(
                r#"<a class="file-card" href="{}">
  <div class="file-card-icon">📄</div>
  <div class="file-card-name">{}</div>
  <div class="file-card-path">{}</div>
</a>"#,
                escape_html_attr(path),
                escape_html(title),
                escape_html(rel_path)
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// Escape HTML special characters for text content and attributes.
pub fn escape_html(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

/// Escape for use inside HTML attributes (same as escape_html here).
pub fn escape_html_attr(s: &str) -> String {
    escape_html(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_basic() {
        let mut vars = HashMap::new();
        vars.insert("TITLE", "Hello World");
        vars.insert("CONTENT", "<p>hi</p>");
        let result = render("<title>{{TITLE}}</title>{{CONTENT}}", &vars);
        assert_eq!(result, "<title>Hello World</title><p>hi</p>");
    }

    #[test]
    fn test_render_unknown_key_untouched() {
        let vars = HashMap::new();
        let result = render("{{UNKNOWN}}", &vars);
        assert_eq!(result, "{{UNKNOWN}}");
    }

    #[test]
    fn test_conditional_index() {
        let tmpl = "A{{#IF_INDEX}}INDEX{{/IF_INDEX}}{{#IF_DOC}}DOC{{/IF_DOC}}B";
        let result = apply_conditionals(tmpl, true);
        assert_eq!(result, "AINDEXB");
    }

    #[test]
    fn test_conditional_doc() {
        let tmpl = "A{{#IF_INDEX}}INDEX{{/IF_INDEX}}{{#IF_DOC}}DOC{{/IF_DOC}}B";
        let result = apply_conditionals(tmpl, false);
        assert_eq!(result, "ADOCB");
    }

    #[test]
    fn test_build_toc_empty() {
        assert_eq!(build_toc_html(&[]), "");
    }

    #[test]
    fn test_build_toc_single() {
        let headings = vec![(2, "intro".to_string(), "Introduction".to_string())];
        let html = build_toc_html(&headings);
        assert!(html.contains("href=\"#intro\""));
        assert!(html.contains("Introduction"));
    }

    #[test]
    fn test_build_toc_siblings_share_ul() {
        // H1 with three H2 siblings: all three should be <li> inside ONE <ul>, not separate <ul> each.
        let headings = vec![
            (1, "title".to_string(), "Title".to_string()),
            (2, "feat".to_string(), "Features".to_string()),
            (2, "code".to_string(), "Code".to_string()),
            (2, "tbl".to_string(), "Table".to_string()),
        ];
        let html = build_toc_html(&headings);
        // Exactly two <ul> tags: one outer (for H1) and one inner (shared by all H2s).
        let ul_count = html.matches("<ul").count();
        assert_eq!(ul_count, 2, "Expected exactly 2 <ul> tags, got {ul_count}: {html}");
        // All four headings present.
        assert!(html.contains("#title"));
        assert!(html.contains("#feat"));
        assert!(html.contains("#code"));
        assert!(html.contains("#tbl"));
    }

    #[test]
    fn test_build_toc_back_to_parent() {
        // H1, H2, H1: second H1 must not open an extra <ul>.
        let headings = vec![
            (1, "a".to_string(), "A".to_string()),
            (2, "b".to_string(), "B".to_string()),
            (1, "c".to_string(), "C".to_string()),
        ];
        let html = build_toc_html(&headings);
        // Outer <ul> for H1s, inner <ul> for the H2.
        let ul_count = html.matches("<ul").count();
        assert_eq!(ul_count, 2, "Expected exactly 2 <ul> tags, got {ul_count}: {html}");
    }

    #[test]
    fn test_escape_html() {
        assert_eq!(escape_html("<script>alert('xss')</script>"), "&lt;script&gt;alert(&#39;xss&#39;)&lt;/script&gt;");
    }
}
