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
        _ => false, // Not a built-in command
    }
}

fn cd(args: &[String], shell_env: &mut ShellEnv) {
    let new_dir = if args.is_empty() {
        env::var("HOME").unwrap_or_else(|_| String::from("/"))
    } else {
        args[0].clone()
    };

    if let Err(e) = env::set_current_dir(Path::new(&new_dir)) {
        eprintln!("cd: {}", e);
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
