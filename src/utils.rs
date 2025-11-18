use filetime::set_file_mtime;
use std::path::Path;
use std::{fs, io};

/// Similar to the unix `touch` command, this function updates the timestamp of a file to the current time.
/// If the file does not exist, it is created if the parent directory exists. If the file does not exist and the
/// parent directory does not exist, an error is returned.
pub fn touch(path: impl AsRef<Path>) -> io::Result<()> {
    let path = path.as_ref();
    if path.exists() {
        let now = filetime::FileTime::now();
        set_file_mtime(path, now)?;
    } else {
        fs::write(path, b"")?;
    }
    Ok(())
}
