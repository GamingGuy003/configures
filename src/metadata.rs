use serde::{Deserialize, Serialize};

type Error = crate::Error;

/// associates profile name and a unique hash identifier
#[derive(Serialize, Deserialize, Debug)]
pub struct MetaData {
    /// md5 hash made out of name + salt
    pub id: String,
    /// profile name
    pub name: String,
}

impl MetaData {
    /// attempts to create a new profile identifier
    pub fn new(name: String) -> Result<Self, Error> {
        let salt = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(Error::SystemTime)?
            .as_nanos();

        Ok(Self {
            // computes a unique identifier out of current time and profile name
            id: format!("{:x}", md5::compute(format!("{name}{salt}"))),
            name,
        })
    }
}
