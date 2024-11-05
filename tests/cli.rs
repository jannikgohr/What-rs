use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;
use assert_fs::prelude::*;


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