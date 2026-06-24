/// builder.rs - Markdown and YAML to HTML conversion and file discovery
///
/// Uses pulldown-cmark to convert markdown to HTML.
/// YAML files are rendered as Prism.js syntax-highlighted code blocks.
/// Generates TOC, handles syntax highlighting class injection, task lists.
/// Also handles recursive file scanning for folder mode.

use pulldown_cmark::{html, Event, HeadingLevel, Options, Parser, Tag, TagEnd};
use regex::Regex;
use std::path::{Path, PathBuf};
use std::sync::LazyLock;

use crate::sanitize::md_path_to_html_name;
use crate::templates::{build_toc_html, escape_html};

// ── File type ────────────────────────────────────────────────────────────────

/// The type of a source file supported by md-viewer.
#[derive(Debug, Clone, PartialEq)]
pub enum FileType {
    /// CommonMark / GFM markdown (.md)
    Markdown,
    /// YAML document (.yaml / .yml)
    Yaml,
}

impl FileType {
    /// Detect file type from a path's extension.
    /// Returns None if the extension is not supported.
    pub fn from_path(path: &Path) -> Option<Self> {
        match path.extension().and_then(|e| e.to_str()) {
            Some("md") => Some(FileType::Markdown),
            Some("yaml") | Some("yml") => Some(FileType::Yaml),
            _ => None,
        }
    }
}

// ── Markdown parsing options ─────────────────────────────────────────────────

/// Build pulldown-cmark Options with tables, footnotes, and task lists enabled.
fn cmark_options() -> Options {
    let mut opts = Options::empty();
    opts.insert(Options::ENABLE_TABLES);
    opts.insert(Options::ENABLE_FOOTNOTES);
    opts.insert(Options::ENABLE_TASKLISTS);
    opts.insert(Options::ENABLE_STRIKETHROUGH);
    opts.insert(Options::ENABLE_HEADING_ATTRIBUTES);
    opts
}

// ── Heading extraction ──────────────────────────────────────────────────────

/// A heading found in the parsed markdown.
#[derive(Debug, Clone)]
pub struct Heading {
    /// Heading level: 1-6.
    pub level: u32,
    /// Generated HTML id (slug).
    pub id: String,
    /// Plain text display label.
    pub text: String,
}

/// Parse markdown source and extract all headings for TOC generation.
fn extract_headings(md: &str) -> Vec<Heading> {
    let parser = Parser::new_ext(md, cmark_options());
    let mut headings = Vec::new();
    let mut current_level: Option<u32> = None;
    let mut current_text = String::new();
    let mut id_counter: std::collections::HashMap<String, usize> = std::collections::HashMap::new();

    for event in parser {
        match event {
            Event::Start(Tag::Heading { level, .. }) => {
                current_level = Some(heading_level_to_u32(level));
                current_text.clear();
            }
            Event::Text(t) if current_level.is_some() => {
                current_text.push_str(&t);
            }
            Event::Code(t) if current_level.is_some() => {
                current_text.push_str(&t);
            }
            Event::End(TagEnd::Heading(_)) => {
                if let Some(level) = current_level.take() {
                    let base_id = slugify(&current_text);
                    let count = id_counter.entry(base_id.clone()).or_insert(0);
                    let id = if *count == 0 {
                        base_id.clone()
                    } else {
                        format!("{}-{}", base_id, count)
                    };
                    *count += 1;
                    headings.push(Heading {
                        level,
                        id,
                        text: current_text.clone(),
                    });
                }
            }
            _ => {}
        }
    }

    headings
}

// ── HTML generation ─────────────────────────────────────────────────────────

/// Render markdown source to an HTML fragment (not a full document).
///
/// Returns (html_body, headings) where html_body is the rendered content
/// and headings is extracted for TOC generation.
pub fn render_markdown(md: &str) -> (String, Vec<Heading>) {
    let headings = extract_headings(md);
    let html_body = render_body(md, &headings);
    (html_body, headings)
}

/// Render markdown to HTML body, injecting heading IDs for anchor links.
fn render_body(md: &str, headings: &[Heading]) -> String {
    let parser = Parser::new_ext(md, cmark_options());

    // We need to inject id= attributes on heading elements.
    // pulldown-cmark supports ENABLE_HEADING_ATTRIBUTES but the syntax
    // is {#id} in markdown. Instead we post-process the HTML to add IDs.
    let mut html_output = String::with_capacity(md.len() * 2);
    html::push_html(&mut html_output, parser);

    // Post-process: inject heading IDs matching our TOC
    let with_ids = inject_heading_ids(&html_output, headings);

    // Post-process: rewrite .md hyperlinks to .html so cross-references work
    let with_links = rewrite_md_links(&with_ids);

    // Post-process: rewrite relative image paths to use /source/ prefix
    rewrite_image_paths(&with_links)
}

