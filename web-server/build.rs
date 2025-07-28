use std::env;
use std::path::Path;
use std::process::Command;
use std::fs;

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
                || env::var("FORCE_FRONTEND_BUILD").is_ok();
            
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