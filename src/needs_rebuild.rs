use crate::errors::NeedsRebuildError;
use crate::options::ScanOptions;
use glob::Pattern;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

/// Checks if any source files matching the given patterns are newer than the output file.
///
/// # Arguments
/// * `source_dir` - The directory to search for source files.
/// * `output_file` - The file to check against (e.g., the compiled library).
/// * `patterns` - A slice of file patterns (e.g., &["*.cpp", "*.hpp"]).
///
/// # Returns
/// `true` if a rebuild is needed, `false` otherwise.
pub fn needs_rebuild(
    source_dir: impl AsRef<Path>,
    asset: impl AsRef<Path>,
    options: ScanOptions,
) -> Result<bool, NeedsRebuildError> {
    // If the output file doesn't exist, we definitely need to build.
    let target = asset.as_ref();
    if !target.exists() {
        return Ok(true);
    }
    // Get the last modified time of the output file.
    let output_modified_time = fs::metadata(target)?.modified()?;
    println!(
        "{}Output {} modified at: {:?}",
        options.log_prefix,
        target.display(),
        output_modified_time
    );

    // Compile the glob patterns for efficiency.
    let compiled_patterns = options
        .patterns
        .iter()
        .map(|p| Pattern::new(p))
        .collect::<Result<Vec<_>, _>>()?;

    // Walk the source directory and check for any file that is newer.
    let mut walker = WalkDir::new(source_dir.as_ref())
        .follow_links(options.follow_links)
        .follow_root_links(options.follow_root_links)
        .same_file_system(options.same_file_system);
    if let Some(depth) = options.max_depth {
        walker = walker.max_depth(depth)
    }
    if let Some(max_open) = options.max_open_files {
        walker = walker.max_open(max_open)
    }
    for entry in walker {
        let entry = entry?;
        let path = entry.path();

        // Skip directories
        if path.is_dir() {
            continue;
        }

        // Check if the file matches any of the provided patterns
        let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");

        if !compiled_patterns.iter().any(|p| p.matches(file_name)) {
            continue;
        }

        // Compare timestamps
        let source_modified_time = entry.metadata()?.modified()?;
        let is_newer = source_modified_time > output_modified_time;

        if options.verbose {
            let status = if is_newer { "CHANGED" } else { "Ok" };
            println!("{}{file_name}: {status}", options.log_prefix);
        }

        if is_newer {
            return Ok(true);
        }
    }
    Ok(false)
}
