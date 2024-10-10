use crate::commands::Command;
use crate::environment::ShellEnv;
use std::process::Command as ProcessCommand;

pub fn execute_command(cmd: &Command, shell_env: &ShellEnv) -> std::io::Result<()> {
    let mut child = ProcessCommand::new(&cmd.name)
        .args(&cmd.args)
        .current_dir(&shell_env.cwd)
        .spawn()?;

    child.wait()?;
    Ok(())
}
