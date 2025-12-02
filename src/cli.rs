use std::path::PathBuf;

type Error = crate::Error;

pub enum Arguments {
    /// lists all profiles
    ListProfiles,
    /// adds a profile
    AddProfile(String),
    /// selects a specific profile
    Profile(u16),
    /// adds a path to a profile recursively
    AddPath(std::path::PathBuf),
    /// removes a path from a profile recursively
    RemovePath(std::path::PathBuf),
}

/// holds the parsed cli arguments
pub struct CLI {
    pub arguments: Vec<Arguments>,
}

impl CLI {
    /// parses the env arguments into the arguments list
    /// ```
    /// let args = CLI::new(std::env::args());
    /// ```
    pub fn new(mut value: std::env::Args) -> Result<Self, Error> {
        let mut arguments = Vec::new();
        value.next();

        // iterate through all passed arguments and parse them into the enums
        while let Some(arg) = value.next() {
            // remove leading -
            match arg.as_str() {
                "list" => arguments.push(Arguments::ListProfiles),
                "profile" => arguments.push(Arguments::Profile(
                    value
                        .next()
                        .ok_or(Error::CLIMissingValue(arg))?
                        .parse()
                        .map_err(|err| Error::CLIValueParse(err))?,
                )),
                "add-profile" => arguments.push(Arguments::AddProfile(
                    value.next().ok_or(Error::CLIMissingValue(arg))?,
                )),
                "add" => arguments.push(Arguments::AddPath(PathBuf::from(
                    value.next().ok_or(Error::CLIMissingValue(arg))?,
                ))),
                "remove" => arguments.push(Arguments::RemovePath(PathBuf::from(
                    value.next().ok_or(Error::CLIMissingValue(arg))?,
                ))),
                // does not exist
                _ => continue,
            }
        }

        Ok(Self { arguments })
    }
}
