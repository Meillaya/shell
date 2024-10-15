use std::io::{self, Write};
use codecrafters_shell::environment::ShellEnv;
use codecrafters_shell::commands::parse_command;
use codecrafters_shell::built_ins::handle_builtin;
use codecrafters_shell::executor::execute_command;
use std::env;

fn main() {
    let mut shell_env = ShellEnv::new();

    if let Ok(path) = env::var("PATH") {
        shell_env.set_path(path)
    }
    
    loop {
        // Display prompt
        print!("$ ");
        io::stdout().flush().unwrap();

        // Read user input
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            eprintln!("Error reading input");
            continue;
        }
        
        if input.trim().is_empty() {
            continue;
        }

        
        // Parse the command
        if let Some(command) = parse_command(&input) {
            // Check for built-in commands
            if handle_builtin(&command, &mut shell_env) {
                continue;
            }

            match execute_command(&command, &shell_env) {
                Ok(_) => {},
                Err(e) => {
                    if e.kind() == io::ErrorKind::NotFound {
                        println!("{}: command not found", command.name)
                    } else {
                        eprintln!("Error Executing command: {}", e)
                    }
                }
            }

        }
    }
}
