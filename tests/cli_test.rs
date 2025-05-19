use anyhow::Result;
use assert_cmd::Command;
use pretty_assertions::assert_eq;
use std::fs;
use tempfile::TempDir;

static INPUT_CONTENT_LOCATION: &str = "tests/file_fixtures/inputs";
static INPUT_POST_LOCATION: &str = "posts";
static OUTPUT_POST_LOCATION: &str = "tests/file_fixtures/outputs";

fn compare_files(file1: &str, file2: &str) -> Result<()> {
    let file1_content = fs::read_to_string(file1)?;
    let file2_content = fs::read_to_string(file2)?;
    assert_eq!(file1_content, file2_content, "{} != {}", file1, file2);
    Ok(())
}

fn compare_dirs(ground_truth: &str, output: &str) -> Result<()> {
    let mut ground_truth_files = fs::read_dir(ground_truth)?;
    while let Some(file) = ground_truth_files.next() {
        let file = file?;
        let path = file.path();
        let file_to_compare = format!("{}/{}", output, path.file_name().unwrap().to_str().unwrap());
        if path.is_dir() {
            compare_dirs(&path.to_str().unwrap(), &file_to_compare)?;
        } else {
            compare_files(&path.to_str().unwrap(), &file_to_compare)?;
        }
    }
    Ok(())
}

#[test]
fn test_basic_site_generation() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let output_dir = temp_dir.path().join("_site");

    let mut cmd = Command::cargo_bin("crustdown")?;

    let result = cmd
        .arg("--root-dir")
        .arg(INPUT_CONTENT_LOCATION)
        .arg("--posts-dir")
        .arg(INPUT_POST_LOCATION)
        .arg("--output-dir")
        .arg(&output_dir)
        .assert()
        .success();

    result.success();

    compare_dirs(OUTPUT_POST_LOCATION, output_dir.to_str().unwrap())?;

    Ok(())
}
