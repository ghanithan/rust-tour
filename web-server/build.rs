use std::env;
use std::path::Path;
use std::process::Command;
use std::fs;
use std::time::SystemTime;

fn main() {
    println!("cargo:rerun-if-changed=../web/src");
    println!("cargo:rerun-if-changed=../web/package.json");
    
    // Only build frontend when embedding is enabled (for publishing)
    if env::var("CARGO_FEATURE_EMBED_ASSETS").is_ok() {
        let web_dir = Path::new("../web");
        let dist_dir = web_dir.join("dist");
        let local_dist = Path::new("web-dist");
        let _local_monaco = Path::new("monaco-editor");
        
        // Check if web directory exists (for local builds)
        if web_dir.exists() {
            // Check if we need to build the frontend
            let needs_build = !dist_dir.exists() 
                || !dist_dir.join("index.html").exists()
                || env::var("FORCE_FRONTEND_BUILD").is_ok()
                || source_files_newer_than_dist(&web_dir, &dist_dir);
            
            if needs_build {
                println!("cargo:warning=Building frontend assets for embedding...");
                
                // Change to web directory and run npm run build
                let output = Command::new("npm")
                    .args(&["run", "build"])
                    .current_dir(web_dir)
                    .output()
                    .expect("Failed to execute npm run build");
                
                if !output.status.success() {
                    panic!(
                        "Failed to build frontend:\nstdout: {}\nstderr: {}",
                        String::from_utf8_lossy(&output.stdout),
                        String::from_utf8_lossy(&output.stderr)
                    );
                }
                
                println!("cargo:warning=Frontend build completed successfully");
            }
            
            // Copy assets for packaging (always copy to ensure fresh assets)
            println!("cargo:warning=Copying frontend assets for packaging...");
            if local_dist.exists() {
                println!("cargo:warning=Removing existing web-dist directory...");
                if let Err(e) = fs::remove_dir_all(local_dist) {
                    println!("cargo:warning=Failed to remove existing web-dist: {}", e);
                }
            }
            
            if let Err(e) = copy_dir_all(&dist_dir, local_dist) {
                println!("cargo:warning=Failed to copy dist: {}", e);
            } else {
                println!("cargo:warning=Successfully copied assets to web-dist/");
            }
            
            // Monaco editor will be loaded from CDN - no need to copy
        } else {
            // For packaged builds, assets should already be copied
            println!("cargo:warning=Using pre-copied assets for packaging");
        }
    }
}

fn copy_dir_all(src: &Path, dst: &Path) -> std::io::Result<()> {
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(&entry.path(), &dst.join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.join(entry.file_name()))?;
        }
    }
    Ok(())
}

fn source_files_newer_than_dist(web_dir: &Path, dist_dir: &Path) -> bool {
    // If dist doesn't exist, we definitely need to build
    if !dist_dir.exists() {
        return true;
    }
    
    // Get the oldest modification time in dist directory
    let dist_oldest = match get_oldest_file_time(dist_dir) {
        Some(time) => time,
        None => return true, // No files in dist, need to build
    };
    
    // Check if any source file is newer than the oldest dist file
    let src_dir = web_dir.join("src");
    if src_dir.exists() {
        if let Some(src_newest) = get_newest_file_time(&src_dir) {
            if src_newest > dist_oldest {
                println!("cargo:warning=Source files newer than dist, rebuilding frontend");
                return true;
            }
        }
    }
    
    // Check package.json
    let package_json = web_dir.join("package.json");
    if package_json.exists() {
        if let Ok(metadata) = fs::metadata(&package_json) {
            if let Ok(modified) = metadata.modified() {
                if modified > dist_oldest {
                    println!("cargo:warning=package.json newer than dist, rebuilding frontend");
                    return true;
                }
            }
        }
    }
    
    false
}

fn get_oldest_file_time(dir: &Path) -> Option<SystemTime> {
    let mut oldest: Option<SystemTime> = None;
    
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            if let Ok(metadata) = entry.metadata() {
                if metadata.is_file() {
                    if let Ok(modified) = metadata.modified() {
                        oldest = Some(oldest.map_or(modified, |current| current.min(modified)));
                    }
                } else if metadata.is_dir() {
                    // Recursively check subdirectories
                    if let Some(subdir_oldest) = get_oldest_file_time(&entry.path()) {
                        oldest = Some(oldest.map_or(subdir_oldest, |current| current.min(subdir_oldest)));
                    }
                }
            }
        }
    }
    
    oldest
}

fn get_newest_file_time(dir: &Path) -> Option<SystemTime> {
    let mut newest: Option<SystemTime> = None;
    
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            if let Ok(metadata) = entry.metadata() {
                if metadata.is_file() {
                    if let Ok(modified) = metadata.modified() {
                        newest = Some(newest.map_or(modified, |current| current.max(modified)));
                    }
                } else if metadata.is_dir() {
                    // Recursively check subdirectories
                    if let Some(subdir_newest) = get_newest_file_time(&entry.path()) {
                        newest = Some(newest.map_or(subdir_newest, |current| current.max(subdir_newest)));
                    }
                }
            }
        }
    }
    
    newest
}