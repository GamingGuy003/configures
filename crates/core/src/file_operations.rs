use crate::error::{ConfiguresError, FileManipulationError};
use std::{env, io, path::Path, process::Command};

/// links a file from source to target or requests super privileges if the permissions are not
/// satisfied; source is the original config file and target is where it will show up on the systems
/// fs
pub fn link(source: &Path, target: &Path) -> Result<(), ConfiguresError> {
    let result = std::os::unix::fs::symlink(source, target);
    match result {
        Ok(()) => Ok(()),
        Err(err) if err.kind() == io::ErrorKind::PermissionDenied => {
            elevate_and_link(source, target)
        }
        Err(err) => Err(ConfiguresError::FileManipulationError(
            FileManipulationError::LinkingError(err.to_string()),
        )),
    }
}

/// attempts to elevate the process in order to link a path
fn elevate_and_link(source: &Path, target: &Path) -> Result<(), ConfiguresError> {
    // build the tool available on the current platform
    let tool = EscalationTool::detect().ok_or(ConfiguresError::FileManipulationError(
        FileManipulationError::NoEscalationTool,
    ))?;

    // try running the command built
    let status = tool
        .build_command("ln", &["-s".as_ref(), source, target])
        .status()
        .map_err(|err| {
            ConfiguresError::FileManipulationError(FileManipulationError::ExecutionError(
                err.to_string(),
            ))
        })?;

    // if command succeeded
    if status.success() {
        Ok(())
    } else {
        // if command didnt succeed we probably experienced a permission error
        Err(ConfiguresError::FileManipulationError(
            FileManipulationError::PermissionDenied,
        ))
    }
}

#[derive(Debug, Clone, Copy)]
enum EscalationTool {
    Pkexec,
    Doas,
    Sudo,
}

impl EscalationTool {
    /// tries to detect which elevation tool is available
    fn detect() -> Option<Self> {
        let path_var = env::var_os("PATH")?;
        for path in env::split_paths(&path_var) {
            if path.join("doas").is_file() {
                return Some(Self::Doas);
            }
            if path.join("sudo").is_file() {
                return Some(Self::Sudo);
            }
            if path.join("pkexec").is_file() {
                return Some(Self::Pkexec);
            }
        }
        None
    }

    /// builds the appropriate command out of the detected tool
    fn build_command(&self, program: &str, args: &[&Path]) -> Command {
        let mut cmd = match self {
            EscalationTool::Pkexec => Command::new("pkexec"),
            EscalationTool::Doas => Command::new("doas"),
            EscalationTool::Sudo => Command::new("sudo"),
        };
        cmd.arg(program);
        for arg in args {
            cmd.arg(arg);
        }
        cmd
    }
}
