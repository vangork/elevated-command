# sudo-prompt

Run command using `sudo`, prompting the user with a graphical OS dialog if necessary. Useful for background rust applications or native [Tauri](https://tauri.app/) apps that need `sudo`.

## Cross-Platform
`sudo-prompt` provides a native OS dialog prompt on **macOS**, **Linux** and **Windows**.

## Behavior
On Windows, `sudo-prompt` will elevate your command using User Account Control (UAC).

On Linux, `sudo-prompt` will use `pkexec` to show the password prompt and run your command.

On macOS, `sudo-prompt` should behave just like the `sudo` command in the shell.

## Reference
[jorangreef/sudo-prompt](https://github.com/jorangreef/sudo-prompt)

## License
[Apache-2.0](https://raw.githubusercontent.com/vangork/sudo-prompt/main/LICENSE)
