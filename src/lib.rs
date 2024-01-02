/*---------------------------------------------------------------------------------------------
 *  Copyright (c) Luis Liu. All rights reserved.
 *  Licensed under the MIT License. See License in the project root for license information.
 *--------------------------------------------------------------------------------------------*/

use std::process::Command as StdCommand;
use std::ffi::OsStr;

pub struct Command {
    cmd: StdCommand,
    #[allow(dead_code)]
    icon: Option<Vec<u8>>,
    #[allow(dead_code)]
    name: Option<String>,
}

impl Command {
    pub fn new<S: AsRef<OsStr>>(program: S) -> Self {
        Self {
            cmd: StdCommand::new(program),
            icon: None,
            name: None,
        }
    }

    /// Adds an argument to pass to the program.
    /// Same as std::process::Command::arg method.
    pub fn arg<S: AsRef<OsStr>>(&mut self, arg: S) -> &mut Command {
        self.cmd.arg(arg);
        self
    }

    /// Adds multiple arguments to pass to the program.
    /// Same as std::process::Command::args method.
    pub fn args<I, S>(&mut self, args: I) -> &mut Command
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        self.cmd.args(args);
        self
    }
}

#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "macos")]
mod macos;
