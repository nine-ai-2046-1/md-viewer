/// main.rs - md-viewer CLI entry point
///
/// Orchestrates: asset extraction, markdown scanning, HTML generation,
/// HTTP server startup, and browser auto-open.

mod assets;
mod builder;
mod port;
mod sanitize;
mod server;
mod templates;

use clap::Parser;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

use builder::FileType;

// ── CLI definition ────────────────────────────────────────────────────────────

/// md-viewer — View markdown and YAML files as beautiful HTML pages in your browser.
///
/// Usage examples:
///   md-viewer --md README.md
///   md-viewer --mds ./docs --port 9141
///   md-viewer --md README.md --build ./dist
///   md-viewer --mds ./docs --build ./dist
///   md-viewer --serve ./dist --port 9141
#[derive(Parser, Debug)]
#[command(name = "md-viewer", version,
    about = "A self-contained CLI to view markdown and YAML files as beautiful HTML pages",
    long_about = None)]
struct Cli {
    /// Single markdown or YAML file to view (.md / .yaml / .yml).
    /// Use with --build to export to a static site instead of serving.
    #[arg(long, conflicts_with = "mds", value_name = "FILE")]
    md: Option<PathBuf>,

    /// Directory of markdown and YAML files to view (recursive scan).
    /// Use with --build to export to a static site instead of serving.
    #[arg(long, conflicts_with = "md", value_name = "DIR")]
    mds: Option<PathBuf>,

    /// Build output directory: render sources to a self-contained static site and exit.
    /// Requires --md or --mds. Does not start a web server.
    #[arg(long, conflicts_with = "serve", value_name = "OUTPUT_DIR")]
    build: Option<PathBuf>,

    /// Serve a pre-built static directory over localhost HTTP.
    /// Requires index.html to exist in the directory. Does not build anything.
    #[arg(long, conflicts_with_all = ["md", "mds", "build"], value_name = "STATIC_DIR")]
    serve: Option<PathBuf>,

    /// Port to serve on. Auto-increments if occupied.
    #[arg(long, default_value = "9141")]
    port: u16,
}

// ── Main ──────────────────────────────────────────────────────────────────────

