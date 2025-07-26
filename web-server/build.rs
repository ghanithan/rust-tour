use std::env;
use std::path::Path;
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=../web/src");
    println!("cargo:rerun-if-changed=../web/package.json");
    
    // Only build frontend when embedding is enabled (for publishing)
    if env::var("CARGO_FEATURE_EMBED_ASSETS").is_ok() {
        let web_dir = Path::new("../web");
        let dist_dir = web_dir.join("dist");
        
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
    }
}