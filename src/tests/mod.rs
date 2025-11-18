use crate::utils::touch;
use crate::{ScanOptions, needs_rebuild};
use log::info;
#[test]
fn new_file() {
    env_logger::try_init().ok();
    const OUTPUT_FILE: &str = "test_files/output/output.txt";

    info!("First check: target is newer than source files. No rebuild needed.");
    touch(OUTPUT_FILE).unwrap();
    let mut options = ScanOptions::default();
    options.verbose = true;
    let rebuild_needed = needs_rebuild("test_files", OUTPUT_FILE, options).expect("rebuild");
    assert!(!rebuild_needed);

    info!("Second check: create a new source file. Rebuild needed.");
    let mut options = ScanOptions::default();
    options.verbose = true;
    touch("test_files/new.foo").unwrap();
    let rebuild_needed =
        needs_rebuild("test_files", OUTPUT_FILE, options.clone()).expect("rebuild");
    assert!(rebuild_needed);

    info!("Third check: filter out .foo files. No rebuild needed.");
    options.patterns(&["**/*.txt", "**/*.cpp"]);
    let rebuild_needed = needs_rebuild("test_files", OUTPUT_FILE, options).expect("rebuild");
    assert!(!rebuild_needed);
}

#[test]
fn target_does_not_exist() {
    env_logger::try_init().ok();
    const OUTPUT_FILE: &str = "test_files/output/non_existent_output.txt";
    let options = ScanOptions::default();
    let rebuild_needed = needs_rebuild("test_files", OUTPUT_FILE, options).expect("rebuild");
    assert!(rebuild_needed);
}

#[test]
fn filter_patterns() {
    env_logger::try_init().ok();
    const OUTPUT_FILE: &str = "test_files/output/output.txt";
    let mut options = ScanOptions::default();
    options.patterns(&["*.c"]);
    options.verbose = true;
    info!("Only checking for .c files.");
    let rebuild_needed = needs_rebuild("test_files", OUTPUT_FILE, options).expect("rebuild");
    // Only a few files are tested
    assert!(!rebuild_needed);
}
