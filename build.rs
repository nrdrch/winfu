fn main() {
    // Tell Cargo that if the given file changes, to rerun this build script.
    println!("cargo:rerun-if-changed=src//main.rs"); 
    // Use the `cargo` command to build the crate with optimizations enabled.
    std::process::Command::new("cargo")
        .args(&["build", "--release"]) 
        .status()
        .unwrap();
}
