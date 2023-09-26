# Wyclef

Wyclef is a lightweight [Compact Log Event Format (CLEF)](https://clef-json.org/) log parser written in Rust, designed to read the CLEF formatted `*.jlog` files produced by the app - and work on both Windows, Linux and macOS.

## Usage

Wyclef is a [CLI tool](https://en.wikipedia.org/wiki/Command-line_interface) that takes the path to a log file, with CLEF formatted events, as a single argument.

### Keybindings

| Keybinding     | Description                         |
|----------------|-------------------------------------|
| `q`            | Quit the program                    |
| `Up`           | Scroll up                           |
| `Down`         | Scroll down                         |
| `Up + Shift`   | Scroll up by 10 lines               |
| `Down + Shift` | Scroll down by 10 lines             |
| `Enter`        | Go to the top of the log            |
|                |                                     |
| TODO:          |                                     |
| `??`           | Go to the bottom of the log         |
| `f`            | Toggle filtering of events by level |

If you need search functionality within the window, consider using a terminal that supports it.

### Windows

Build the project and store the resulting binary somewhere on your computer. You can also store it in a folder that is not part of your PATH, open your terminal and then run it by specifying the full path to the program. Example:

```powershell
C:\Users\username\Downloads\wyclef.exe somefile.log
```

Optionally, if you want to be able to run the program from anywhere, then store it in a folder that is part of your PATH - or add the folder to your environment variables as [explained in this article](https://www.c-sharpcorner.com/article/how-to-addedit-path-environment-variable-in-windows-11/).

### MacOS and Linux

Build the project and store the resulting binary somewhere in your path, e.g. `/usr/local/bin`. Then run from your terminal:

```bash
wyclef somefile.log
```

You if you want to store the program somewhere else, then you can add it to your ``.bashrc`` or `.zshrc` file by making an alias:

```bash
alias wyclef="/usr/local/bin/wyclef"
```

or to your PATH by adding the following to your ``.bashrc`` or `.zshrc` file:

```bash
export PATH="$PATH:/path/to/wyclef"
```

Then run the program like described above.

## Note on Logging levels

The CLEF spec states that the level property can be both strings and numbers. In order to support coloring of the output, Wyclef currently supports the following case-insensitive strings for levels, or the corresponding number value:

| String Level | Number Level | Color      |
| ------------ | ------------ | ---------- |
| Verbose      | 1            | White      |
| Debug        | 2            | Cyan       |
| Information  | 3            | Green      |
| Warning      | 4            | Yellow     |
| Error        | 5            | Red        |
| Fatal        | 6            | Red + Bold |
