use std::path::PathBuf;

fn main() {
    let config = PathBuf::from("/home")
        .join(whoami::username())
        .join(".configures");
    println!(
        "Config path determined to be:  {}",
        config.to_str().unwrap_or_default()
    );
}
