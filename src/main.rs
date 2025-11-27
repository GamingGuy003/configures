use std::path::PathBuf;

use crate::profile::{ConfigFile, Profile};

mod error;
mod metadata;
mod profile;

#[allow(dead_code)]
/// wether to backup or simply replace conflicting files
static REPLACE: bool = false;

fn main() {
    let test_source = PathBuf::from("/home")
        .join(whoami::username())
        .join("Documents/Pfush/test_src");
    let test_target = PathBuf::from("/home")
        .join(whoami::username())
        .join(".configures");

    let files = vec![
        ConfigFile::new(test_source.join("1"), test_target.join("1")),
        ConfigFile::new(test_source.join("2"), test_target.join("2")),
        ConfigFile::new(test_source.join("3"), test_target.join("3")),
        ConfigFile::new(test_source.join("4"), test_target.join("4")),
    ];
    let profile = Profile::new("sondprofile", &files, true);
    profile.apply().unwrap()
}
