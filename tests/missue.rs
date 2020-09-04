// Mosto of tests from https://github.com/pingcap/talent-plan/blob/master/courses/rust/projects/project-2/tests/tests.rs

use assert_cmd::prelude::*;
use missue::{KvStore, Result, MissueEngine};
use predicates::str::{contains, is_empty};
use std::process::Command;
use tempfile::TempDir;

// `missue` with no args should exit with a non-zero code.
#[test]
fn cli_no_args() {
    Command::cargo_bin("missue").unwrap().assert().failure();
}

// `missue -V` should print the version
#[test]
fn cli_version() {
    Command::cargo_bin("missue-client")
        .unwrap()
        .args(&["-V"])
        .assert()
        .stdout(contains(env!("CARGO_PKG_VERSION")));
}

// `missue add -n=<NAME> -d=<DESCRIPTION>` should print nothing and exit with zero.
#[test]
fn cli_add() {
    let temp_dir = TempDir::new().expect("unable to create temporary working directory");
    Command::cargo_bin("missue")
        .unwrap()
        .args(&["add", "-n", "key", "-d", "descr"])
        .current_dir(&temp_dir)
        .assert()
        .success()
        .stdout(is_empty());
}

#[test]
fn cli_invalid_get() {
    Command::cargo_bin("missue")
        .unwrap()
        .args(&["get"])
        .assert()
        .failure();

    Command::cargo_bin("missue")
        .unwrap()
        .args(&["get", "extra", "field"])
        .assert()
        .failure();
}

#[test]
fn cli_invalid_set() {
    Command::cargo_bin("missue")
        .unwrap()
        .args(&["set"])
        .assert()
        .failure();

    Command::cargo_bin("missue")
        .unwrap()
        .args(&["set", "missing_field"])
        .assert()
        .failure();

    Command::cargo_bin("missue")
        .unwrap()
        .args(&["set", "extra", "extra", "field"])
        .assert()
        .failure();
}

#[test]
fn cli_invalid_rm() {
    Command::cargo_bin("missue")
        .unwrap()
        .args(&["rm"])
        .assert()
        .failure();

    Command::cargo_bin("missue")
        .unwrap()
        .args(&["rm", "extra", "field"])
        .assert()
        .failure();
}

#[test]
fn cli_invalid_subcommand() {
    Command::cargo_bin("missue")
        .unwrap()
        .args(&["unknown", "subcommand"])
        .assert()
        .failure();
}

// Should get previously stored value.
#[test]
fn get_stored_value() -> Result<()> {
    let temp_dir = TempDir::new().expect("unable to create temporary working directory");
    let mut store = KvStore::open(temp_dir.path())?;

    store.set("key1".to_owned(), "value1".to_owned())?;
    store.set("key2".to_owned(), "value2".to_owned())?;

    assert_eq!(store.get("key1".to_owned())?, Some("value1".to_owned()));
    assert_eq!(store.get("key2".to_owned())?, Some("value2".to_owned()));

    // Open from disk again and check persistent data.
    drop(store);
    let mut store = KvStore::open(temp_dir.path())?;
    assert_eq!(store.get("key1".to_owned())?, Some("value1".to_owned()));
    assert_eq!(store.get("key2".to_owned())?, Some("value2".to_owned()));

    Ok(())
}

// Should overwrite existent value.
#[test]
fn overwrite_value() -> Result<()> {
    let temp_dir = TempDir::new().expect("unable to create temporary working directory");
    let mut store = KvStore::open(temp_dir.path())?;

    store.set("key1".to_owned(), "value1".to_owned())?;
    assert_eq!(store.get("key1".to_owned())?, Some("value1".to_owned()));
    store.set("key1".to_owned(), "value2".to_owned())?;
    assert_eq!(store.get("key1".to_owned())?, Some("value2".to_owned()));

    // Open from disk again and check persistent data.
    drop(store);
    let mut store = KvStore::open(temp_dir.path())?;
    assert_eq!(store.get("key1".to_owned())?, Some("value2".to_owned()));
    store.set("key1".to_owned(), "value3".to_owned())?;
    assert_eq!(store.get("key1".to_owned())?, Some("value3".to_owned()));

    Ok(())
}

// Should get `None` when getting a non-existent key.
#[test]
fn get_non_existent_value() -> Result<()> {
    let temp_dir = TempDir::new().expect("unable to create temporary working directory");
    let mut store = KvStore::open(temp_dir.path())?;

    store.set("key1".to_owned(), "value1".to_owned())?;
    assert_eq!(store.get("key2".to_owned())?, None);

    // Open from disk again and check persistent data.
    drop(store);
    let mut store = KvStore::open(temp_dir.path())?;
    assert_eq!(store.get("key2".to_owned())?, None);

    Ok(())
}

#[test]
fn remove_non_existent_key() -> Result<()> {
    let temp_dir = TempDir::new().expect("unable to create temporary working directory");
    let mut store = KvStore::open(temp_dir.path())?;
    assert!(store.remove("key1".to_owned()).is_err());
    Ok(())
}

#[test]
fn remove_key() -> Result<()> {
    let temp_dir = TempDir::new().expect("unable to create temporary working directory");
    let mut store = KvStore::open(temp_dir.path())?;
    store.set("key1".to_owned(), "value1".to_owned())?;
    assert!(store.remove("key1".to_owned()).is_ok());
    assert_eq!(store.get("key1".to_owned())?, None);
    Ok(())
}