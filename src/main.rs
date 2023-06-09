use clipboard_win::{formats};
use std::env;
use std::fs::{OpenOptions};
use regex::Regex;
use std::fs::{self, File, write};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::process::Command;
use std::fs::read_to_string;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use bat::error::Error;
use std::io::Read;
fn create_paths_if_not_exists(paths: &[&str]) -> Result<(), Error> {
    for path in paths {
        if let Some(parent) = std::path::Path::new(path).parent() {
            fs::create_dir_all(parent)?;
        }
        if !std::path::Path::new(path).exists() {
            let mut file = File::create(path)?;
            file.write_all(b"")?; // Write an empty byte sequence to create the file
        }
    }
    Ok(())
}

fn check_profile_file(profile_file_path: &str, lines: &[&str]) -> Result<(), Error> {
    let mut file = File::open(profile_file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut file_modified = false;
    let mut updated_contents = String::new();

    for line in lines {
        if !contents.contains(line) {
            file_modified = true;
            updated_contents.push_str(line);
            updated_contents.push('\n');
        }
    }

    updated_contents.push_str(&contents);

    if file_modified {
        let mut file = OpenOptions::new().write(true).truncate(true).open(profile_file_path)?;
        file.write_all(updated_contents.as_bytes())?;
        println!("Profile file modified: Added missing line(s)");
    }

    Ok(())
}



fn main() {
    let home_dir = env::var("USERPROFILE").expect("Failed to get home directory path");
    let mods_file_path = format!("{}\\Documents\\WindowsPowerShell\\mods.psm1", home_dir);
    let profile_file_path = format!("{}\\Documents\\WindowsPowerShell\\Microsoft.PowerShell_profile.ps1", home_dir);

    let paths = vec![mods_file_path.as_str(), profile_file_path.as_str()];

    create_paths_if_not_exists(&paths)
        .unwrap_or_else(|err| eprintln!("Failed to create paths: {}", err));

    let lines_to_check = [
        r#"oh-my-posh init pwsh --config "$HOME\Documents\WindowsPowerShell\ompthemes\custom.omp.yaml" | Invoke-Expression"#,
        r#"oh-my-posh init pwsh --config "$HOME\Documents\WindowsPowerShell\ompthemes\custom.omp.json" | Invoke-Expression"#,
    ];

    if let Err(err) = check_profile_file(&profile_file_path, &lines_to_check) {
        eprintln!("Profile file check failed: {}", err);
        // Handle the case when the required lines are not found in the file
    } else {
        // The file contains the required lines or they were added
        println!("");
    }
    let args: Vec<String> = env::args().collect();
    let mut stream = StandardStream::stdout(ColorChoice::Always);
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
                for i in 2..args.len() {
                    match args[i].as_str() {
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
                    (Some(name), Some( args)) => {
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
            
                        let function = format!("function {} {{\n    {}{}\n}}\n", name, args,{""});
            
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
            "cp" => {
                let user_profile = env::var("USERPROFILE").unwrap();
                let mods_file_path = format!("{}/Documents/WindowsPowerShell/mods.psm1", user_profile);
                let ps_profile_file_path = format!("{}/Documents/WindowsPowerShell/Microsoft.PowerShell_profile.ps1", user_profile);
                let file_content_mods = read_to_string(&mods_file_path).unwrap_or_default();
                let file_content_ps_profile = read_to_string(&ps_profile_file_path).unwrap_or_default();
                let newliner = "\n";
                let clipboard_content: String = clipboard_win::get_clipboard(formats::Unicode).expect("ERROR");
                let clipboard_trimmed_start = clipboard_content.trim_start();
                let clipboard_trimmed_end = clipboard_trimmed_start.trim_end();
                if clipboard_trimmed_end.starts_with("function ") && clipboard_trimmed_end.ends_with("\n}") {
                    if file_content_mods.contains(&clipboard_trimmed_end) {
                        writeln!(
                            stream,
                            "{}Function already exists in the mods file",
                            ansi_term::Color::Red.bold().paint("[ERROR] "),
                        )
                        .unwrap();
                        return;
                    }
                    let mut file = OpenOptions::new().append(true).open(&mods_file_path).unwrap();
                    file.write_all(&newliner.as_bytes()).unwrap();
                    file.write_all(&clipboard_trimmed_end.as_bytes()).unwrap();
                    writeln!(
                        stream,
                        "\u{001b}[32m[SUCCESS]\u{001b}[0m Function successfully added to mods file",
                    )
                    .unwrap();
                } else if clipboard_trimmed_end.starts_with("$") {
                    if file_content_ps_profile.contains(&clipboard_trimmed_end) {
                        writeln!(
                            stream,
                            "{}Variable already exists in the PS Profile file",
                            ansi_term::Color::Red.bold().paint("[ERROR] "),
                        )
                        .unwrap();
                        return;
                    }
                    let mut file = OpenOptions::new().append(true).open(&ps_profile_file_path).unwrap();
                    file.write_all(&newliner.as_bytes()).unwrap();
                    file.write_all(&clipboard_trimmed_end.as_bytes()).unwrap();
                    writeln!(
                        stream,
                        "\u{001b}[32m[SUCCESS]\u{001b}[0m Variable successfully added to PS Profile file",
                    )
                    .unwrap();
                } else {
                    writeln!(
                        stream,
                        "{}Clipboard does not contain a PowerShell variable or function",
                        ansi_term::Color::Red.bold().paint("[ERROR] "),
                    )
                    .unwrap();
                    return;
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
        ("cp", "              | import complete functions or variables from clipboard"),
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