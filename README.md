# Wyclef

Wyclef is a lightweight [Compact Log Event Format (CLEF)](https://clef-json.org/) log parser written in Rust.

## Usage

Assuming you have some CLEF log file, you can output the contents to the terminal by running:
```bash
wyclef somefile.log
```

## Limitations

While the CLEF spec states that the level property can be both a string or a number, Wyclef only currently supports case-insensitive strings of levels `Error`, `Warning` and `Verbose`. All other types will default to `Information`.
