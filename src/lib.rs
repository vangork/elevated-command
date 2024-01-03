/*---------------------------------------------------------------------------------------------
 *  Copyright (c) Luis Liu. All rights reserved.
 *  Licensed under the MIT License. See License in the project root for license information.
 *--------------------------------------------------------------------------------------------*/
//! [![crates.io](https://img.shields.io/crates/v/elevated-command?logo=rust)](https://crates.io/crates/elevated-command/)
//! [![docs.rs](https://docs.rs/elevated-command/badge.svg)](https://docs.rs/elevated-command)
//! 
//! elevated-command - Run command using `sudo`, prompting the user with a graphical OS dialog if necessary
use std::convert::From;
use std::process::Command as StdCommand;

/// Wrap of std::process::command and escalate privileges while executing
pub struct Command {
    cmd: StdCommand,
    #[allow(dead_code)]
    icon: Option<Vec<u8>>,
    #[allow(dead_code)]
    name: Option<String>,
}

/// Command initialization shares the same logic across all the platforms
impl Command {
    /// Constructs a new `Command` from a std::process::Command 
    /// instance, it would read the following configuration from
    /// the instance while executing:
    ///
    /// * The instance's path to the program
    /// * The instance's arguments
    /// * The instance's environment variables
    /// 
    /// Current working directory would be the following while executing the command:
    ///   - %SystemRoot%\System32 on Windows
    ///   - /root on Linux
    ///   - $TMPDIR/sudo_prompt_applet/applet.app/Contents/MacOS on MacOS
    /// 
    /// # Examples
    ///
    /// ```no_run
    /// use elevated_command::Command;
    /// use std::process::Command as StdCommand;
    ///
    /// fn main() {
    ///     let mut cmd = StdCommand::new("path to the application");
    ///
    ///     cmd.arg("some arg");
    ///     cmd.env("some key", "some value");
    ///
    ///     let elevated_cmd = Command::new(cmd);
    /// }
    /// ```
    pub fn new(cmd: StdCommand) -> Self {
        Self {
            cmd,
            icon: None,
            name: None,
        }
    }

    /// Consumes the `Take`, returning the wrapped std::process::Command
    /// 
    /// # Examples
    ///
    /// ```no_run
    /// use elevated_command::Command;
    /// use std::process::Command as StdCommand;
    ///
    /// fn main() {
    ///     let mut cmd = StdCommand::new("path to the application");
    ///     let elevated_cmd = Command::new(cmd);
    ///     let cmd = elevated_cmd.into_inner();
    /// }
    /// ```
    pub fn into_inner(self) -> StdCommand {
        self.cmd
    }

    /// Gets a mutable reference to the underlying std::process::Command
    /// 
    /// # Examples
    ///
    /// ```no_run
    /// use elevated_command::Command;
    /// use std::process::Command as StdCommand;
    ///
    /// fn main() {
    ///     let mut cmd = StdCommand::new("path to the application");
    ///     let elevated_cmd = Command::new(cmd);
    ///     let cmd = elevated_cmd.get_ref();
    /// }
    /// ```
    pub fn get_ref(&self) -> &StdCommand {
        &self.cmd
    }

    /// Gets a reference to the underlying std::process::Command
    /// 
    /// # Examples
    ///
    /// ```no_run
    /// use elevated_command::Command;
    /// use std::process::Command as StdCommand;
    ///
    /// fn main() {
    ///     let mut cmd = StdCommand::new("path to the application");
    ///     let elevated_cmd = Command::new(cmd);
    ///     let cmd = elevated_cmd.get_mut();
    /// }
    /// ```
    pub fn get_mut(&mut self) -> &mut StdCommand {
        &mut self.cmd
    }

    /// Set the `icon` for the pop-up graphical OS dialog
    /// 
    /// This method is only applicable on `MacOS`
    /// 
    /// # Examples
    ///
    /// ```no_run
    /// use elevated_command::Command;
    /// use std::process::Command as StdCommand;
    ///
    /// fn main() {
    ///     let mut cmd = StdCommand::new("path to the application");
    ///     let elevated_cmd = Command::new(cmd);
    ///     elevated_cmd.icon(include_bytes!("path to the icon").to_vec());
    /// }
    /// ```
    pub fn icon(&mut self, icon: Vec<u8>) -> &mut Self {
        self.icon = Some(icon);
        self
    }

    /// Set the name for the pop-up graphical OS dialog
    /// 
    /// This method is only applicable on `MacOS`
    /// 
    /// # Examples
    ///
    /// ```no_run
    /// use elevated_command::Command;
    /// use std::process::Command as StdCommand;
    ///
    /// fn main() {
    ///     let mut cmd = StdCommand::new("path to the application");
    ///     let elevated_cmd = Command::new(cmd);
    ///     elevated_cmd.name("some name".to_string());
    /// }
    /// ```
    pub fn name(&mut self, name: String) -> &mut Self {
        self.name = Some(name);
        self
    }
}

impl From<StdCommand> for Command {
    /// Converts from a std::process::Command
    /// 
    /// It is similiar with the construct method
    fn from(cmd: StdCommand) -> Self {
        Self {
            cmd,
            icon: None,
            name: None,
        }
    }
}

#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "macos")]
mod macos;
