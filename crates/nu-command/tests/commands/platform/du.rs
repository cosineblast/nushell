use nu_test_support::{nu, pipeline};

#[test]
fn test_du_flag_min_size() {
    let actual = nu!(
        cwd: "tests/fixtures/formats", pipeline(
        r#"
            du -m -1
        "#
    ));
    assert!(actual
        .err
        .contains("Negative value passed when positive one is required"));

    let actual = nu!(
        cwd: "tests/fixtures/formats", pipeline(
        r#"
            du -m 1
        "#
    ));
    assert!(actual.err.is_empty());
}

#[test]
fn test_du_flag_max_depth() {
    let actual = nu!(
        cwd: "tests/fixtures/formats", pipeline(
        r#"
            du -d -2
        "#
    ));
    assert!(actual
        .err
        .contains("Negative value passed when positive one is required"));

    let actual = nu!(
        cwd: "tests/fixtures/formats", pipeline(
        r#"
            du -d 2
        "#
    ));
    assert!(actual.err.is_empty());
}

#[test]
fn test_du_empty_dir_has_zero_apparent_size() {
    let actual = nu!(
        cwd: "tests/fixtures/file_sizes", pipeline(
        r#"
            du -d 1 empty_dir | get apparent | get 0 | into int | into string
        "#
    ));

    assert_eq!(actual.out, "0");
}
