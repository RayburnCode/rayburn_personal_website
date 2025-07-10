use std::env;
use std::fs;

fn main() {
    // Load .env file if it exists
    if let Ok(contents) = fs::read_to_string(".env") {
        for line in contents.lines() {
            if let Some((key, value)) = line.split_once('=') {
                let key = key.trim();
                let value = value.trim();
                // Only set if not already set by the environment
                if env::var(key).is_err() {
                    println!("cargo:rustc-env={}={}", key, value);
                }
            }
        }
    }

    // Tell cargo to rerun this script if .env changes
    println!("cargo:rerun-if-changed=.env");
}
