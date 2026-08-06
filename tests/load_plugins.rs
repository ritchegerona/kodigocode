use std::fs;
use std::path::Path;

#[test]
fn load_plugins_empty_dir() {
    let tmp_dir = std::env::temp_dir().join("openclaude_test_plugins");
    let _ = fs::remove_dir_all(&tmp_dir);
    fs::create_dir_all(&tmp_dir).expect("failed to create temp dir");

    let plugins =
        kodigocode::plugin::load_plugins(Path::new(&tmp_dir)).expect("load_plugins failed");
    assert!(
        plugins.is_empty(),
        "Expected no plugins in empty directory"
    );

    let _ = fs::remove_dir_all(&tmp_dir);
}