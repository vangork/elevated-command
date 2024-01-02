# elevated-command

Run command using `sudo`, prompting the user with a graphical OS dialog if necessary. Useful for background rust applications or native [Tauri](https://tauri.app/) apps that need `sudo`.


## Cross-Platform
`elevated-command` provides a native OS dialog prompt on **Windows**, **Linux** and **MacOS**.


## Behavior
On Windows, `elevated-command` will elevate your command using User Account Control (UAC).

On Linux, `elevated-command` will use `pkexec` to show the password prompt and run your command.

On MacOS, `elevated-command` should behave just like the `sudo` command in the shell.

![Windows](https://raw.githubusercontent.com/jorangreef/sudo-prompt/master/windows.png)

![Linux](https://raw.githubusercontent.com/jorangreef/sudo-prompt/master/linux.png)

![macOS](https://raw.githubusercontent.com/jorangreef/sudo-prompt/master/macos.png)


## Reference
1. [jorangreef/sudo-prompt](https://github.com/jorangreef/sudo-prompt)
2. https://stackoverflow.com/questions/8046097/how-to-check-if-a-process-has-the-administrative-rights/8196291#8196291


## License
Licensed under the [MIT](https://github.com/vangork/elevated-command/blob/main/LICENSE) license.