/// Post-process rendered HTML to inject id= attributes on heading tags.
///
/// Matches headings by their text content (in order of appearance).
fn inject_heading_ids(html: &str, headings: &[Heading]) -> String {
    if headings.is_empty() {
        return html.to_string();
    }

    let mut result = String::with_capacity(html.len());
    let mut remaining = html;
    let mut heading_idx = 0;

    while !remaining.is_empty() && heading_idx < headings.len() {
        let h = &headings[heading_idx];
        let open_tag = format!("<h{}>", h.level);
        let open_tag_with_id = format!("<h{} id=\"{}\">", h.level, escape_html(&h.id));

        match remaining.find(&open_tag) {
            None => break,
            Some(pos) => {
                result.push_str(&remaining[..pos]);
                result.push_str(&open_tag_with_id);
                remaining = &remaining[pos + open_tag.len()..];
                heading_idx += 1;
            }
        }
    }

    result.push_str(remaining);
    result
}

/// Lazy-compiled regex for matching .md links in href attributes.
/// Matches: href="path/to/file.md" or href='path/to/file.md'
static MD_LINK_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"href="([^"]*\.md)""#).unwrap()
});

/// Rewrite relative `.md` hyperlinks in rendered HTML to `.html`.
///
/// This enables markdown cross-references to resolve to the correct
/// rendered pages. Only `href` attributes are rewritten; `src` attributes
/// (images, videos) are left untouched so they can be served via `/source/`.
///
/// # Example
/// ```ignore
/// let html = rewrite_md_links(r#"<a href="./other.md">link</a>"#);
/// assert_eq!(html, r#"<a href="./other.html">link</a>"#);
/// ```
pub fn rewrite_md_links(html: &str) -> String {
    MD_LINK_RE
        .replace_all(html, |caps: &regex::Captures| {
            let original = &caps[1];
            let rewritten = original.strip_suffix(".md").unwrap_or(original);
            format!("href=\"{}.html\"", rewritten)
        })
        .into_owned()
}

/// Lazy-compiled regex for matching image src attributes.
/// Matches: src="path/to/image.png"
static IMG_SRC_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"src="([^"]*)""#).unwrap()
});

/// Rewrite relative image `src` attributes to use `/source/` prefix.
///
/// This enables images referenced in markdown to load correctly via the
/// `/source/*` HTTP route. Only relative paths are rewritten; absolute URLs,
/// protocol-relative URLs, and data URIs are left untouched.
///
/// # Example
/// ```ignore
/// let html = rewrite_image_paths(r#"<img src="screenshots/photo.png">"#);
/// assert_eq!(html, r#"<img src="/source/screenshots/photo.png">"#);
/// ```
pub fn rewrite_image_paths(html: &str) -> String {
    IMG_SRC_RE
        .replace_all(html, |caps: &regex::Captures| {
            let src = &caps[1];

            // Skip absolute paths (already start with /)
            if src.starts_with('/') {
                return caps[0].to_string();
            }

            // Skip protocol-relative URLs (//cdn.example.com/...)
            if src.starts_with("//") {
                return caps[0].to_string();
            }

            // Skip data URIs (data:image/...)
            if src.starts_with("data:") {
                return caps[0].to_string();
            }

            // Skip URLs with protocol (https://, http://, etc.)
            if src.contains("://") {
                return caps[0].to_string();
            }

            // Normalize: strip leading ./ prefix
            let normalized = src.strip_prefix("./").unwrap_or(src);

            // Rewrite relative path with /source/ prefix
            format!("src=\"/source/{}\"", normalized)
        })
        .into_owned()
}

/// Render a YAML source file as a syntax-highlighted code block (HTML fragment).
///
/// Returns (html_body, headings) where headings is always empty — YAML files
/// have no document headings; the TOC panel will be omitted by the template.
pub fn render_yaml(yaml_content: &str) -> (String, Vec<Heading>) {
    // Escape HTML special characters before injecting into <code> block
    let escaped = escape_html(yaml_content);
    let html = format!(
        "<pre><code class=\"language-yaml\">{}</code></pre>\n",
        escaped
    );
    (html, vec![])
}

