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
                let mut fn_name = None;
                let mut fn_args = None;
                let mut has_param = false;
            
                // Parse command-line arguments
                for i in 2..args.len() {
                    match args[i].as_str() {
                        "-p" | "-param" => {
                            has_param = true;
                        }
                        arg => {
                            if fn_name.is_none() {
                                fn_name = Some(arg.to_owned());
                            } else {
                                fn_args = Some(arg.to_owned());
                                break;
                            }
                        }
                    }
                }
            
                match (fn_name, fn_args) {
                    (Some(name), Some(mut args)) => {
                        if has_param {
                            if !args.starts_with("param") {
                                args = format!("param([string] $CustomInput)\n    {} $CustomInput", args);
                            } else {
                                args = args.replacen("param", "param([string] $CustomInput)", 1);
                                args = args.replacen("{", "{\n    $CustomInput ", 1);
                            }
                        }
            
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
            
                        let mut file = OpenOptions::new().append(true).open(&file_path).unwrap();
            
                        let function = format!("function {} {{\n    {}{}\n}}\n", name, args, if has_param {" "} else {""});
            
                        file.write_all(function.as_bytes()).unwrap();
            
                        writeln!(
                            stream,
                            "\u{001b}[32m[SUCCESS]\u{001b}[0m Function '{}' successfully saved",
                            name
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
    cs.set_fg(Some(Color::Ansi256(42))).set_bold(true);
    stream.set_color(&cs).unwrap();
    writeln!(stream, "{}", "").unwrap();
    
    writeln!(stream, "{}", "            Windows Function Manager ").unwrap();
    stream.reset().unwrap();

    let mut cs = ColorSpec::new();
    cs.set_fg(Some(Color::Ansi256(36))).set_bold(false);
    stream.set_color(&cs).unwrap();
    write!(stream, "{}", "<>").unwrap();
    
    let mut cs = ColorSpec::new();
    cs.set_fg(Some(Color::Ansi256(230))).set_bold(true);
    stream.set_color(&cs).unwrap();
    write!(stream, "{}", " - - - - - - - - - - - - - - - - - - - - - - ").unwrap();
    
    let mut cs = ColorSpec::new();
    cs.set_fg(Some(Color::Ansi256(36))).set_bold(false);
    stream.set_color(&cs).unwrap();
    writeln!(stream, "{}", "<>").unwrap();
          
    let mut cs = ColorSpec::new();
    cs.set_fg(Some(Color::White)).set_bold(true);
    stream.set_color(&cs).unwrap();
    write!(stream, "{}", " >_ ").unwrap();
    stream.reset().unwrap();
    let mut cs = ColorSpec::new();
    cs.set_fg(Some(Color::Ansi256(231))).set_bold(false);
    stream.set_color(&cs).unwrap();
    writeln!(stream, "{}", "Usage example").unwrap();
    stream.reset().unwrap();

    
    let mut cs = ColorSpec::new();
    cs.set_fg(Some(Color::Ansi256(42))).set_bold(true);
    stream.set_color(&cs).unwrap();
    write!(stream, "{}", "      sv ").unwrap();
    stream.reset().unwrap();


    let mut cs = ColorSpec::new();
    cs.set_fg(Some(Color::White)).set_bold(true);
    stream.set_color(&cs).unwrap();
    writeln!(stream, "= save a new function").unwrap();
    stream.reset().unwrap();


    let mut cs = ColorSpec::new();
    cs.set_fg(Some(Color::White)).set_bold(true);
    stream.set_color(&cs).unwrap();
    write!(stream, "              wifu").unwrap();
    stream.reset().unwrap();

    let mut cs = ColorSpec::new();
    cs.set_fg(Some(Color::Ansi256(42))).set_bold(true);
    stream.set_color(&cs).unwrap();
    write!(stream, "{}", " sv ").unwrap();
    stream.reset().unwrap();

    let mut cs = ColorSpec::new();
    cs.set_fg(Some(Color::Ansi256(38))).set_bold(true);
    stream.set_color(&cs).unwrap();
    write!(stream, "<option>").unwrap();
    stream.reset().unwrap();

    let mut cs = ColorSpec::new();
    cs.set_fg(Some(Color::White)).set_bold(true);
    stream.set_color(&cs).unwrap();
    writeln!(stream, " <name> <args>").unwrap();
    stream.reset().unwrap();

    //let mut cs = ColorSpec::new();
    //cs.set_fg(Some(Color::White)).set_bold(true);
    //stream.set_color(&cs).unwrap();
    //writeln!(stream, "                      add a string input parameter with \n").unwrap();
    let mut cs = ColorSpec::new();
    cs.set_fg(Some(Color::Ansi256(38))).set_bold(true);
    stream.set_color(&cs).unwrap();
    write!(stream, "                      -p").unwrap();
    stream.reset().unwrap();
    let mut cs = ColorSpec::new();
    cs.set_fg(Some(Color::Ansi256(231))).set_bold(true);
    stream.set_color(&cs).unwrap();
    write!(stream, " or ").unwrap();
    stream.reset().unwrap();
    let mut cs = ColorSpec::new();
    cs.set_fg(Some(Color::Ansi256(38))).set_bold(true);
    stream.set_color(&cs).unwrap();
    write!(stream, "-param ").unwrap();
    stream.reset().unwrap();
    let mut cs = ColorSpec::new();
    cs.set_fg(Some(Color::Ansi256(231))).set_bold(true);
    stream.set_color(&cs).unwrap();
    writeln!(stream, "= add a string parameter").unwrap();

    stream.reset().unwrap();
    println!("");
    let mut cs = ColorSpec::new();
    cs.set_fg(Some(Color::Ansi256(42))).set_bold(true);
    stream.set_color(&cs).unwrap();
    write!(stream, "{}", "      rm ").unwrap();
    stream.reset().unwrap();

    let mut cs = ColorSpec::new();
    cs.set_fg(Some(Color::White)).set_bold(true);
    stream.set_color(&cs).unwrap();
    writeln!(stream, "= remove an existing function").unwrap();
    stream.reset().unwrap();

    let mut cs = ColorSpec::new();
    cs.set_fg(Some(Color::White)).set_bold(true);
    stream.set_color(&cs).unwrap();
    write!(stream, "              wifu").unwrap();
    stream.reset().unwrap();

    let mut cs = ColorSpec::new();
    cs.set_fg(Some(Color::Ansi256(42))).set_bold(true);
    stream.set_color(&cs).unwrap();
    write!(stream, "{}", " rm ").unwrap();
    stream.reset().unwrap();

    let mut cs = ColorSpec::new();
    cs.set_fg(Some(Color::White)).set_bold(true);
    stream.set_color(&cs).unwrap();
    writeln!(stream, "{}", "<name>").unwrap();
    stream.reset().unwrap();
    println!("");
    let mut cs = ColorSpec::new();
    cs.set_fg(Some(Color::Ansi256(42))).set_bold(true);
    stream.set_color(&cs).unwrap();
    write!(stream, "      ls").unwrap();
    stream.reset().unwrap();

    let mut cs = ColorSpec::new();
    cs.set_fg(Some(Color::White)).set_bold(true);
    stream.set_color(&cs).unwrap();
    writeln!(stream, "{}", " = list all functions").unwrap();
    stream.reset().unwrap();

    let mut cs = ColorSpec::new();
    cs.set_fg(Some(Color::White)).set_bold(true);
    stream.set_color(&cs).unwrap();
    write!(stream, "              wifu").unwrap();
    stream.reset().unwrap();

    let mut cs = ColorSpec::new();
    cs.set_fg(Some(Color::Ansi256(42))).set_bold(true);
    stream.set_color(&cs).unwrap();
    writeln!(stream, " ls").unwrap();
    stream.reset().unwrap();

    let mut cs = ColorSpec::new();
    cs.set_fg(Some(Color::Yellow)).set_bold(true);
    stream.set_color(&cs).unwrap();
    //writeln!(stream, "{}", "").unwrap();
    

}
