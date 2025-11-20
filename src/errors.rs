use thiserror::Error;

#[derive(Debug, Error)]
pub enum NeedsRebuildError {
    #[error("Could not complete rebuild detection due to an I/O error: {0}")]
    IOError(#[from] std::io::Error),
    #[error("Invalid glob pattern: {0}")]
    PatternError(#[from] globset::Error),
    #[error("Error walking directory: {0}")]
    WalkDirError(#[from] walkdir::Error),
}
