use std::env;
use std::path::PathBuf;

fn main() {
    println!(
        "cargo:warning=INFO: 🔧 Rustytime build script started - configuring database paths..."
    );

    // Check for build-time database path configuration
    if let Ok(db_path) = env::var("RUSTYTIME_BUILD_DB_PATH") {
        println!("cargo:rustc-env=RUSTYTIME_BUILD_DB_PATH={}", db_path);
        println!(
            "cargo:warning=INFO: Using custom build-time DB path: {}",
            db_path
        );
    } else {
        // Set a default build-time database path using the same logic as runtime
        let base = if let Some(data_dir) = dirs::data_dir() {
            data_dir.join("rustytime")
        } else {
            // Fallback to a relative path for build time
            PathBuf::from(".rustytime")
        };
        let default_path = base.join("rustytime.db");
        println!(
            "cargo:rustc-env=RUSTYTIME_BUILD_DB_PATH={}",
            default_path.display()
        );
        println!(
            "cargo:warning=INFO: Using default build-time DB path: {}",
            default_path.display()
        );
    }

    // Set build metadata
    println!(
        "cargo:rustc-env=RUSTYTIME_BUILD_DATE={}",
        std::process::Command::new("date")
            .arg("+%Y-%m-%d %H:%M:%S")
            .output()
            .map(|output| String::from_utf8_lossy(&output.stdout).trim().to_string())
            .unwrap_or_else(|_| "unknown".to_string())
    );

    // Rerun if the environment variable changes
    println!("cargo:rerun-if-env-changed=RUSTYTIME_BUILD_DB_PATH");

    println!("cargo:warning=INFO: ✅ Rustytime build script completed successfully!");
}
