use std::env;
use std::process::Command;

fn main() {
    // Fetch dependencies
    let status = Command::new("cargo")
        .arg("fetch")
        .status()
        .expect("failed to execute cargo fetch");

    if !status.success() {
        panic!("failed to fetch dependencies");
    }

    // Compile the main.rs file
    let status = Command::new("cargo")
        .arg("build")
        .arg("--release")
        .status()
        .expect("failed to execute cargo build");

    if !status.success() {
        panic!("failed to compile application");
    }

    // Move the executable to the standard bin directory
    let bin_dir = env::var("USERPROFILE").unwrap() + "\\.cargo\\bin\\";
    let output = Command::new("cmd")
        .arg("/C")
        .arg("copy")
        .arg(format!("target\\release\\{}.exe", env!("wifutest")))
        .arg(&bin_dir)
        .output()
        .expect("failed to execute copy command");

    if !output.status.success() {
        panic!("failed to move executable to bin directory: {}", String::from_utf8_lossy(&output.stderr));
    }
}