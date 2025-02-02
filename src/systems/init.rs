use std::fs;
use std::io::{self, Write, Read};
use std::path::{Path, PathBuf};

pub const GRIT_DIR: &str = ".grit";
pub const CONFIG_FILE: &str = "config";

/// Initialize a new grit repository
pub fn init_grit() -> io::Result<()> {
    let current_dir = std::env::current_dir()?;
    let grit_path = current_dir.join(GRIT_DIR);

    if grit_path.exists() {
        println!("Already a grit repository.");
        update_grit_root(&current_dir)?;
        return Ok(());
    }

    // Create .grit directory
    fs::create_dir(&grit_path)?;

    // Write the absolute path to .grit/config
    update_grit_root(&current_dir)?;

    println!("Initialized empty grit repository in {}", grit_path.display());
    Ok(())
}

/// Find the root directory of the grit repository
pub fn find_grit_root(start_path: &Path) -> Option<PathBuf> {
    let mut current = start_path.canonicalize().ok()?;

    while current.parent().is_some() {
        let grit_path = current.join(GRIT_DIR);
        if grit_path.exists() {
            return Some(normalize_path(&current));
        }
        current = current.parent()?.to_path_buf();
    }

    None
}

/// Check if the current directory is inside a grit repository
pub fn is_grit_repo() -> bool {
    find_grit_root(&std::env::current_dir().unwrap()).is_some()
}

/// Update the `.grit/config` file with the current directory
pub fn update_grit_root(current_dir: &Path) -> io::Result<()> {
    let grit_path = current_dir.join(GRIT_DIR);
    let config_path = grit_path.join(CONFIG_FILE);

    // Check if .grit/config exists and create if not
    if !config_path.exists() {
        fs::File::create(&config_path)?;
        println!("Setup config file at: {}", config_path.display());
    }

    // Read existing path from config
    let mut file = fs::File::open(&config_path)?;
    let mut old_path = String::new();
    file.read_to_string(&mut old_path)?;

    let new_path = normalize_path(&current_dir.canonicalize()?).display().to_string();

    // If paths are different, update config
    if old_path.trim() != new_path {
        let mut file = fs::File::create(config_path)?;
        writeln!(file, "{}", new_path)?;
        println!("Updated grit repository path to {}", new_path);
    }

    Ok(())
}

/// Normalize a path by removing redundant components and resolving `.` and `. .`
pub fn normalize_path(path: &Path) -> PathBuf {
    let mut normalized = PathBuf::new();
    for component in path.components() {
        match component {
            std::path::Component::Prefix(prefix) => {
                // On Windows, strip the `\\?\` prefix if present
                let prefix_str = prefix.as_os_str().to_string_lossy();
                if prefix_str.starts_with(r"\\?\") {
                    normalized.push(&prefix_str[4..]);
                } else {
                    normalized.push(prefix.as_os_str());
                }
            }
            std::path::Component::RootDir => {
                normalized.push("/");
            }
            std::path::Component::CurDir => {} // Ignore `.`
            std::path::Component::ParentDir => {
                if !normalized.pop() {
                    // If we can't go up, just push `. .`
                    normalized.push("..");
                }
            }
            std::path::Component::Normal(name) => {
                normalized.push(name);
            }
        }
    }
    normalized
}