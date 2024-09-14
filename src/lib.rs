use std::process;

pub fn stdout_to_string(output: process::Output) -> String {
    String::from_utf8(output.stdout).expect("output of `pw-cli` should be valid utf-8")
}

pub fn execute_command(command: &str, args: &[&str]) -> process::Output {
    process::Command::new(command)
        .args(args)
        .stdin(process::Stdio::inherit())
        .output()
        .unwrap_or_else(|_| panic!("Failed to execute command `{command}`"))
}