fn main() {
    let cli = Cli::parse();

    // --serve is mutually exclusive with --md/--mds/--build (enforced by clap conflicts_with_all).
    // --build requires --md or --mds.
    // Without --serve, at least one of --md / --mds must be provided.
    if cli.serve.is_none() && cli.md.is_none() && cli.mds.is_none() {
        eprintln!("Error: specify --md <file>, --mds <directory>, or --serve <dir>");
        eprintln!("Run `md-viewer --help` for usage.");
        std::process::exit(1);
    }

    if cli.build.is_some() && cli.md.is_none() && cli.mds.is_none() {
        eprintln!("Error: --build requires --md <file> or --mds <directory>");
        eprintln!("Run `md-viewer --help` for usage.");
        std::process::exit(1);
    }

    if let Err(e) = run(cli) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn run(cli: Cli) -> Result<(), String> {
    // ── --serve mode: serve pre-built static dir, no building ────────────────
    if let Some(serve_dir) = &cli.serve {
        return run_serve_mode(serve_dir, cli.port);
    }

    // ── --build mode: render to output dir and exit (no server) ─────────────
    if let Some(output_dir) = &cli.build {
        return run_build_mode(cli.md.as_deref(), cli.mds.as_deref(), output_dir);
    }

    // ── Standard serve mode: build to ~/.opens/md-viewer-data/ then serve ──
    let root = assets::resolve_root()?;
    let base_dir = root.join("base");
    let pages_dir = assets::ensure_pages_dir(&root)?;

    // Always re-extract embedded assets so CSS/JS/templates are never stale
    // (this is fast: ~200 KB of small files)
    assets::ensure_assets(&base_dir)?;

    // Read the template files (freshly extracted above)
    let single_template = read_template(&base_dir.join("single.html"))?;
    let multi_template = read_template(&base_dir.join("multi.html"))?;

    let index_file = if let Some(md_path) = &cli.md {
        run_single_mode(
            md_path,
            &pages_dir,
            &single_template,
        )?
    } else if let Some(dir_path) = &cli.mds {
        run_folder_mode(
            dir_path,
            &pages_dir,
            &base_dir,
            &multi_template,
        )?
    } else {
        unreachable!()
    };

    // source_dir: only set for --mds mode (for serving images/videos from source)
    let source_dir = cli.mds.clone();

    // Find available port
    let actual_port = port::find_available_port(cli.port)?;
    if actual_port != cli.port {
        eprintln!("Port {} in use, using port {} instead", cli.port, actual_port);
    }

    let url = format!("http://127.0.0.1:{}", actual_port);
    eprintln!("Serving at {}", url);

    // Open browser
    port::open_browser(&url);

    // Start server (blocks)
    server::start(&server::ServerConfig {
        port: actual_port,
        root_dir: root,
        base_dir,
        pages_dir,
        index_file,
        source_dir,
    })
}

// ── Build mode ────────────────────────────────────────────────────────────────

/// Render source files to a self-contained static site in `output_dir` and exit.
///
/// Copies embedded assets to `output_dir/base/`, renders each source file to
/// `output_dir/<name>.html`, and generates `output_dir/index.html`.
/// Does NOT start a web server.
///
/// # Arguments
/// * `md`         - Optional single source file path (from --md).
/// * `mds`        - Optional source directory path (from --mds).
/// * `output_dir` - Destination directory for the static build.
fn run_build_mode(
    md: Option<&Path>,
    mds: Option<&Path>,
    output_dir: &Path,
) -> Result<(), String> {
    // Create output directory and parents
    std::fs::create_dir_all(output_dir)
        .map_err(|e| format!("Cannot create output dir {}: {}", output_dir.display(), e))?;

    // Copy all embedded assets to output_dir/base/
    let out_base = output_dir.join("base");
    assets::copy_assets_to(&out_base)?;
    eprintln!("Assets written to {}", out_base.display());

    // Load templates directly from embedded assets (no disk extraction needed)
    let single_template = get_embedded_template("single.html")?;
    let multi_template = get_embedded_template("multi.html")?;

    if let Some(md_path) = md {
        // Single-file build
        build_single_to_dir(md_path, output_dir, &single_template)?;
    } else if let Some(dir_path) = mds {
        // Folder build
        build_folder_to_dir(dir_path, output_dir, &multi_template)?;
    } else {
        return Err("--build requires --md or --mds".to_string());
    }

    eprintln!("Build complete: {}", output_dir.display());
    Ok(())
}

/// Build a single source file into `output_dir`.
///
/// Writes `output_dir/<name>.html` and a minimal `output_dir/index.html` redirect.
fn build_single_to_dir(
    md_path: &Path,
    output_dir: &Path,
    template: &str,
) -> Result<(), String> {
    if !md_path.exists() {
        return Err(format!("File not found: {}", md_path.display()));
    }

    let file_type = FileType::from_path(md_path).ok_or_else(|| {
        format!(
            "Unsupported file type: {}. Supported: .md, .yaml, .yml",
            md_path.display()
        )
    })?;

    let html_name = sanitize::md_path_to_html_name(md_path);
    let html_output = output_dir.join(&html_name);

    eprintln!("Building {}", html_name);
    build_single_file_html(md_path, &file_type, &html_output, template)?;

    // Generate a minimal index.html that redirects to the rendered page
    let index_html = format!(
        r#"<!DOCTYPE html>
<html>
<head>
  <meta charset="UTF-8">
  <meta http-equiv="refresh" content="0; url=/{html_name}">
  <title>Redirecting…</title>
</head>
<body>
  <p><a href="/{html_name}">Click here if not redirected.</a></p>
</body>
</html>"#,
        html_name = html_name
    );

    let index_output = output_dir.join("index.html");
    std::fs::write(&index_output, index_html)
        .map_err(|e| format!("Cannot write index.html: {}", e))?;

    eprintln!("Written: {} and index.html", html_name);
    Ok(())
}

/// Build an entire folder of source files into `output_dir`.
///
/// Writes each source as `output_dir/<name>.html` and generates `output_dir/index.html`.
/// Build an entire folder of source files into `output_dir`.
///
/// Writes `output_dir/base/nav.js` with shared FILE_TREE + SEARCH_INDEX,
/// then writes each source as `output_dir/<name>.html` and generates `output_dir/index.html`.
fn build_folder_to_dir(
    dir_path: &Path,
    output_dir: &Path,
    template: &str,
) -> Result<(), String> {
    if !dir_path.exists() {
        return Err(format!("Directory not found: {}", dir_path.display()));
    }

    let md_files = builder::scan_markdown_files(dir_path)?;
    eprintln!("Found {} file(s)", md_files.len());

    // Determine site title from directory name
    let site_title = dir_path
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("Documentation")
        .to_string();

    // Build file tree JSON
    let file_tree = builder::build_file_tree(&md_files);
    let file_tree_json = serde_json::to_string(&file_tree)
        .map_err(|e| format!("Cannot serialize file tree: {}", e))?;

    // Read all file contents for search index
    let mut file_contents: Vec<(builder::MdFile, String)> = Vec::new();
    for file in &md_files {
        let content = std::fs::read_to_string(&file.source).unwrap_or_default();
        file_contents.push((file.clone(), content));
    }

    let search_index = builder::build_search_index(&file_contents);
    let search_index_json = serde_json::to_string(&search_index)
        .map_err(|e| format!("Cannot serialize search index: {}", e))?;

    // Write shared nav.js to output_dir/base/ — single source of truth for navigation data
    write_nav_js(&output_dir.join("base"), &file_tree_json, &search_index_json)?;

    let generated_at = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    // Render each document page directly into output_dir/
    for (file, md_content) in &file_contents {
        let html_output = output_dir.join(&file.html_name);
        eprintln!("Building {}", file.html_name);
        build_doc_html(
            file,
            md_content,
            &html_output,
            template,
            &site_title,
            &generated_at,
        )?;
    }

    // Build index page directly into output_dir/
    let index_output = output_dir.join("index.html");
    eprintln!("Building index.html");
    build_index_html(
        &md_files,
        &index_output,
        template,
        &site_title,
        &generated_at,
    )?;

    Ok(())
}

/// Load a template from embedded assets by filename, returning its UTF-8 content.
fn get_embedded_template(name: &str) -> Result<String, String> {
    let bytes = assets::get_asset_content(name)
        .ok_or_else(|| format!("Embedded template not found: {}", name))?;
    String::from_utf8(bytes.to_vec())
        .map_err(|e| format!("Template {} is not valid UTF-8: {}", name, e))
}

/// Write `nav.js` to `target_dir/nav.js`.
///
/// The file contains the shared FILE_TREE and SEARCH_INDEX variables used by all
/// pages in folder mode. Writing this as a single shared file (instead of baking
/// data into each page) ensures the left-menu is always consistent across all pages,
/// even when only a subset of pages is rebuilt incrementally.
///
/// # Arguments
/// * `target_dir` - Directory to write nav.js into (base_dir for serve, output/base for build).
/// * `file_tree_json`   - Serialized file tree for the sidebar.
/// * `search_index_json` - Serialized search index for fuse.js.
fn write_nav_js(
    target_dir: &Path,
    file_tree_json: &str,
    search_index_json: &str,
) -> Result<(), String> {
    std::fs::create_dir_all(target_dir)
        .map_err(|e| format!("Cannot create dir {}: {}", target_dir.display(), e))?;

    let content = format!(
        "/* nav.js — auto-generated by md-viewer; do not edit manually */\nvar FILE_TREE={file_tree};\nvar SEARCH_INDEX={search_index};\n",
        file_tree = file_tree_json,
        search_index = search_index_json,
    );

    let path = target_dir.join("nav.js");
    std::fs::write(&path, content)
        .map_err(|e| format!("Cannot write nav.js to {}: {}", path.display(), e))
}

/// Build and prepare a single markdown or YAML file for serving.
///
/// Returns the HTML filename (index_file) to pass to ServerConfig.
fn run_single_mode(
    md_path: &Path,
    pages_dir: &Path,
    template: &str,
) -> Result<String, String> {
    if !md_path.exists() {
        return Err(format!("File not found: {}", md_path.display()));
    }

    // Validate extension; reject unsupported types early
    let file_type = FileType::from_path(md_path).ok_or_else(|| {
        format!(
            "Unsupported file type: {}. Supported: .md, .yaml, .yml",
            md_path.display()
        )
    })?;

    let html_name = sanitize::md_path_to_html_name(md_path);
    let html_output = pages_dir.join(&html_name);

    // Always rebuild — no caching
    eprintln!("Building {}", html_name);
    build_single_file_html(md_path, &file_type, &html_output, template)?;

    Ok(html_name)
}

/// Convert a markdown or YAML file to HTML and write to output path.
///
/// Dispatches to the appropriate renderer based on file_type.
fn build_single_file_html(
    source_path: &Path,
    file_type: &FileType,
    output: &Path,
    template: &str,
) -> Result<(), String> {
    let content = std::fs::read_to_string(source_path)
        .map_err(|e| format!("Cannot read {}: {}", source_path.display(), e))?;

    let stem = source_path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("Document")
        .to_string();

    // Page title: "YAML: name" for YAML files, just stem for markdown
    let title = match file_type {
        FileType::Yaml => format!("YAML: {}", stem),
        FileType::Markdown => stem,
    };

    let (html_body, headings) = match file_type {
        FileType::Markdown => builder::render_markdown(&content),
        FileType::Yaml => builder::render_yaml(&content),
    };
    let toc_html = builder::build_toc(&headings);

    let generated_at = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    let mut vars: HashMap<&str, &str> = HashMap::new();
    vars.insert("TITLE", &title);
    vars.insert("CONTENT", &html_body);
    vars.insert("TOC", &toc_html);
    vars.insert("GENERATED_AT", &generated_at);

    let html = templates::render(template, &vars);

    std::fs::write(output, html)
        .map_err(|e| format!("Cannot write {}: {}", output.display(), e))
}

// ── Folder mode ───────────────────────────────────────────────────────────────

/// Build and prepare a folder of markdown files for serving.
///
/// Returns the index_file_name ("index.html").
/// Writes `base_dir/nav.js` containing shared FILE_TREE + SEARCH_INDEX so all
/// pages reference the same navigation data (prevents stale left-menu on incremental builds).
fn run_folder_mode(
    dir_path: &Path,
    pages_dir: &Path,
    base_dir: &Path,
    template: &str,
) -> Result<String, String> {
    if !dir_path.exists() {
        return Err(format!("Directory not found: {}", dir_path.display()));
    }

    // Scan all markdown and YAML files
    let md_files = builder::scan_markdown_files(dir_path)?;
    eprintln!("Found {} file(s)", md_files.len());

    // Build file tree JSON for sidebar
    let file_tree = builder::build_file_tree(&md_files);
    let file_tree_json = serde_json::to_string(&file_tree)
        .map_err(|e| format!("Cannot serialize file tree: {}", e))?;

    // Determine site title from directory name
    let site_title = dir_path
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("Documentation")
        .to_string();

    // Read all markdown content (needed for search index)
    let mut file_contents: Vec<(builder::MdFile, String)> = Vec::new();
    for file in &md_files {
        let content = std::fs::read_to_string(&file.source)
            .unwrap_or_default();
        file_contents.push((file.clone(), content));
    }

    // Build search index
    let search_index = builder::build_search_index(&file_contents);
    let search_index_json = serde_json::to_string(&search_index)
        .map_err(|e| format!("Cannot serialize search index: {}", e))?;

    // Write shared nav.js to base_dir — always fresh, shared by ALL pages.
    write_nav_js(base_dir, &file_tree_json, &search_index_json)?;

    let generated_at = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    // Build each individual markdown → HTML (always rebuild, no caching)
    for (file, md_content) in &file_contents {
        let html_output = pages_dir.join(&file.html_name);

        eprintln!("Building {}", file.html_name);
        build_doc_html(
            file,
            md_content,
            &html_output,
            template,
            &site_title,
            &generated_at,
        )?;
    }

    // Build index page
    let index_output = pages_dir.join("index.html");
    eprintln!("Building index.html");
    build_index_html(
        &md_files,
        &index_output,
        template,
        &site_title,
        &generated_at,
    )?;

    Ok("index.html".to_string())
}

/// Build HTML for a single document page in folder mode.
///
/// FILE_TREE and SEARCH_INDEX are now loaded from /base/nav.js (shared across all pages),
/// so they are no longer injected inline here.
fn build_doc_html(
    file: &builder::MdFile,
    content: &str,
    output: &Path,
    template: &str,
    site_title: &str,
    generated_at: &str,
) -> Result<(), String> {
    // Render body and extract headings based on file type
    let (html_body, headings) = match file.file_type {
        FileType::Markdown => builder::render_markdown(content),
        FileType::Yaml => builder::render_yaml(content),
    };
    let toc_html = builder::build_toc(&headings);

    // Page title: "YAML: name" for YAML files, just stem for markdown
    let display_title = match file.file_type {
        FileType::Yaml => format!("YAML: {}", file.title),
        FileType::Markdown => file.title.clone(),
    };

    let mut vars: HashMap<&str, &str> = HashMap::new();
    vars.insert("TITLE", &display_title);
    vars.insert("SITE_TITLE", site_title);
    vars.insert("CONTENT", &html_body);
    vars.insert("TOC", &toc_html);
    vars.insert("GENERATED_AT", generated_at);
    vars.insert("CURRENT_PAGE", &file.url_path);
    vars.insert("FOLDER_LABEL", site_title);
    vars.insert("FILE_COUNT", "");
    vars.insert("FILE_CARDS", "");

    let html = templates::render_with_conditionals(template, &vars, false);

    std::fs::write(output, html)
        .map_err(|e| format!("Cannot write {}: {}", output.display(), e))
}

/// Build the index/home page for folder mode.
///
/// FILE_TREE and SEARCH_INDEX are loaded from /base/nav.js (shared file).
fn build_index_html(
    files: &[builder::MdFile],
    output: &Path,
    template: &str,
    site_title: &str,
    generated_at: &str,
) -> Result<(), String> {
    // Build file cards HTML
    let card_data: Vec<(String, String, String)> = files
        .iter()
        .map(|f| (
            f.title.clone(),
            f.url_path.clone(),
            f.relative.to_string_lossy().to_string(),
        ))
        .collect();

    let file_cards = templates::build_file_cards(&card_data);
    let file_count = files.len().to_string();

    let mut vars: HashMap<&str, &str> = HashMap::new();
    vars.insert("TITLE", site_title);
    vars.insert("SITE_TITLE", site_title);
    vars.insert("CONTENT", "");
    vars.insert("TOC", "");
    vars.insert("GENERATED_AT", generated_at);
    vars.insert("CURRENT_PAGE", "/");
    vars.insert("FOLDER_LABEL", site_title);
    vars.insert("FILE_COUNT", &file_count);
    vars.insert("FILE_CARDS", &file_cards);

    let html = templates::render_with_conditionals(template, &vars, true);

    std::fs::write(output, html)
        .map_err(|e| format!("Cannot write {}: {}", output.display(), e))
}

// ── Serve mode ────────────────────────────────────────────────────────────────

/// Validate that `serve_dir` is suitable for --serve: exists and contains index.html.
///
/// Returns `Ok(())` on success, `Err(message)` otherwise.
/// Extracted as a testable function separate from port-binding/browser-open side effects.
pub fn validate_serve_dir(serve_dir: &Path) -> Result<(), String> {
    if !serve_dir.exists() {
        return Err(format!(
            "Serve directory not found: {}",
            serve_dir.display()
        ));
    }
    let index = serve_dir.join("index.html");
    if !index.exists() {
        return Err(format!(
            "No index.html found in {}. Run with --build first.",
            serve_dir.display()
        ));
    }
    Ok(())
}

/// Serve a pre-built static directory over localhost HTTP.
///
/// Validates the directory has index.html, then finds an available port,
/// opens the browser, and starts the blocking HTTP server.
fn run_serve_mode(serve_dir: &Path, requested_port: u16) -> Result<(), String> {
    // Validate before binding to any port
    validate_serve_dir(serve_dir)?;

    // Resolve to absolute path for security anchor
    let root_dir = serve_dir
        .canonicalize()
        .map_err(|e| format!("Cannot resolve serve dir {}: {}", serve_dir.display(), e))?;

    let base_dir = root_dir.join("base");
    // For --serve, HTML files live directly in root_dir (not a pages/ subdir)
    let pages_dir = root_dir.clone();

    let actual_port = port::find_available_port(requested_port)?;
    if actual_port != requested_port {
        eprintln!("Port {} in use, using port {} instead", requested_port, actual_port);
    }

    let url = format!("http://127.0.0.1:{}", actual_port);
    eprintln!("Serving {} at {}", root_dir.display(), url);

    port::open_browser(&url);

    server::start(&server::ServerConfig {
        port: actual_port,
        root_dir,
        base_dir,
        pages_dir,
        index_file: "index.html".to_string(),
        source_dir: None,
    })
}

// ── Helpers ───────────────────────────────────────────────────────────────────

/// Read a template file from disk.
fn read_template(path: &Path) -> Result<String, String> {
    std::fs::read_to_string(path)
        .map_err(|e| format!("Cannot read template {}: {}", path.display(), e))
}

// ── Unit Tests ────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    // ── validate_serve_dir ────────────────────────────────────────────────────

    /// validate_serve_dir rejects a directory that does not exist.
    #[test]
    fn test_validate_serve_dir_missing_dir() {
        let result = validate_serve_dir(Path::new("/tmp/md-viewer-nonexistent-dir-xyz"));
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("not found"));
    }

    /// validate_serve_dir rejects a directory that exists but has no index.html.
    #[test]
    fn test_validate_serve_dir_missing_index() {
        let tmp = std::env::temp_dir().join("md-viewer-serve-no-index");
        fs::create_dir_all(&tmp).unwrap();
        // Remove index.html if it exists from a prior run
        let _ = fs::remove_file(tmp.join("index.html"));

        let result = validate_serve_dir(&tmp);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("index.html"));

        let _ = fs::remove_dir_all(&tmp);
    }

    /// validate_serve_dir returns Ok when directory and index.html both exist.
    #[test]
    fn test_validate_serve_dir_valid() {
        let tmp = std::env::temp_dir().join("md-viewer-serve-valid");
        fs::create_dir_all(&tmp).unwrap();
        fs::write(tmp.join("index.html"), "<html></html>").unwrap();

        let result = validate_serve_dir(&tmp);
        assert!(result.is_ok());

        let _ = fs::remove_dir_all(&tmp);
    }

    // ── build_single_to_dir ───────────────────────────────────────────────────

    /// Single-file build writes <name>.html and an index.html redirect.
    #[test]
    fn test_build_single_to_dir_generates_redirect_index() {
        let tmp = std::env::temp_dir().join("md-viewer-build-single");
        let _ = fs::remove_dir_all(&tmp);
        fs::create_dir_all(&tmp).unwrap();

        // Create a minimal markdown source
        let src = tmp.join("hello.md");
        fs::write(&src, "# Hello\nWorld").unwrap();

        let template = get_embedded_template("single.html").unwrap();
        build_single_to_dir(&src, &tmp, &template).unwrap();

        let hello_html = tmp.join("hello.html");
        let index_html = tmp.join("index.html");

        assert!(hello_html.exists(), "hello.html should exist");
        assert!(index_html.exists(), "index.html should exist");

        let index_content = fs::read_to_string(&index_html).unwrap();
        assert!(
            index_content.contains("hello.html"),
            "index.html should redirect to hello.html"
        );

        let _ = fs::remove_dir_all(&tmp);
    }

    // ── always rebuild behavior ───────────────────────────────────────────────

    /// Always rebuild: run_single_mode rebuilds even when HTML already exists.
    #[test]
    fn test_always_rebuild() {
        let tmp = std::env::temp_dir().join("md-viewer-always-rebuild");
        let _ = fs::remove_dir_all(&tmp);
        fs::create_dir_all(&tmp).unwrap();

        let md = tmp.join("doc.md");
        fs::write(&md, "# Version 1\nOriginal content.").unwrap();

        let template = get_embedded_template("single.html").unwrap();

        // First call: builds HTML
        run_single_mode(&md, &tmp, &template).unwrap();
        let html_path = tmp.join("doc.html");
        assert!(html_path.exists(), "First call should build doc.html");
        let content_v1 = fs::read_to_string(&html_path).unwrap();
        assert!(content_v1.contains("Version 1"), "HTML should contain v1 heading");

        // Modify source
        fs::write(&md, "# Version 2\nUpdated content.").unwrap();

        // Second call: always rebuilds even without cache
        run_single_mode(&md, &tmp, &template).unwrap();
        let content_v2 = fs::read_to_string(&html_path).unwrap();
        assert!(content_v2.contains("Version 2"), "HTML should be rebuilt with v2 heading");

        let _ = fs::remove_dir_all(&tmp);
    }

    // ── build_folder_to_dir ───────────────────────────────────────────────────

    /// Folder build generates index.html with file cards for all source files.
    #[test]
    fn test_build_folder_to_dir_generates_index_with_cards() {
        let tmp = std::env::temp_dir().join("md-viewer-build-folder");
        let _ = fs::remove_dir_all(&tmp);

        let src_dir = tmp.join("docs");
        fs::create_dir_all(&src_dir).unwrap();

        let out_dir = tmp.join("out");
        fs::create_dir_all(&out_dir).unwrap();

        fs::write(src_dir.join("alpha.md"), "# Alpha\nContent A").unwrap();
        fs::write(src_dir.join("beta.md"), "# Beta\nContent B").unwrap();

        let template = get_embedded_template("multi.html").unwrap();
        build_folder_to_dir(&src_dir, &out_dir, &template).unwrap();

        let index_html = out_dir.join("index.html");
        assert!(index_html.exists(), "index.html should be generated");

        let content = fs::read_to_string(&index_html).unwrap();
        // Both file titles should appear somewhere in the index
        assert!(content.contains("Alpha") || content.contains("alpha"),
            "index.html should mention alpha doc");
        assert!(content.contains("Beta") || content.contains("beta"),
            "index.html should mention beta doc");

        let _ = fs::remove_dir_all(&tmp);
    }
}