// ── TOC ─────────────────────────────────────────────────────────────────────

/// Build HTML TOC from a list of headings.
pub fn build_toc(headings: &[Heading]) -> String {
    let items: Vec<(u32, String, String)> = headings
        .iter()
        .map(|h| (h.level, h.id.clone(), h.text.clone()))
        .collect();
    build_toc_html(&items)
}

// ── File scanning ───────────────────────────────────────────────────────────

/// Information about a discovered source file (.md, .yaml, or .yml).
#[derive(Debug, Clone)]
pub struct MdFile {
    /// Absolute path to the source file.
    pub source: PathBuf,
    /// Path relative to the scanned root directory.
    pub relative: PathBuf,
    /// Display title (derived from filename stem).
    pub title: String,
    /// Output HTML filename (sanitized).
    pub html_name: String,
    /// URL path for browser (e.g. "/my-notes").
    pub url_path: String,
    /// Whether this is a markdown or YAML file.
    pub file_type: FileType,
}

/// Recursively scan `dir` for all `.md`, `.yaml`, and `.yml` files.
///
/// Returns them sorted: directories before files, then alphabetically.
pub fn scan_markdown_files(dir: &Path) -> Result<Vec<MdFile>, String> {
    if !dir.exists() {
        return Err(format!("Directory not found: {}", dir.display()));
    }
    if !dir.is_dir() {
        return Err(format!("Not a directory: {}", dir.display()));
    }

    let mut files = Vec::new();
    collect_md_files(dir, dir, &mut files)?;

    if files.is_empty() {
        return Err(format!(
            "No markdown or YAML files found in: {}",
            dir.display()
        ));
    }

    // Sort: by relative path alphabetically
    files.sort_by(|a, b| a.relative.cmp(&b.relative));
    Ok(files)
}

/// Recursively collect .md files from `current_dir` relative to `root`.
fn collect_md_files(root: &Path, current: &Path, files: &mut Vec<MdFile>) -> Result<(), String> {
    let entries = std::fs::read_dir(current)
        .map_err(|e| format!("Cannot read dir {}: {}", current.display(), e))?;

    let mut dir_entries: Vec<_> = entries
        .filter_map(|e| e.ok())
        .collect();

    // Sort: directories first, then files, alphabetically
    dir_entries.sort_by(|a, b| {
        let a_is_dir = a.path().is_dir();
        let b_is_dir = b.path().is_dir();
        match (a_is_dir, b_is_dir) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => a.file_name().cmp(&b.file_name()),
        }
    });

    for entry in dir_entries {
        let path = entry.path();

        // Skip hidden files/dirs
        if entry.file_name().to_string_lossy().starts_with('.') {
            continue;
        }

        if path.is_dir() {
            collect_md_files(root, &path, files)?;
        } else if let Some(file_type) = FileType::from_path(&path) {
            let relative = path.strip_prefix(root)
                .unwrap_or(&path)
                .to_path_buf();

            let html_name = md_path_to_html_name(&path);
            let url_path = format!("/{}", html_name.trim_end_matches(".html"));
            let title = path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("Untitled")
                .to_string();

            files.push(MdFile {
                source: path,
                relative,
                title,
                html_name,
                url_path,
                file_type,
            });
        }
    }

    Ok(())
}

// ── Tree structure ───────────────────────────────────────────────────────────

/// A node in the file tree (for sidebar rendering).
#[derive(serde::Serialize, Debug, Clone)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum TreeNode {
    Folder {
        name: String,
        path: String,
        children: Vec<TreeNode>,
    },
    File {
        name: String,
        path: String,
    },
}

/// Build a hierarchical tree from a flat list of MdFile entries.
///
/// Groups files by their directory structure.
pub fn build_file_tree(files: &[MdFile]) -> Vec<TreeNode> {
    let mut root: Vec<TreeNode> = Vec::new();
    build_tree_recursive(files, &PathBuf::new(), &mut root);
    root
}

