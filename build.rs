use std::process::Command;

fn main() {
    // Build the main.rs file using `cargo build`
    Command::new("cargo")
        .arg("install")
        .arg("bat")
        .status()
        .expect("Failed to run `cargo install bat` command");
    Command::new("cargo")
        .current_dir("src")
        .arg("build")
        .arg("--release")
        .status()
        .expect("Failed to build main.rs using `cargo build`");

    // Run the `cargo install bat` command

}
