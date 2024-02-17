use assert_cmd::{prelude::*}; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use std::process::Command; // Run programs
#[test]
fn test_bin_3() -> Result<(), Box<dyn std::error::Error>>{
    let mut cmd = Command::cargo_bin("starfield")?;

    cmd.arg("中国人").arg("test.map");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("zh g r"));

    Ok(())
}

#[test]
fn test_bin_2() -> Result<(), Box<dyn std::error::Error>>{
    let mut cmd = Command::cargo_bin("starfield")?;

    cmd.arg("厉害").arg("test.map");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("li hai"));

    Ok(())
}

#[test]
fn test_bin_2_hulve() -> Result<(), Box<dyn std::error::Error>>{
    let mut cmd = Command::cargo_bin("starfield")?;

    cmd.arg("忽略").arg("test.map");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("hu lüe"));

    Ok(())
}
#[test]
fn test_bin_2_xuexi() -> Result<(), Box<dyn std::error::Error>>{
    let mut cmd = Command::cargo_bin("starfield")?;

    cmd.arg("学习").arg("test.map");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("xue xi"));

    Ok(())
}
#[test]
fn test_bin_2_lvguo() -> Result<(), Box<dyn std::error::Error>>{
    let mut cmd = Command::cargo_bin("starfield")?;

    cmd.arg("铝锅").arg("test.map");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("lü guo"));

    Ok(())
}
#[test]
fn test_bin_2_juji() -> Result<(), Box<dyn std::error::Error>>{
    let mut cmd = Command::cargo_bin("starfield")?;

    cmd.arg("聚集").arg("test.map");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("ju ji"));

    Ok(())
}
