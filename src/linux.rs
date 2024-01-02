use crate::Command;
use anyhow::Result;
use std::env;
use std::ffi::OsStr;
use std::path::PathBuf;
use std::process::{Command as StdCommand, Output};
use std::str::FromStr;

impl Command {
    pub fn is_elevated() -> bool {
        let uid = unsafe { 
            libc::getuid()
        };
        if uid == 0 { 
            true 
        } else { 
            false 
        }
    }

    pub fn output(&self) -> Result<Output> {
        let pkexec = PathBuf::from_str("/bin/pkexec")?;
        let mut command = StdCommand::new(pkexec);
        let display = env::var("DISPLAY");
        let xauthority = env::var("XAUTHORITY");
        let home = env::var("HOME");

        command.arg("--disable-internal-agent");
        if display.is_ok() || xauthority.is_ok() || home.is_ok() {
            command.arg("env");
            if let Ok(display) = display {
                command.arg(format!("DISPLAY={}", display));
            }
            if let Ok(xauthority) = xauthority {
                command.arg(format!("XAUTHORITY={}", xauthority));
            }
            if let Ok(home) = home {
                command.arg(format!("HOME={}", home));
            }
        }
        command.arg(self.cmd.get_program());
        let args: Vec<&OsStr> = self.cmd.get_args().collect();
        if !args.is_empty() {
            command.args(args);
        }

        let output = command.output()?;
        Ok(output)
    }
}
