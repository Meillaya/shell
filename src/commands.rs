use std::collections::VecDeque;

#[derive(Debug)]
pub struct Command {
    pub name: String,
    pub args: Vec<String>,
}

pub fn parse_command(input: &str) -> Option<Command> {
    // Simple parsing by splitting on whitespace
    let mut tokens: VecDeque<&str> = input.trim().split_whitespace().collect();

    if tokens.is_empty() {
        return None;
    }

    let name = tokens.pop_front()?.to_string();
    let args = tokens.into_iter().map(String::from).collect();

    Some(Command { name, args })
}
