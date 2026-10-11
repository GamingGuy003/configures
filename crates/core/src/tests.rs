use super::*;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;
use tempfile::tempdir;

#[test]
/// tries to initialize the core by cloning a test repo
fn test_init() -> Result<(), ConfiguresError> {
    let configures_core = ConfiguresCoreBuilder::new()
        .remote_url("https://github.com/GamingGuy003/configures.git")
        .build()?;
    configures_core.init()?;

    let local_path = configures_core.local_path.clone();

    // test if the repos cargo.toml has been cloned
    assert!(Path::new(&local_path).join("Cargo.toml").exists());
    Ok(())
}

#[test]
fn test_builder_ok() -> Result<(), ConfiguresError> {
    let configures_core = ConfiguresCoreBuilder::new()
        .remote_url("remote_url")
        .build()?;

    assert_eq!(configures_core.remote_url, "remote_url");
    Ok(())
}

#[test]
fn test_builder_error() -> Result<(), ConfiguresError> {
    let builder = ConfiguresCoreBuilder::new();
    assert!(builder.build().is_err());

    let builder = ConfiguresCoreBuilder::new().remote_url("remote_url");
    assert!(builder.build().is_ok());
    Ok(())
}

#[test]
/// create normal symlink
fn test_create_link_success() -> io::Result<()> {
    let dir = tempdir()?;
    let src_path = dir.path().join("source_config");
    let dst_path = dir.path().join("destination_config");

    let mut file = File::create(&src_path)?;
    write!(file, "testcontent")?;

    file_operations::link(&src_path, &dst_path)
        .map_err(|err| std::io::Error::other(format!("{:?}", err)))?;

    assert!(dst_path.exists());
    assert_eq!(fs::read_link(&dst_path)?, src_path);

    let content = fs::read_to_string(dst_path)?;
    assert_eq!(content.trim(), "testcontent");

    Ok(())
}

#[test]
/// create dangling symlink
fn test_create_dangling_symlink() -> io::Result<()> {
    let dir = tempdir()?;
    let src_path = dir.path().join("source_config");
    let dst_path = dir.path().join("destination_config");

    file_operations::link(&src_path, &dst_path)
        .map_err(|err| std::io::Error::other(format!("{:?}", err)))?;

    assert!(fs::symlink_metadata(&dst_path).is_ok());
    assert!(!dst_path.exists());
    Ok(())
}

#[test]
/// create link to elevated folder
fn test_build_elevated_command_args() -> io::Result<()> {
    let dir = tempdir()?;
    let src_path = dir.path().join("source_config");
    let dst_path = Path::new("/etc/elevated_config");

    File::create(&src_path)?.write_all(b"testcontent")?;

    file_operations::link(&src_path, dst_path)
        .map_err(|err| std::io::Error::other(format!("{:?}", err)))?;

    assert!(dst_path.exists());
    assert_eq!(fs::read_link(dst_path)?, src_path);

    let content = fs::read_to_string(dst_path)?;
    assert_eq!(content.trim(), "testcontent");

    Ok(())
}
