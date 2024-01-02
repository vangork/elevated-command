/*---------------------------------------------------------------------------------------------
 *  Copyright (c) Luis Liu. All rights reserved.
 *  Licensed under the MIT License. See License in the project root for license information.
 *--------------------------------------------------------------------------------------------*/

use crate::Command;
use anyhow::{bail, Result};
use std::mem;
use std::os::windows::process::ExitStatusExt;
use std::process::{Output, ExitStatus};
use winapi::shared::minwindef::{DWORD, LPVOID};
use winapi::um::processthreadsapi::{GetCurrentProcess, OpenProcessToken};
use winapi::um::securitybaseapi::GetTokenInformation;
use winapi::um::winnt::{HANDLE, TokenElevation, TOKEN_ELEVATION, TOKEN_QUERY};
use windows::core::{HSTRING, PCWSTR, w};
use windows::Win32::Foundation::HWND;
use windows::Win32::UI::Shell::ShellExecuteW;
use windows::Win32::UI::WindowsAndMessaging::SW_HIDE;


/// The implementation of state check and elevated executing varies on each platform
impl Command {
    /// Check the state the current program running
    /// 
    /// Return `true` if the program is running as root, otherwise false
    /// 
    /// # Examples
    ///
    /// ```no_run
    /// use elevated_command::Command;
    ///
    /// fn main() {
    ///     let is_elevated = Command::is_elevated();
    ///
    /// }
    /// ```
    pub fn is_elevated() -> bool {
        // Thanks to https://stackoverflow.com/a/8196291
        unsafe {
            let mut current_token_ptr: HANDLE = mem::zeroed();
            let mut token_elevation: TOKEN_ELEVATION = mem::zeroed();
            let token_elevation_type_ptr: *mut TOKEN_ELEVATION = &mut token_elevation;
            let mut size: DWORD = 0;
    
            let result = OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut current_token_ptr);
    
            if result != 0 {
                let result = GetTokenInformation(
                    current_token_ptr,
                    TokenElevation,
                    token_elevation_type_ptr as LPVOID,
                    mem::size_of::<winapi::um::winnt::TOKEN_ELEVATION_TYPE>() as u32,
                    &mut size,
                );
                if result != 0 {
                    return token_elevation.TokenIsElevated != 0;
                }
            }
        }
        false
    }

    /// Prompting the user with a graphical OS dialog for the root password, 
    /// excuting the command with escalated privileges, and return the output
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
    ///     let output = elevated_cmd.output().unwrap();
    /// }
    /// ```
    pub fn output(&self) -> Result<Output> {
        let args = self.cmd.get_args()
            .map(|c| c.to_str().unwrap().to_string())
            .collect::<Vec<String>>();
        let lpparameters = if args.is_empty() {
            PCWSTR::null()
        } else {
            let arg_str = args.join(" ");
            PCWSTR(HSTRING::from(arg_str).as_ptr())
        };

        let r = unsafe { ShellExecuteW(HWND(0), w!("runas"), &HSTRING::from(self.cmd.get_program()), lpparameters, PCWSTR::null(), SW_HIDE) };
        // https://learn.microsoft.com/en-us/windows/win32/api/shellapi/nf-shellapi-shellexecutew#return-value
        if r.0 < 32 {
            bail!("error: {:?}", r);
        }
        Ok(Output {
            status: ExitStatus::from_raw(r.0 as u32),
            stdout: Vec::<u8>::new(),
            stderr: Vec::<u8>::new(),
        })
    }
}
