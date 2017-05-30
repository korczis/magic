# hackrf-rust

## Usage 

### Show global help

```
$ hackrf -h
HackRF wrapper 0.0.1
Tomas Korcak <korczis@gmail.com>
HackRF CLI

USAGE:
    hackrf [FLAGS] <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
    -v, --verbose    Verbose mode

SUBCOMMANDS:
    help     Prints this message or the help of the given subcommand(s)
    info     Show information
    list     List devices
    reset    Reset device
```

### hackrf list

List available HackRF devices

```
$ hackrf list
[
  {
    "index": 0,
    "model": "One",
    "version": {
      "major": 1,
      "minor": 2
    },
    "firmware": "2017.02.1",
    "serial": {
      "part_id": [
        2684406588,
        6308698
      ],
      "serial_no": [
        0,
        0,
        2425906376,
        741960399
      ]
    },
    "serial_string": "0000000000000000909864c82c396acf"
  }
]
```