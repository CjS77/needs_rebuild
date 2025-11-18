# needs_rebuild

Checks whether a project needs to be rebuilt based on the modification times of its source files and build artifacts.

## Usage

This crate is generally most useful in build scripts (`build.rs`) to conditionally trigger rebuilds.

For example, to check if a rebuild is necessary for C source files:
```rust
use needs_rebuild::{needs_rebuild, ScanOptions};
fn main() {
    const OUTPUT_FILE: &str = "build/lib/liboutput.a";
    let options = ScanOptions::new(&["*.c", "*.h"]);
    if needs_rebuild("libs/my_c_lib", OUTPUT_FILE, options) {
        rebuild_c_library();
    }
}
```
