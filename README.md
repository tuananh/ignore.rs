ignore
======

Generate `.gitignore` file.

## Usage

```
ignore 0.1.0

USAGE:
    ignore [OPTIONS] --keywords <keywords>...

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -k, --keywords <keywords>...    
    -w, --write <write>             Create or update .gitignore file [default: false]
```

To generate `.gitignore` file for `macos` and `rust`, do `ignore --keywords macos node --write true`

## License

MIT
