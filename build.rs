fn main() {
    // Tell Cargo that if the given file changes, to rerun this build script.
    println!("cargo:rerun-if-changed=src//main.rs"); 
    // Use the `rustc` command to compile the Rust file to an executable.
    std::process::Command::new("rustc")
        .args(&["src//main.rs", "-o", "wifu.exe"]) 
        .status()
        .unwrap();
}
