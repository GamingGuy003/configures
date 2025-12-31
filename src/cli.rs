use std::path::PathBuf;

type Error = crate::Error;

#[derive(Clone, Debug)]
pub enum Arguments {
    /// lists all profiles
    ListProfiles,
    /// adds a profile via name
    AddProfile(String),
    /// removes a profile via identifier
    RemoveProfile(String),
    /// selects a specific profile via identitifer
    Profile(String),
    /// adds a path to a profile recursively
    AddPath(std::path::PathBuf),
    /// removes a path from a profile recursively
    RemovePath(std::path::PathBuf),
    /// applies a profile
    Apply,
    /// disables a profile
    Strip,
}

/// holds the parsed cli arguments
pub struct CLI {
    pub arguments: Vec<Arguments>,
}

impl IntoIterator for CLI {
    type Item = Arguments;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.arguments.into_iter()
    }
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
                    value.next().ok_or(Error::CLIMissingValue(arg))?,
                )),
                "add-profile" => arguments.push(Arguments::AddProfile(
                    value.next().ok_or(Error::CLIMissingValue(arg))?,
                )),
                "remove-profile" => arguments.push(Arguments::RemoveProfile(
                    value.next().ok_or(Error::CLIMissingValue(arg))?,
                )),
                "add" => arguments.push(Arguments::AddPath(PathBuf::from(
                    value.next().ok_or(Error::CLIMissingValue(arg))?,
                ))),
                "remove" => arguments.push(Arguments::RemovePath(PathBuf::from(
                    value.next().ok_or(Error::CLIMissingValue(arg))?,
                ))),
                "apply" => arguments.push(Arguments::Apply),
                "strip" => arguments.push(Arguments::Strip),
                // does not exist
                _ => continue,
            }
        }

        Ok(Self { arguments })
    }

    /// fetches an argument from the cli list
    /// ```
    /// let args = CLI::new(std::env::args());
    /// args.get(|arg| matches!(arg, Arguments::Profile(_))).is_some()
    /// ```
    pub fn get<F>(&self, mut search: F) -> Option<Arguments>
    where
        F: FnMut(&&Arguments) -> bool,
    {
        self.arguments
            .iter()
            // for some reason _element seems to be unused
            .find(|element| search(element))
            .cloned()
    }
}
