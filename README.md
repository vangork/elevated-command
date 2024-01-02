# elevated-command

Run command using `sudo`, prompting the user with a graphical OS dialog if necessary. Useful for background rust applications or native [Tauri](https://tauri.app/) apps that need `sudo`.

## Cross-Platform
`elevated-command` provides a native OS dialog prompt on **Windows**, **Linux** and **MacOS**.

## Behavior
On Windows, `elevated-command` will elevate your command using User Account Control (UAC).

On Linux, `elevated-command` will use `pkexec` to show the password prompt and run your command.

On MacOS, `elevated-command` should behave just like the `sudo` command in the shell.

## Reference
[jorangreef/sudo-prompt](https://github.com/jorangreef/sudo-prompt)

## License
[Apache-2.0](https://raw.githubusercontent.com/vangork/elevated-command/main/LICENSE)
