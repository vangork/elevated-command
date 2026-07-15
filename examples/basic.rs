use elevated_command::Command;
use std::process::Command as StdCommand;

fn main() {
    let mut cmd = StdCommand::new("sh");
    cmd.args(["-c", "whoami"]);
    let elevated_cmd = Command::new(cmd);
    let output = elevated_cmd.output().unwrap();
    println!("{}", str::from_utf8(&output.stdout).unwrap())
}
