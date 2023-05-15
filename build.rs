use std::process::Command;

fn main() {
    // Check if the `bat` binary is installed
    let bat_installed = Command::new("cargo")
        .arg("install")
        .arg("--list")
        .output()
        .expect("Failed to run `cargo install --list` command")
        .stdout
        .contains(&b"bat"[..]);

    if !bat_installed {
        // Run the `cargo install bat` command if `bat` is not installed
        Command::new("cargo")
            .arg("install")
            .arg("bat")
            .status()
            .expect("Failed to run `cargo install bat` command");
    }

    // Build the main.rs file using `cargo build`
    Command::new("cargo")
        .current_dir("src")
        .arg("build")
        .arg("--release")
        .status()
        .expect("Failed to build main.rs using `cargo build`");
}
