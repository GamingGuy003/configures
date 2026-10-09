pub mod error;
pub mod file_operations;
pub mod git;
pub mod storage;

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use std::io::{self, Write};
    use std::path::Path;
    use tempfile::tempdir;

    #[test]
    /// create normal symlink
    fn test_create_link_success() -> std::io::Result<()> {
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
}
