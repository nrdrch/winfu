use std::process::Command;

fn main() {
    // Run cargo to get all dependencies
    let output = Command::new("cargo")
        .args(&["build", "--release"])
        .output()
        .expect("Failed to execute command");

    if output.status.success() {
        // If the build was successful, print a success message
        println!("Dependencies built successfully!");
    } else {
        // If the build failed, print an error message and exit with a non-zero code
        eprintln!("Error building dependencies: {:?}", output);
        std::process::exit(1);
    }
}