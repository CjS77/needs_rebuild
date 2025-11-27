use crate::errors::NeedsRebuildError;
use crate::options::ScanOptions;
use globset::{Glob, GlobSetBuilder};
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
    if options.verbose {
        println!(
            "{}Output {} modified at: {:?}",
            options.log_prefix,
            target.display(),
            output_modified_time
        );
    }

    // Compile the glob patterns for efficiency.
    let mut builder = GlobSetBuilder::new();
    let compiled_patterns = options
        .patterns
        .iter()
        .map(|p| Glob::new(p))
        .collect::<Result<Vec<_>, _>>()?;
    compiled_patterns.into_iter().for_each(|pattern| {
        builder.add(pattern);
    });
    let set = builder.build()?;

    if options.verbose {
        println!("{}Checking source files for updates", options.log_prefix);
    }
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

        if !set.is_match(path) {
            continue;
        }

        // Compare timestamps
        let source_modified_time = entry.metadata()?.modified()?;
        let is_newer = source_modified_time > output_modified_time;

        if options.verbose && is_newer {
            println!("{}{}: CHANGED", options.log_prefix, path.display());
        }

        if is_newer {
            return Ok(true);
        }
    }
    Ok(false)
}
