use std::io::{self, Write};
use codecrafters_shell::environment::ShellEnv;
use codecrafters_shell::commands::parse_command;
use codecrafters_shell::built_ins::handle_builtin;
use codecrafters_shell::executor::execute_command;


fn main() {
    let mut shell_env = ShellEnv::new();

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

        // Parse the command
        if let Some(command) = parse_command(&input) {
            // Check for built-in commands
            if handle_builtin(&command, &mut shell_env) {
                continue;
            }

            // Execute external command
            if let Err(e) = execute_command(&command, &shell_env) {
                eprintln!("Error executing command: {}", e);
            }
        }
    }
}
