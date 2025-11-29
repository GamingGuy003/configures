use serde::{Deserialize, Serialize};
use std::path::PathBuf;

type Error = crate::Error;

/// holds which files are tracked by profile
#[derive(Deserialize, Serialize)]
pub struct Profile {
    /// the files tracked by the profile
    files: Vec<ConfigFile>,
    /// should profile be copied or symlinked
    copy: bool,
}

impl Profile {
    /// creates a new profile with the specified config files
    pub fn new(files: &Vec<ConfigFile>, copy: bool) -> Self {
        Self {
            files: files.to_owned(),
            copy,
        }
    }

    /// applies a profile to the system
    pub fn apply(&self) -> Result<(), Error> {
        if self.copy {
            for file in &self.files {
                file.copy()?;
            }
        } else {
            for file in &self.files {
                file.link()?;
            }
        }
        Ok(())
    }

    /// removes everything related to a profile
    pub fn strip(&self) -> Result<(), Error> {
        for file in &self.files {
            file.unlink()?;
        }
        Ok(())
    }
}

/// holds a file entry
#[derive(Deserialize, Serialize, Clone)]
pub struct ConfigFile {
    /// the path in the profile
    profile_path: PathBuf,
    /// the path to create the link at
    system_path: PathBuf,
}

impl ConfigFile {
    /// creates a new configfile
    pub fn new(profile_path: PathBuf, system_path: PathBuf) -> Self {
        Self {
            profile_path,
            system_path,
        }
    }

    /// links a file from the profile to its system destination
    pub fn link(&self) -> Result<(), Error> {
        println!(
            "{} -> {}",
            self.profile_path.display(),
            self.system_path.display(),
        );
        if std::fs::exists(&self.system_path)
            .map_err(|err| Error::Link(self.profile_path.clone(), err))?
        {
            todo!("File conflicts not yet implemented");
        }

        std::os::unix::fs::symlink(&self.profile_path, &self.system_path)
            .map_err(|err| Error::Link(self.profile_path.clone(), err))
    }

    /// this name is inaccurate. moves a file to its system location. used for restoring backup
    /// profiles
    pub fn copy(&self) -> Result<(), Error> {
        if !std::fs::exists(&self.system_path)
            .map_err(|err| Error::Copy(self.system_path.clone(), err))?
        {
            println!(
                "{} -> {}",
                self.profile_path.display(),
                self.system_path.display(),
            );
            std::fs::rename(&self.profile_path, &self.system_path)
                .map_err(|err| Error::Copy(self.system_path.clone(), err))?
        } else {
            println!(
                "Skipping {} -X-> {}",
                &self.profile_path.display(),
                &self.system_path.display()
            )
        }
        Ok(())
    }

    /// unlinks a file from the system
    pub fn unlink(&self) -> Result<(), Error> {
        // check if file to remove is symlink
        if std::fs::symlink_metadata(&self.system_path)
            .map_err(|err| Error::Unlink(self.system_path.clone(), err))?
            .is_symlink()
            // check if symlink points to our source file
            && std::fs::read_link(&self.system_path)
                .map_err(|err| Error::Unlink(self.profile_path.clone(), err))?
                .eq(&self.profile_path)
        {
            println!(
                "{} -X-> {}",
                &self.profile_path.display(),
                &self.system_path.display()
            );
            std::fs::remove_file(&self.system_path)
                .map_err(|err| Error::Unlink(self.profile_path.clone(), err))?
        } else {
            println!(
                "Skipping {} -> {}",
                &self.profile_path.display(),
                &self.system_path.display()
            )
        }
        Ok(())
    }
}
