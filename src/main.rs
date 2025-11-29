use std::path::PathBuf;

use crate::{
    metadata::MetaData,
    profile::{ConfigFile, Profile},
};

mod error;
mod metadata;
mod profile;

type Error = crate::error::Error;

#[allow(dead_code)]
/// wether to backup or simply replace conflicting files
static REPLACE: bool = false;

fn main() -> Result<(), Error> {
    // profiles config folder
    let test_source = PathBuf::from("/home")
        .join(whoami::username())
        .join("Documents/Pfush/test_src");

    // read metadata from file
    let metadata = serde_json::from_str::<Vec<MetaData>>(
        std::fs::read_to_string(&test_source.join("profiles.json"))
            .map_err(|err| Error::ConfigRead(test_source.clone(), err))?
            .as_str(),
    )
    .map_err(Error::ConfigParse)?;

    // fetch a random profile
    let profile = metadata.first().unwrap();
    let profile_config = test_source.join(&profile.id).join("profile_config.json");

    let test_target = PathBuf::from("/home")
        .join(whoami::username())
        .join(".configures");

    let files = vec![
        ConfigFile::new(test_source.join("1"), test_target.join("1")),
        ConfigFile::new(test_source.join("2"), test_target.join("2")),
        ConfigFile::new(test_source.join("3"), test_target.join("3")),
        ConfigFile::new(test_source.join("4"), test_target.join("4")),
    ];
    let profile = Profile::new(&files, true);

    // write profile config
    std::fs::write(
        &profile_config,
        serde_json::to_string_pretty(&profile).map_err(Error::ConfigSerialize)?,
    )
    .map_err(|err| Error::ConfigWrite(profile_config, err))
}
