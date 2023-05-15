use std::env;
use std::fs::{OpenOptions};
use std::io::{Write};
use regex::Regex;
use std::fs::{self, File, write};
use std::io::{BufRead, BufReader, BufWriter};
use std::process::Command;
extern crate termcolor;
extern crate bat;
extern crate chrono;
use clipboard::ClipboardProvider;
use clipboard::ClipboardContext;
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
fn get_clipboard_contents() -> String {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    ctx.get_contents().unwrap_or_default()
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let mut stream = StandardStream::stdout(ColorChoice::Always);
    create_file_if_not_exists().expect("Failed to create file");
    let output = Command::new("bat")
        .arg("--version")
        .output()
        .ok();
    if output.is_none() {
        println!("'bat' is not installed. Installing...");
        let status = Command::new("cargo")
            .args(&["install", "bat"])
            .status();

        match status {
            Ok(status) => {
                if !status.success() {
                    eprintln!("Failed to install 'bat'");
                }
            }
            Err(e) => eprintln!("Failed to run cargo: {}", e),
        }
    } else {
    }

    if args.len() == 1 {
        print_usage(&mut stream);
    } else {
        match args[1].as_str() {
            "sv" => {
                let mut fn_name = None;
                let mut fn_args = None;
                let mut has_param = false;
                let mut has_clipboard = false;
                
                // Parse command-line arguments
                for i in 2..args.len() {
                    match args[i].as_str() {
                        "-p" | "-param" => {
                            if i + 1 < args.len() {
                                has_param = true;
                                fn_args = Some(args[i+1].to_owned());
                            } else {
                                print_usage(&mut stream);
                                return;
                            }
                        }
                        "-c" | "-clip" => {
                            has_clipboard = true;
                            fn_args = Some(get_clipboard_contents());
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
                match (fn_name, fn_args.clone()) {
                    (Some(name), Some(mut args)) => {
                        if has_clipboard {
                            has_param = true;
                        }
                        if let Some(param_name) = args.chars().find(|&arg| arg.to_string().starts_with("-p=")).map(|arg| arg.to_string()[3..].to_owned()) {
                            let _param_decl = format!("[string]${}", param_name);
                            args = args.chars().filter(|&arg| !arg.to_string().starts_with("-p=")).map(|arg| arg.to_owned()).collect::<String>();
                            args.insert(0, '$')
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
            "svp" => {
                let mut var_name = None;
                let mut var_value = None;
            
                // Parse command-line arguments
                for i in 2..args.len() {
                    match args[i].as_str() {
                        arg => {
                            if var_name.is_none() {
                                var_name = Some(arg.to_owned());
                            } else {
                                var_value = Some(arg.to_owned());
                                break;
                            }
                        }
                    }
                }
                match (var_name, var_value) {
                    (Some(name), Some(value)) => {
                        let user_profile = env::var("USERPROFILE").unwrap();
                        let file_path = format!("{}/Documents/WindowsPowerShell/Microsoft.PowerShell_profile.ps1", user_profile);
                        let file_content = read_to_string(&file_path).unwrap_or_default();
            
                        if file_content.contains(&format!("${} =", name)) {
                            writeln!(
                                stream,
                                "{}Variable '{}' already exists",
                                ansi_term::Color::Red.bold().paint("[ERROR] "),
                                name
                            )
                            .unwrap();
                            return;
                        }
            
                        let mut file = OpenOptions::new().append(true).open(&file_path).unwrap();
                        // let variable = format!("${} =  {}\n", name, value);
                        let variable = format!("${} =  '{}'\n", name, value.replace("'", "''").replace('"', r#"""""#));
                        file.write_all(variable.as_bytes()).unwrap();
            
                        writeln!(
                            stream,
                            "\u{001b}[32m[SUCCESS]\u{001b}[0m Variable '{}' successfully saved",
                            name
                        )
                        .unwrap();
                    }
                    _ => print_usage(&mut stream),
                }
            }
            "rmp" => {
                if args.len() < 3 {
                    print_usage(&mut stream);
                    return;
                }
            
                let var_name = args[2].as_str();
                let user_profile = env::var("USERPROFILE").unwrap();
                let file_path = format!("{}/Documents/WindowsPowerShell/Microsoft.PowerShell_profile.ps1", user_profile);
                let mut file_content = read_to_string(&file_path).unwrap_or_default();
            
                let variable_regex = Regex::new(&format!(r"(?m)^\${}\s*=\s*.+[\r\n]*", var_name)).unwrap();
                if !variable_regex.is_match(&file_content) {
                    writeln!(
                        stream,
                        "{}Variable '{}' not found",
                        ansi_term::Color::Red.bold().paint("[ERROR] "),
                        var_name
                    ).unwrap();
                    return;
                }
            
                file_content = variable_regex.replace_all(&file_content, "").to_string();
                write(&file_path, file_content.as_bytes()).unwrap();
            
                writeln!(
                    stream,
                    "\u{001b}[32m[SUCCESS]\u{001b}[0m Variable '{}' successfully deleted",
                    var_name
                ).unwrap();
            }
            
            "lsp" => {
                let user_profile = env::var("USERPROFILE").unwrap();
                let profile_path = format!("{}/Documents/WindowsPowerShell/Microsoft.PowerShell_profile.ps1", user_profile);
                let vars_file_path = env::temp_dir().join("psvars.ps1");
            
                // Remove the temporary file if it exists
                if vars_file_path.exists() {
                    match std::fs::remove_file(&vars_file_path) {
                        Ok(_) => {}
                        Err(err) => {
                            writeln!(
                                stream,
                                "{}Failed to remove temporary file: {}",
                                ansi_term::Color::Red.bold().paint("[ERROR] "),
                                err
                            ).unwrap();
                            return;
                        }
                    }
                }
            
                // Read the PowerShell profile file
                let profile_content = match read_to_string(&profile_path) {
                    Ok(content) => content,
                    Err(err) => {
                        writeln!(
                            stream,
                            "{}Failed to read PowerShell profile file: {}",
                            ansi_term::Color::Red.bold().paint("[ERROR] "),
                            err
                        ).unwrap();
                        return;
                    }
                };
            
                // Extract the variable declarations from the PowerShell profile file
                let var_regex = Regex::new(r"(?m)^\$([a-zA-Z0-9_]+)\s*=\s*'?(.*?)'?\s*$").unwrap();
                let mut vars = String::new();
                for capture in var_regex.captures_iter(&profile_content) {
                    let name = &capture[1];
                    let value = &capture[2];
                    vars.push_str(&format!("${} = '{}'\n", name, value.replace("'", "''").replace('"', r#"""""#)));
                }
            
                // Write the variable declarations to the temporary file
                match write(&vars_file_path, vars.as_bytes()) {
                    Ok(_) => {
                        // Use bat to get the content of the temporary file
                        let bat_output = Command::new("bat")
                            .arg("--style=numbers,changes")
                            .arg("--color=always")
                            .arg(&vars_file_path)
                            .output()
                            .unwrap();
            
                        // Write the bat output to the output stream
                        writeln!(
                            stream,
                            "{}",
                            String::from_utf8_lossy(&bat_output.stdout)
                        ).unwrap();
                    }
                    Err(err) => {
                        writeln!(
                            stream,
                            "{}Failed to write PowerShell variables file: {}",
                            ansi_term::Color::Red.bold().paint("[ERROR] "),
                            err
                        ).unwrap();
                    }
                };
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
    let header = ">_Windows Function Manager ";
    
    let cmd_descs = [
        ("sv", "<name> <args> | save a new function"),
        ("rm", "<name>        | remove an existing function"),
        ("ls", "              | list all functions"),
        ("svp", "<name> <args> | save a new PowerShell variable"),
        ("rmp", "<name>        | remove an existing PowerShell variable"),
        ("lsp", "              | list all PowerShell variables"),
    ];

    let footer = "";

    let mut cs = ColorSpec::new();
    cs.set_fg(Some(Color::Ansi256(42))).set_bold(true);
    stream.set_color(&cs).unwrap();
    writeln!(stream, "\n{}", header).unwrap();
    stream.reset().unwrap();
    println!("   ______________________________________________________________");
    stream.reset().unwrap();
    
    for (cmd, desc) in cmd_descs.iter() {
        let mut cs = ColorSpec::new();
        cs.set_fg(Some(Color::Ansi256(42))).set_bold(true);
        stream.set_color(&cs).unwrap();
        write!(stream, "   {:<4}", cmd).unwrap();
        stream.reset().unwrap();

        let mut cs = ColorSpec::new();
        cs.set_fg(Some(Color::Ansi256(255))).set_bold(true);
        stream.set_color(&cs).unwrap();
        writeln!(stream, "{}", desc).unwrap();
        stream.reset().unwrap();
    }

    let mut cs = ColorSpec::new();
    cs.set_fg(Some(Color::Ansi256(42))).set_bold(true);
    stream.set_color(&cs).unwrap();
    writeln!(stream, "{}", footer).unwrap();
}