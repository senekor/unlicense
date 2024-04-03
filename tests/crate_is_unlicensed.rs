#[test]
fn cargo_toml_contains_unlicense() {
    let cargo_toml = include_str!("../Cargo.toml");
    assert!(cargo_toml.contains(r#"license = "Unlicense""#));
}

#[test]
fn unlicense_file_matches_library() {
    let unlicense_file = include_str!("../UNLICENSE");
    assert_eq!(unlicense_file, unlicense::TEXT);
}
