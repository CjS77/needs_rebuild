#[derive(Debug, Clone)]
pub struct ScanOptions {
    /// Glob patterns to include files and directories.
    pub(crate) patterns: Vec<String>,
    /// Logs the names of files and directories as they are processed.
    pub verbose: bool,
    /// Follow symbolic links. By default, this is disabled.
    ///
    /// When true, symbolic links are followed as if they were normal directories and files. If a symbolic link is
    /// broken or is involved in a loop, an error is yielded.
    pub follow_links: bool,
    /// Follow symbolic links if these are the root of the traversal. By default, this is enabled.
    ///
    /// When true, symbolic links on root paths are followed which is effective if the symbolic link points to
    /// a directory. If a symbolic link is broken or is involved in a loop, an error is yielded as the first entry
    /// of the traversal.
    pub follow_root_links: bool,
    /// Set the maximum depth of entries yield by the source scanner.
    pub(crate) max_depth: Option<usize>,
    /// Set the maximum number of simultaneously open file descriptors used by the iterator.
    ///
    /// The value must be greater than or equal to 1. If n is 0, then it is set to 1 automatically. If this is not set,
    /// then it defaults to some reasonably low number.
    pub(crate) max_open_files: Option<usize>,
    /// Do not cross file system boundaries.
    ///
    /// When this option is enabled, directory traversal will not descend into directories that are on a different
    /// file system from the root path.
    ///
    /// Currently, this option is only supported on Unix and Windows. If this option is used on an unsupported
    /// platform, then directory traversal will immediately return an error and will not yield any entries.
    pub same_file_system: bool,
}

impl Default for ScanOptions {
    fn default() -> Self {
        Self {
            patterns: vec!["**/*".to_string()],
            verbose: false,
            follow_links: false,
            follow_root_links: true,
            max_depth: None,
            max_open_files: None,
            same_file_system: false,
        }
    }
}

impl ScanOptions {
    /// Creates a new `ScanOptions` with the specified patterns. Only files matching the given glob patterns
    /// will be included in the scan.
    pub fn new(patterns: &[&str]) -> Self {
        let patterns = patterns.iter().map(|s| s.to_string()).collect();
        Self {
            patterns,
            ..Default::default()
        }
    }

    pub fn patterns(&mut self, patterns: &[&str]) {
        self.patterns = patterns.iter().map(|s| s.to_string()).collect();
    }

    pub fn max_depth(&mut self, value: usize) {
        self.max_depth = Some(value);
    }

    pub fn max_open_files(&mut self, value: usize) {
        self.max_open_files = Some(value);
    }
}
