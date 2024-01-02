# elevated-command

[![crates.io](https://img.shields.io/crates/v/elevated-command?logo=rust)](https://crates.io/crates/elevated-command/)
[![docs.rs](https://docs.rs/elevated-command/badge.svg)](https://docs.rs/elevated-command)

Run command using `sudo`, prompting the user with a graphical OS dialog if necessary. Useful for background rust applications or native [Tauri](https://tauri.app/) apps that need `sudo`.


## Cross-Platform
`elevated-command` provides a native OS dialog prompt on **Windows**, **Linux** and **MacOS**.

![Windows](https://raw.githubusercontent.com/jorangreef/sudo-prompt/master/windows.png)

![Linux](https://raw.githubusercontent.com/jorangreef/sudo-prompt/master/linux.png)

![macOS](https://raw.githubusercontent.com/jorangreef/sudo-prompt/master/macos.png)


## Example
Add the following to your Cargo.toml:

```
[dependencies]
elevated-command = "1.1"
```

In your `main.rs`: 

```
use elevated_command::Command;
use std::process::Command as StdCommand;

fn main() {
    let is_elevated = Command::is_elevated();

    let mut cmd = StdCommand::new("path to the application");
    cmd.arg("some arg");

    let output = if is_elevated {
        cmd.output().unwrap()
    } else {
        let elevated_cmd = Command::new(cmd);
        elevated_cmd.output().unwrap()
    };
}
```
Note: The application should not be `sudo`.

To get started using `elevated-command`, please see the [API reference (docs.rs)](https://docs.rs/elevated-command/).

## Behavior
On Windows, `elevated-command` will elevate your command using User Account Control (UAC).

On Linux, `elevated-command` will use `pkexec` to show the password prompt and run your command.

On MacOS, `elevated-command` should behave just like the `sudo` command in the shell.


## Reference
1. [jorangreef/sudo-prompt](https://github.com/jorangreef/sudo-prompt)
2. https://stackoverflow.com/questions/8046097/how-to-check-if-a-process-has-the-administrative-rights/8196291#8196291


## License
Licensed under the [MIT](https://github.com/vangork/elevated-command/blob/main/LICENSE) license.