fn build_tree_recursive(files: &[MdFile], prefix: &Path, nodes: &mut Vec<TreeNode>) {
    use std::collections::BTreeMap;

    // Group files by their first component under `prefix`
    let mut dirs: BTreeMap<String, Vec<&MdFile>> = BTreeMap::new();
    let mut direct: Vec<&MdFile> = Vec::new();

    for file in files {
        let rel = file.relative.strip_prefix(prefix).unwrap_or(&file.relative);
        let components: Vec<_> = rel.components().collect();

        if components.len() == 1 {
            direct.push(file);
        } else if let Some(first) = components.first() {
            let dir_name = first.as_os_str().to_string_lossy().to_string();
            dirs.entry(dir_name).or_default().push(file);
        }
    }

    // Add directories first
    for (dir_name, dir_files) in &dirs {
        let dir_prefix = prefix.join(dir_name);
        let mut children = Vec::new();
        let owned_files: Vec<MdFile> = dir_files.iter().map(|f| (*f).clone()).collect();
        build_tree_recursive(&owned_files, &dir_prefix, &mut children);
        nodes.push(TreeNode::Folder {
            name: dir_name.clone(),
            path: dir_prefix.to_string_lossy().to_string(),
            children,
        });
    }

    // Then add direct files
    for file in direct {
        nodes.push(TreeNode::File {
            name: file.title.clone(),
            path: file.html_name.trim_end_matches(".html").to_string(),
        });
    }
}

// ── Search index ─────────────────────────────────────────────────────────────

/// A searchable entry for fuse.js.
#[derive(serde::Serialize, Debug)]
pub struct SearchEntry {
    pub title: String,
    pub path: String,
    pub content: String,
    pub headings: Vec<String>,
}

/// Build search index from a list of (MdFile, raw file content) pairs.
///
/// Markdown files: headings and stripped text are extracted via pulldown-cmark.
/// YAML files: raw content is used directly; headings are empty.
pub fn build_search_index(files: &[(MdFile, String)]) -> Vec<SearchEntry> {
    files
        .iter()
        .map(|(file, content)| match file.file_type {
            FileType::Markdown => {
                let (_, headings) = render_markdown(content);
                let heading_texts: Vec<String> = headings.iter().map(|h| h.text.clone()).collect();
                let plain = strip_markdown(content);
                SearchEntry {
                    title: file.title.clone(),
                    path: file.url_path.clone(),
                    content: plain.chars().take(2000).collect(),
                    headings: heading_texts,
                }
            }
            FileType::Yaml => {
                // YAML content is indexed as raw text; no headings to extract
                SearchEntry {
                    title: file.title.clone(),
                    path: file.url_path.clone(),
                    content: content.chars().take(2000).collect(),
                    headings: vec![],
                }
            }
        })
        .collect()
}

/// Very basic markdown stripping — remove common syntax for search indexing.
fn strip_markdown(md: &str) -> String {
    let parser = Parser::new_ext(md, cmark_options());
    let mut text = String::new();
    for event in parser {
        match event {
            Event::Text(t) | Event::Code(t) => {
                text.push_str(&t);
                text.push(' ');
            }
            Event::SoftBreak | Event::HardBreak => text.push(' '),
            _ => {}
        }
    }
    text
}

// ── Helpers ──────────────────────────────────────────────────────────────────

/// Convert a HeadingLevel enum to a u32.
fn heading_level_to_u32(level: HeadingLevel) -> u32 {
    match level {
        HeadingLevel::H1 => 1,
        HeadingLevel::H2 => 2,
        HeadingLevel::H3 => 3,
        HeadingLevel::H4 => 4,
        HeadingLevel::H5 => 5,
        HeadingLevel::H6 => 6,
    }
}

