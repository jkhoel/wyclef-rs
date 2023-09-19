# Wyclef

Wyclef is a lightweight [Compact Log Event Format (CLEF)](https://clef-json.org/) log parser written in Rust.

## Usage

Assuming you have some CLEF log file, you can output the contents to the terminal by running:
```bash
wyclef somefile.log
```

## Note on Logging levels

The CLEF spec states that the level property can be both strings and numbers. In order to support coloring of the output, Wyclef currently supports the following case-insensitive strings for levels, or the corresponding number value:

| String Level | Number Level | Color      |
|--------------|--------------|------------|
| Verbose      | 1            | White      |
| Debug        | 2            | Cyan       |
| Information  | 3            | Green      |
| Warning      | 4            | Yellow     |
| Error        | 5            | Red        |
| Fatal        | 6            | Red + Bold |
