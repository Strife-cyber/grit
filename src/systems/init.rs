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
        update_grit_root(&current_dir, "Main")?;
        return Ok(());
    }

    // Create .grit directory
    fs::create_dir(&grit_path)?;

    // Write the absolute path and branch to .grit/config
    update_grit_root(&current_dir, "Main")?;

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

pub fn update_branch(branch: &str) -> io::Result<()> {
    let current_dir = std::env::current_dir()?;
    let grit_path = current_dir.join(GRIT_DIR);

    if grit_path.exists() {
        update_grit_root(&current_dir, branch)?;
        println!("Branch set to {}", branch);
        return Ok(());
    }

    Ok(())
}

/// Update the `.grit/config` file with the current directory and branch
pub fn update_grit_root(current_dir: &Path, branch: &str) -> io::Result<()> {
    let grit_path = current_dir.join(GRIT_DIR);
    let config_path = grit_path.join(CONFIG_FILE);

    // Ensure the .grit directory exists
    if !grit_path.exists() {
        fs::create_dir(&grit_path)?;
    }

    let new_path = normalize_path(&current_dir.canonicalize()?).display().to_string();

    // Read existing config
    let mut old_content = String::new();
    if let Ok(mut file) = fs::File::open(&config_path) {
        file.read_to_string(&mut old_content)?;
    }

    let mut old_branch = "Main".to_string();
    let mut old_path = String::new();

    for line in old_content.lines() {
        if line.starts_with("path=") {
            old_path = line.replace("path=", "").trim().to_string();
        } else if line.starts_with("branch=") {
            old_branch = line.replace("branch=", "").trim().to_string();
        }
    }

    // Only update if there are changes
    if old_path != new_path || old_branch != branch {
        let mut file = fs::File::create(&config_path)?;
        writeln!(file, "path={}", new_path)?;
        writeln!(file, "branch={}", branch)?;
        println!("Updated grit repository path to {}", new_path);
        println!("Set branch to {}", branch);
    }

    Ok(())
}

/// Read the current branch from `.grit/config`
pub fn get_current_branch() -> io::Result<String> {
    let current_dir = std::env::current_dir()?;
    let grit_root = find_grit_root(&current_dir).ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Not a grit repository"))?;
    let config_path = grit_root.join(GRIT_DIR).join(CONFIG_FILE);

    let mut content = String::new();
    fs::File::open(&config_path)?.read_to_string(&mut content)?;

    for line in content.lines() {
        if line.starts_with("branch=") {
            return Ok(line.replace("branch=", "").trim().to_string());
        }
    }

    Err(io::Error::new(io::ErrorKind::InvalidData, "Branch not found in config"))
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
