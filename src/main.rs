use std::{path::PathBuf, sync::OnceLock};

use crate::{cli::CLI, metadata::MetaData};

mod cli;
mod error;
mod metadata;

type Error = crate::error::Error;

#[allow(dead_code)]
/// wether to backup or simply replace conflicting files
static REPLACE: bool = false;

static PROFILE: OnceLock<String> = OnceLock::new();
static WORKING_DIR: OnceLock<PathBuf> = OnceLock::new();
static PROFILES_FILE: &str = "profiles.json";

fn main() -> Result<(), Error> {
    // parse arguments
    let arguments = CLI::new(std::env::args())?;

    // fetch profile
    if let Some(cli::Arguments::Profile(profile_index)) =
        arguments.get(cli::Arguments::Profile(String::new()))
    {
        PROFILE.get_or_init(|| profile_index);
    }

    // work through arguments
    for argument in arguments {
        match argument {
            cli::Arguments::ListProfiles => list_profiles()?,
            cli::Arguments::AddProfile(name) => add_profile(name)?,
            cli::Arguments::RemoveProfile(identifier) => remove_profile(identifier)?,
            cli::Arguments::AddPath(path_buf) => add_path(path_buf)?,
            cli::Arguments::RemovePath(path_buf) => remove_path(path_buf)?,
            _ => {}
        }
    }

    Ok(())
}

/// loads the metadata file
fn load_metadata() -> Result<Vec<MetaData>, Error> {
    // determine metadata path
    let path = get_working_dir()?.join(PROFILES_FILE);

    // if config file does not exist create it with empty data inside
    if !path.exists() {
        std::fs::write(&path, "[]").map_err(|err| Error::ConfigWrite(path.clone(), err))?;
    }

    // parse file
    serde_json::from_str::<Vec<MetaData>>(
        &std::fs::read_to_string(&path).map_err(|err| Error::ConfigRead(path, err))?,
    )
    .map_err(Error::ConfigParse)
}

/// lists all currently available profiles
fn list_profiles() -> Result<(), Error> {
    let metadata = load_metadata()?;
    if metadata.is_empty() {
        println!("No profiles. Add on with add-profile <name>");
    } else {
        // output
        for item in metadata {
            println!("{}\t{}", item.id, item.name);
        }
    }
    Ok(())
}

/// adds new profile
fn add_profile(name: String) -> Result<(), Error> {
    let path = get_working_dir()?.join(PROFILES_FILE);
    let mut old_metadata = load_metadata()?;
    let metadata = MetaData::new(name)?;
    old_metadata.push(metadata);
    let new_metadata =
        serde_json::to_string_pretty(&old_metadata).map_err(Error::ConfigSerialize)?;
    std::fs::write(&path, new_metadata).map_err(|err| Error::ConfigWrite(path.to_path_buf(), err))
}

/// removes a profile via identifier
fn remove_profile(identifier: String) -> Result<(), Error> {
    let path = get_working_dir()?.join(PROFILES_FILE);
    let old_metadata = load_metadata()?;
    let new_metadata = old_metadata
        .iter()
        .filter(|element| element.id != identifier)
        .collect::<Vec<&MetaData>>();
    std::fs::write(
        &path,
        &serde_json::to_string_pretty(&new_metadata).map_err(Error::ConfigSerialize)?,
    )
    .map_err(|err| Error::ConfigWrite(path, err))
}

/// adds file to hopefully specified profile
fn add_path(path: PathBuf) -> Result<(), Error> {
    Ok(())
}

/// removes file from hopefully specified profile
fn remove_path(path: PathBuf) -> Result<(), Error> {
    Ok(())
}

/// determines the configures working directory and makes sure it exists
fn get_working_dir<'a>() -> Result<&'a PathBuf, Error> {
    let path = WORKING_DIR.get_or_init(|| {
        PathBuf::from("/home")
            .join(whoami::username())
            .join(".configures")
    });

    if !path.exists() {
        std::fs::create_dir_all(path)
            .map_err(|err| Error::WorkingDirCreation(path.to_owned(), err))?;
    }

    Ok(path)
}
