use std::fs::File;

use toml_edit::{value, Document};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pwd = std::env::current_dir()?;
    if !pwd.join("Cargo.toml").exists() {
        eprintln!("Not a cargo project, no Cargo.toml. Exiting.");
        return Ok(());
    }
    let home = dirs::home_dir().ok_or("couldn't get home dir")?;
    let suffix = pwd.strip_prefix(home)?;
    let tmp_dir = std::env::temp_dir();
    let constructed_path = tmp_dir.join("cargo-target").join(suffix);
    eprintln!("Determined path to be '{}'.", constructed_path.display());
    let cfg_file = pwd.join(".cargo/config.toml");
    let parent = cfg_file.parent().ok_or("cfg_file.parent() failed")?;
    if !parent.exists() {
        std::fs::create_dir(parent)?;
    }
    if !cfg_file.exists() {
        eprintln!("{} didn't exist, created.", cfg_file.display());
        File::create(&cfg_file)?;
    }
    let toml = std::fs::read_to_string(&cfg_file)?;
    let mut doc = toml.parse::<Document>()?;
    doc["build"]["target-dir"] = value(constructed_path.display().to_string());
    std::fs::write(&cfg_file, doc.to_string())?;
    eprintln!("Successfully written config file");
    Ok(())
}
