use std::env;
use std::fs::{OpenOptions};
use std::io::{Write};
use std::fs::{self, File};
use std::io::{BufRead, BufReader, BufWriter};

use std::process::Command;
extern crate termcolor;
extern crate bat;
extern crate chrono;
use std::fs::read_to_string;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

fn create_file_if_not_exists() -> std::io::Result<()> {
    let home_dir = env::var("USERPROFILE").expect("Failed to get home directory path");
    let mods_file_path = format!("{}\\Documents\\WindowsPowerShell\\mods.psm1", home_dir);
    let profile_file_path = format!("{}\\Documents\\WindowsPowerShell\\Microsoft.PowerShell_profile.ps1", home_dir);
    let profile_file_contents = fs::read_to_string(&profile_file_path)?;
    if !profile_file_contents.contains(&format!("Import-Module -DisableNameChecking \"{}\"", mods_file_path)) {
        let mut profile_file = OpenOptions::new().append(true).open(&profile_file_path)?;
        writeln!(profile_file, "Import-Module -DisableNameChecking \"{}\"", mods_file_path)?;
    }
    if !std::path::Path::new(&mods_file_path).exists() {
        let mut file = File::create(&mods_file_path)?;
        file.write_all(b"")?;
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut stream = StandardStream::stdout(ColorChoice::Always);
    create_file_if_not_exists().expect("Failed to create file");
    if args.len() == 1 {
        print_usage(&mut stream);
    } else {
        match args[1].as_str() {
            "sv" => {
                let fn_name = args.get(2);
                let fn_args = args.get(3);
            
                match (fn_name, fn_args) {
                    (Some(name), Some(args)) => {
                        let user_profile = env::var("USERPROFILE").unwrap();
                        let file_path = format!("{}/Documents/WindowsPowerShell/mods.psm1", user_profile);
                        let file_content = read_to_string(&file_path).unwrap_or_default();
                        
                        if file_content.contains(&format!("function {} {{", name)) {
                            writeln!(
                                stream,
                                "{}Function '{}' already exists",
                                ansi_term::Color::Red.bold().paint("[ERROR] "),
                                name
                                
                            )
                            .unwrap();
                            return;
                        }
                        
                        let mut file = OpenOptions::new()
                            .append(true)
                            .open(&file_path)
                            .unwrap();
            
                        let function = format!(
                            "function {} {{\n    Clear-Host; {}\n}}\n",
                            name, args
                        );
            
                        file.write_all(function.as_bytes()).unwrap();
            
                        writeln!(
                            stream,
                            "{}Function '{}' with args '{}' successfully saved",
                            ansi_term::Color::White.bold().paint("[SUCCESS] "),
                            name, args
                        )
                        .unwrap();
                    }
                    _ => print_usage(&mut stream),
                }
            }
            "rm" => {
                let fn_name = args.get(2);
                let mods_file_path = std::env::var("USERPROFILE")
                    .unwrap()
                    .replace("\\", "/")
                    .to_owned()
                    + "/Documents/WindowsPowerShell/mods.psm1";
            
                match fn_name {
                    Some(name) => {
                        let file = File::open(&mods_file_path).unwrap();
            
                        let reader = BufReader::new(file);
            
                        let mut writer = BufWriter::new(
                            OpenOptions::new()
                                .create(true)
                                .write(true)
                                .truncate(true)
                                .open(&mods_file_path.replace(".psm1", "_tmp.psm1"))
                                .unwrap(),
                        );
            
                        let mut found = false;
                        let mut in_function = false;
                        let mut depth = 0;
            
                        for line in reader.lines() {
                            let line = line.unwrap();
            
                            if line.starts_with(&format!("function {} {{", name)) && !in_function {
                                found = true;
                                in_function = true;
                                depth = 1;
            
                            } else if in_function && line.trim() == "}" {
                                depth -= 1;
            
                                if depth == 0 {
                                    in_function = false;
                                    continue;
                                }
                            } else if in_function {
                                depth += line.matches('{').count() as i32;
                                depth -= line.matches('}').count() as i32;
                            } else {
                                writeln!(writer, "{}", line).unwrap();
                            }
                        }
                        
                        if found {
                            std::fs::rename(
                                &mods_file_path.replace(".psm1", "_tmp.psm1"),
                                &mods_file_path,
                            )
                            .unwrap();
                            writeln!(
                                stream,
                                "\u{001b}[32m[SUCCESS]\u{001b}[0m Function '{}' successfully removed from mods.psm1",
                                name
                            )
                            .unwrap();
                        } else {
                            std::fs::remove_file(
                                &mods_file_path.replace(".psm1", "_tmp.psm1"),
                            )
                            .unwrap();
                            writeln!(
                                stream,
                                "\u{001b}[31m[ERROR]\u{001b}[0m Function '{}' not found in mods.psm1",
                                
                                name
                            )
                            .unwrap();
                        }
                    }
                    _ => print_usage(&mut stream),
                }
            }
                     
            
            "ls" => {
                let mods_file_path = std::env::var("USERPROFILE")
                    .unwrap()
                    .replace("\\", "/")
                    .to_owned()
                    + "/Documents/WindowsPowerShell/mods.psm1";
            
                let output = Command::new("bat")
                    .arg("--style=numbers,changes")
                    .arg("--color=always")
                    .arg(&mods_file_path)
                    .output()
                    .expect("Failed to execute bat command");
            
                let stdout = String::from_utf8(output.stdout).unwrap();
                println!("{}", stdout);
            }
            _ => print_usage(&mut stream),
        }
    }
}

fn print_usage(stream: &mut StandardStream) {
    let mut cs = ColorSpec::new();
    cs.set_fg(Some(Color::Yellow)).set_bold(true);
    stream.set_color(&cs).unwrap();
    writeln!(stream, "{}", "").unwrap();
    writeln!(stream, "{}", "    fish inspired ").unwrap();
    writeln!(stream, "{}", "      function creation for Windows").unwrap();
    stream.reset().unwrap();


    let mut cs = ColorSpec::new();
    cs.set_fg(Some(Color::White)).set_bold(true);
    stream.set_color(&cs).unwrap();
    writeln!(stream, "{}", "<------------------------------------------------>").unwrap();
    


    let mut cs = ColorSpec::new();
    cs.set_fg(Some(Color::Green)).set_bold(true);
    stream.set_color(&cs).unwrap();
    writeln!(stream, "{}", ">_ Usage example").unwrap();
    stream.reset().unwrap();
    
    
    let mut cs = ColorSpec::new();
    cs.set_fg(Some(Color::White)).set_bold(true);
    stream.set_color(&cs).unwrap();
    writeln!(stream, "      sv = Save a new function").unwrap();
    stream.reset().unwrap();


    let mut cs = ColorSpec::new();
    cs.set_fg(Some(Color::Blue)).set_bold(true);
    stream.set_color(&cs).unwrap();
    writeln!(stream, "              wifu sv <name> <args>").unwrap();
    stream.reset().unwrap();


    let mut cs = ColorSpec::new();
    cs.set_fg(Some(Color::White)).set_bold(true);
    stream.set_color(&cs).unwrap();
    writeln!(stream, "      rm = Remove an existing function").unwrap();
    stream.reset().unwrap();


    let mut cs = ColorSpec::new();
    cs.set_fg(Some(Color::Blue)).set_bold(true);
    stream.set_color(&cs).unwrap();
    writeln!(stream, "              wifu rm <name>").unwrap();
    stream.reset().unwrap();


    let mut cs = ColorSpec::new();
    cs.set_fg(Some(Color::White)).set_bold(true);
    stream.set_color(&cs).unwrap();
    writeln!(stream, "      ls = List all functions").unwrap();
    stream.reset().unwrap();


    let mut cs = ColorSpec::new();
    cs.set_fg(Some(Color::Blue)).set_bold(true);
    stream.set_color(&cs).unwrap();
    writeln!(stream, "              wifu ls").unwrap();
    stream.reset().unwrap();
}
