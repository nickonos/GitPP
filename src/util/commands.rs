use std::env;
use std::process::Command;

pub fn execute(cmd: &mut Command) {
    let debug_mode = env::var("DEBUG").unwrap_or("false".to_string()) == "true";

    println!("DEBUG MODE: {:?}", debug_mode);

    // In debug mode, print the command instead of executing it
    if debug_mode {
        println!("executing command {:?}", cmd);
    } else {
        let _ = cmd.spawn();
    }
}

pub fn parse_command(command: &str) -> Command {
    let split: Vec<&str> = command.split_whitespace().collect();
    let mut cmd = Command::new(split[0]);
    let args = &split[1..];

    for arg in args {
        cmd.arg(arg);
    }

    cmd
}
