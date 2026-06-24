/// assets.rs - Embedded asset extraction and base directory management
///
/// Embeds all frontend assets (CSS, JS, HTML templates) into the binary at
/// compile time using include_dir!. Extracts them to ~/.opens/md-viewer-data/base/
/// unconditionally on every run, so the on-disk files always match the binary.

use include_dir::{include_dir, Dir};
use std::path::Path;

/// All frontend assets embedded at compile time.
static ASSETS_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/assets");

/// Extract all embedded assets to `base_dir`, always overwriting existing files.
///
/// This is unconditional: every run re-writes all assets so that a new binary
/// (updated CSS, JS, or templates) is always reflected on disk immediately.
/// The assets are small (~200 KB total) so the overhead is negligible.
///
/// # Arguments
/// * `base_dir` - Path to ~/.opens/md-viewer-data/base/
pub fn ensure_assets(base_dir: &Path) -> Result<(), String> {
    std::fs::create_dir_all(base_dir)
        .map_err(|e| format!("Cannot create base dir {}: {}", base_dir.display(), e))?;

    for file in ASSETS_DIR.files() {
        let dest = base_dir.join(file.path());

        // Ensure parent directory exists (handles nested asset paths)
        if let Some(parent) = dest.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| format!("Cannot create dir {}: {}", parent.display(), e))?;
        }

        std::fs::write(&dest, file.contents())
            .map_err(|e| format!("Cannot write asset {}: {}", dest.display(), e))?;
    }

    Ok(())
}

/// Return the path to the `pages` directory, creating it if needed.
pub fn ensure_pages_dir(root: &Path) -> Result<std::path::PathBuf, String> {
    let pages = root.join("pages");
    std::fs::create_dir_all(&pages)
        .map_err(|e| format!("Cannot create pages dir: {}", e))?;
    Ok(pages)
}

/// Copy all embedded assets to `target_base`, overwriting any existing files.
///
/// Used by `--build` mode to write assets into the output directory's `base/` subfolder.
/// Delegates to `ensure_assets` since both operations are now unconditional overwrites.
///
/// # Arguments
/// * `target_base` - Destination directory for all asset files (created if absent).
pub fn copy_assets_to(target_base: &Path) -> Result<(), String> {
    ensure_assets(target_base)
}

/// Return the raw bytes of an embedded asset file by its name (e.g. "single.html").
///
/// Returns `None` if the named file is not found in the embedded asset directory.
pub fn get_asset_content(name: &str) -> Option<&'static [u8]> {
    ASSETS_DIR.get_file(name).map(|f| f.contents())
}

/// Resolve the md-viewer root directory: ~/.opens/md-viewer-data/
pub fn resolve_root() -> Result<std::path::PathBuf, String> {
    let home = std::env::var("HOME")
        .map(std::path::PathBuf::from)
        .or_else(|_| {
            // Fallback: use current dir
            std::env::current_dir().map_err(|e| e.to_string())
        })
        .map_err(|e| format!("Cannot determine home directory: {}", e))?;
    Ok(home.join(".opens").join("md-viewer-data"))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verify copy_assets_to writes all required asset files to a temp directory.
    #[test]
    fn test_copy_assets_to_writes_all_required_files() {
        let tmp = std::env::temp_dir().join("md-viewer-assets-test");
        let _ = std::fs::remove_dir_all(&tmp); // clean up any previous run

        copy_assets_to(&tmp).expect("copy_assets_to should succeed");

        let required = &[
            "style.css", "prism.css", "prism.js", "app.js",
            "sidebar.js", "fuse.js", "single.html", "multi.html",
        ];
        for name in required {
            let path = tmp.join(name);
            assert!(path.exists(), "Expected asset file missing: {}", name);
            let size = std::fs::metadata(&path).unwrap().len();
            assert!(size > 0, "Expected asset file is empty: {}", name);
        }

        let _ = std::fs::remove_dir_all(&tmp);
    }

    /// Verify get_asset_content returns Some for known files and None for unknown.
    #[test]
    fn test_get_asset_content_known_and_unknown() {
        assert!(
            get_asset_content("single.html").is_some(),
            "single.html should be present in embedded assets"
        );
        assert!(
            get_asset_content("multi.html").is_some(),
            "multi.html should be present in embedded assets"
        );
        assert!(
            get_asset_content("does-not-exist.xyz").is_none(),
            "Unknown asset should return None"
        );
    }
}
