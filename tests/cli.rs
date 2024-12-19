use assert_cmd::prelude::*;
use assert_fs::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn find_content_in_directory() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = assert_fs::TempDir::new()?;
    let nested_dir = temp_dir.child("nested/dir");
    nested_dir.create_dir_all()?;

    let file = nested_dir.child("sample.txt");
    file.write_str("0x52908400098527886E0F7030069857D2E4169EE7")?;

    let mut cmd = Command::cargo_bin("what-rs")?;
    cmd.arg(temp_dir.path());
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Ethereum (ETH) Wallet Address"));

    Ok(())
}

#[test]
fn find_content_in_file() -> Result<(), Box<dyn std::error::Error>> {
    let file = assert_fs::NamedTempFile::new("sample.txt")?;
    file.write_str("0x52908400098527886E0F7030069857D2E4169EE7")?;

    let mut cmd = Command::cargo_bin("what-rs")?;
    cmd.arg("0x52908400098527886E0F7030069857D2E4169EE7");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Ethereum (ETH) Wallet Address"));

    Ok(())
}

#[test]
fn find_content_in_text() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("what-rs")?;
    cmd.arg("0x52908400098527886E0F7030069857D2E4169EE7");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Ethereum (ETH) Wallet Address"));

    Ok(())
}

#[test]
fn find_borderless_content_in_text() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("what-rs")?;
    cmd.arg("0x52908400098527886E0F7030069857D2E4169EE7");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Turkish Identification Number"));

    Ok(())
}

#[test]
fn dont_find_bordered_content_in_text() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("what-rs")?;
    cmd.arg("-d").arg("0x52908400098527886E0F7030069857D2E4169EE7");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Turkish Identification Number").not());

    Ok(())
}

#[test]
fn find_url() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("what-rs")?;
    cmd.arg("test.com");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Uniform Resource Locator (URL)"));

    Ok(())
}

#[test]
fn find_url_if_in_included() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("what-rs")?;
    cmd.arg("-i").arg("url").arg("test.com");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Uniform Resource Locator (URL)"));

    Ok(())
}

#[test]
fn find_url_if_unrelated_in_excluded() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("what-rs")?;
    cmd.arg("-e").arg("bitcoin").arg("test.com");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Uniform Resource Locator (URL)"));

    Ok(())
}

#[test]
fn dont_find_url_if_in_excluded() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("what-rs")?;
    cmd.arg("-e").arg("url").arg("test.com");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Uniform Resource Locator (URL)").not());

    Ok(())
}

#[test]
fn dont_find_url_if_unrelated_in_included() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("what-rs")?;
    cmd.arg("-i").arg("bitcoin").arg("test.com");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Uniform Resource Locator (URL)").not());

    Ok(())
}

#[test]
fn dont_find_url_if_too_rare() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("what-rs")?;
    cmd.arg("dQw4w9WgXcQ");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("YouTube Video ID").not());

    Ok(())
}

#[test]
fn find_url_if_not_too_rare() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("what-rs")?;
    cmd.arg("-r").arg("0:1").arg("dQw4w9WgXcQ");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("YouTube Video ID"));

    Ok(())
}