/// Convert heading text to a URL-safe slug (for id attributes).
pub fn slugify(text: &str) -> String {
    let lower = text.to_lowercase();
    let mut slug = String::with_capacity(lower.len());
    let mut prev_hyphen = false;
    for c in lower.chars() {
        if c.is_ascii_alphanumeric() {
            slug.push(c);
            prev_hyphen = false;
        } else if c == ' ' || c == '-' || c == '_' {
            if !prev_hyphen && !slug.is_empty() {
                slug.push('-');
                prev_hyphen = true;
            }
        }
    }
    slug.trim_matches('-').to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_markdown_basic() {
        let (html, _) = render_markdown("# Hello\n\nWorld");
        assert!(html.contains("<h1"));
        assert!(html.contains("Hello"));
        assert!(html.contains("<p>World</p>"));
    }

    #[test]
    fn test_heading_extraction() {
        let md = "# Title\n## Section\n### Subsection";
        let headings = extract_headings(md);
        assert_eq!(headings.len(), 3);
        assert_eq!(headings[0].level, 1);
        assert_eq!(headings[1].level, 2);
        assert_eq!(headings[2].level, 3);
    }

    #[test]
    fn test_toc_generation() {
        let md = "# Title\n## Section A\n## Section B";
        let headings = extract_headings(md);
        let toc = build_toc(&headings);
        assert!(toc.contains("href=\"#title\"") || toc.contains("href=\"#"));
        assert!(toc.contains("Section A") || toc.contains("section-a"));
    }

    #[test]
    fn test_table_rendering() {
        let md = "| Col A | Col B |\n|-------|-------|\n| 1     | 2     |";
        let (html, _) = render_markdown(md);
        assert!(html.contains("<table>") || html.contains("<table "));
        assert!(html.contains("<th>") || html.contains("Col A"));
    }

    #[test]
    fn test_slugify() {
        assert_eq!(slugify("Hello World"), "hello-world");
        assert_eq!(slugify("Section 1.2"), "section-12");
        assert_eq!(slugify("  Leading spaces  "), "leading-spaces");
    }

    #[test]
    fn test_scan_nonexistent_dir() {
        let result = scan_markdown_files(Path::new("/nonexistent/path/xyz"));
        assert!(result.is_err());
    }

    #[test]
    fn test_scan_empty_dir() {
        let dir = tempfile::tempdir().unwrap();
        let result = scan_markdown_files(dir.path());
        assert!(result.is_err()); // No .md or .yaml files
    }

    #[test]
    fn test_scan_with_md_files() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("README.md"), "# Hello").unwrap();
        std::fs::write(dir.path().join("notes.md"), "## Notes").unwrap();

        let files = scan_markdown_files(dir.path()).unwrap();
        assert_eq!(files.len(), 2);
        assert!(files.iter().all(|f| f.file_type == FileType::Markdown));
    }

    #[test]
    fn test_scan_with_yaml_files() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("config.yaml"), "key: value\n").unwrap();
        std::fs::write(dir.path().join("settings.yml"), "debug: true\n").unwrap();

        let files = scan_markdown_files(dir.path()).unwrap();
        assert_eq!(files.len(), 2);
        assert!(files.iter().all(|f| f.file_type == FileType::Yaml));
    }

    #[test]
    fn test_scan_mixed_files() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("README.md"), "# Hello").unwrap();
        std::fs::write(dir.path().join("config.yaml"), "key: value\n").unwrap();
        std::fs::write(dir.path().join("settings.yml"), "debug: true\n").unwrap();

        let files = scan_markdown_files(dir.path()).unwrap();
        assert_eq!(files.len(), 3);
        let md_count = files.iter().filter(|f| f.file_type == FileType::Markdown).count();
        let yaml_count = files.iter().filter(|f| f.file_type == FileType::Yaml).count();
        assert_eq!(md_count, 1);
        assert_eq!(yaml_count, 2);
    }

    #[test]
    fn test_render_yaml_code_block() {
        let yaml = "server:\n  port: 8080\n  host: localhost\n";
        let (html, headings) = render_yaml(yaml);
        // Must be a syntax-highlighted code block
        assert!(html.contains("<pre>") && html.contains("<code"));
        assert!(html.contains("language-yaml"));
        // YAML content must be escaped and present
        assert!(html.contains("server:"));
        assert!(html.contains("port: 8080"));
        // YAML files have no headings
        assert!(headings.is_empty());
    }

    #[test]
    fn test_render_yaml_escapes_html() {
        let yaml = "message: <hello> & \"world\"\n";
        let (html, _) = render_yaml(yaml);
        // Special chars must be HTML-escaped
        assert!(html.contains("&lt;hello&gt;"));
        assert!(html.contains("&amp;"));
        assert!(html.contains("&quot;world&quot;"));
    }

    #[test]
    fn test_file_type_from_path() {
        use std::path::Path;
        assert_eq!(FileType::from_path(Path::new("a.md")), Some(FileType::Markdown));
        assert_eq!(FileType::from_path(Path::new("b.yaml")), Some(FileType::Yaml));
        assert_eq!(FileType::from_path(Path::new("c.yml")), Some(FileType::Yaml));
        assert_eq!(FileType::from_path(Path::new("d.txt")), None);
        assert_eq!(FileType::from_path(Path::new("no_ext")), None);
    }

    #[test]
    fn test_scan_recursive() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::create_dir(dir.path().join("sub")).unwrap();
        std::fs::write(dir.path().join("top.md"), "# Top").unwrap();
        std::fs::write(dir.path().join("sub").join("nested.md"), "# Nested").unwrap();

        let files = scan_markdown_files(dir.path()).unwrap();
        assert_eq!(files.len(), 2);
    }

    // ── rewrite_md_links ──────────────────────────────────────────────────────

    #[test]
    fn test_rewrite_md_links_simple() {
        let html = r#"<a href="./other.md">link</a>"#;
        let result = rewrite_md_links(html);
        assert_eq!(result, r#"<a href="./other.html">link</a>"#);
    }

    #[test]
    fn test_rewrite_md_links_nested_path() {
        let html = r#"<a href="sub/page.md">link</a>"#;
        let result = rewrite_md_links(html);
        assert_eq!(result, r#"<a href="sub/page.html">link</a>"#);
    }

    #[test]
    fn test_rewrite_md_links_parent_dir() {
        let html = r#"<a href="../README.md">link</a>"#;
        let result = rewrite_md_links(html);
        assert_eq!(result, r#"<a href="../README.html">link</a>"#);
    }

    #[test]
    fn test_rewrite_md_links_external_url_unchanged() {
        let html = r#"<a href="https://example.com">link</a>"#;
        let result = rewrite_md_links(html);
        assert_eq!(result, html);
    }

    #[test]
    fn test_rewrite_md_links_anchor_unchanged() {
        let html = r##"<a href="#heading">link</a>"##;
        let result = rewrite_md_links(html);
        assert_eq!(result, html);
    }

    #[test]
    fn test_rewrite_md_links_image_src_unchanged() {
        let html = r#"<img src="./image.png">"#;
        let result = rewrite_md_links(html);
        assert_eq!(result, html);
    }

    #[test]
    fn test_rewrite_md_links_multiple() {
        let html = r#"<a href="a.md">A</a> <a href="b.md">B</a>"#;
        let result = rewrite_md_links(html);
        assert_eq!(result, r#"<a href="a.html">A</a> <a href="b.html">B</a>"#);
    }

    // ── rewrite_image_paths ──────────────────────────────────────────────────

    #[test]
    fn test_rewrite_image_paths_simple() {
        let html = r#"<img src="screenshots/photo.png">"#;
        let result = rewrite_image_paths(html);
        assert_eq!(result, r#"<img src="/source/screenshots/photo.png">"#);
    }

    #[test]
    fn test_rewrite_image_paths_dot_prefix() {
        let html = r#"<img src="./images/diagram.png">"#;
        let result = rewrite_image_paths(html);
        assert_eq!(result, r#"<img src="/source/images/diagram.png">"#);
    }

    #[test]
    fn test_rewrite_image_paths_parent_dir() {
        let html = r#"<img src="../shared/image.png">"#;
        let result = rewrite_image_paths(html);
        assert_eq!(result, r#"<img src="/source/../shared/image.png">"#);
    }

    #[test]
    fn test_rewrite_image_paths_nested() {
        let html = r#"<img src="a/b/c/deep.png">"#;
        let result = rewrite_image_paths(html);
        assert_eq!(result, r#"<img src="/source/a/b/c/deep.png">"#);
    }

    #[test]
    fn test_rewrite_image_paths_absolute_url_unchanged() {
        let html = r#"<img src="https://example.com/image.png">"#;
        let result = rewrite_image_paths(html);
        assert_eq!(result, html);
    }

    #[test]
    fn test_rewrite_image_paths_protocol_relative_unchanged() {
        let html = r#"<img src="//cdn.example.com/image.png">"#;
        let result = rewrite_image_paths(html);
        assert_eq!(result, html);
    }

    #[test]
    fn test_rewrite_image_paths_data_uri_unchanged() {
        let html = r#"<img src="data:image/png;base64,abc123">"#;
        let result = rewrite_image_paths(html);
        assert_eq!(result, html);
    }

    #[test]
    fn test_rewrite_image_paths_already_prefixed_unchanged() {
        let html = r#"<img src="/source/screenshots/photo.png">"#;
        let result = rewrite_image_paths(html);
        assert_eq!(result, html);
    }

    #[test]
    fn test_rewrite_image_paths_multiple() {
        let html = r#"<img src="a.png"> <img src="b.jpg">"#;
        let result = rewrite_image_paths(html);
        assert_eq!(result, r#"<img src="/source/a.png"> <img src="/source/b.jpg">"#);
    }
}
