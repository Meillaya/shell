use crate::commands::Command;
use crate::environment::ShellEnv;
use std::env;
use std::path::Path;

pub fn handle_builtin(command: &Command, shell_env: &mut ShellEnv) -> bool {
    match command.name.as_str() {
        "cd" => {
            cd(&command.args, shell_env);
            true
        }
        "pwd" => {
            pwd(shell_env);
            true
        }
        "echo" => {
            echo(&command.args);
            true
        }
        "exit" => {
            std::process::exit(0);
        }
        "type" => {
            type_command(&command.args);
            true
        }
        _ => false, // Not a built-in command
    }
}
fn find_in_path(cmd: &str) -> Option<String> {
    if let Ok(path) = env::var("PATH") { 
        for dir in path.split(':') {
            let full_path = Path::new(dir).join(cmd);
            if full_path.is_file() {
                return Some(full_path.to_string_lossy().into_owned());
            }
        }
    }
    None
}

fn type_command(args: &[String]) {
    if args.is_empty() {
        println!("type: missing argument");
        return;
    }

    let cmd = &args[0];
    match cmd.as_str() {
        "cd" | "pwd" | "echo" | "exit" | "type" => {
            println!("{} is a shell builtin", cmd);
        }
        _ => {
            if let Some(path) = find_in_path(cmd) {
                println!("{} is {}", cmd, path);
            } else {
                println!("{}: not found", cmd);
            }
            
        }
    }
}

fn cd(args: &[String], shell_env: &mut ShellEnv) {
    let new_dir = if args.is_empty() {
        env::var("HOME").unwrap_or_else(|_| String::from("/"))
    } else {
        let arg = &args[0];
        if arg == "~" || arg.starts_with("~/") {
            let home = env::var("HOME").unwrap_or_else(|_| String::from("/"));
            if arg == "~" {
                home
            } else {
                format!("{}{}", home, &arg[1..])
            }
        } else {
            arg.clone()
        }
    };

    
    if let Err(_e) = env::set_current_dir(Path::new(&new_dir)) {
        eprintln!("cd: {}: No such file or directory", new_dir);
    } else {
        shell_env.update_cwd(); 
    }
}
fn pwd(shell_env: &ShellEnv) {
    println!("{}", shell_env.cwd.display());
}

fn echo(args: &[String]) {
    println!("{}", args.join(" "));
}

