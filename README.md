# slight
Linux utility to control backlight brightness smoothly.

# Build
1. [Install rustc & cargo](https://www.rust-lang.org/tools/install).
2. Clone this repository `git clone https://github.com/geekylthyosaur/slight.git`.
3. Build using `cargo build --release`.
4. Grab binary from `target/release/slight`.

# Usage
```
USAGE:
    slight [OPTIONS] [INPUT]

ARGS:
    <INPUT>
            Input string to control backlight brightness:
              - set exact absolute brightness value: `n`;
              - increase/decrease current brightness by absolute value: `+n`/`-n`;
              - set exact relative brightness value: `n%`;
              - increase/decrease current brightness by relative value: `+n%`/`-n%`.

OPTIONS:
    -e, --exponent [<EXPONENT>]
            Use exponential range with given exponent (or default = 4.0)

    -h, --help
            Print help information

    -i, --id <ID>
            Change brightness of device with given id (use --list to find one)

    -l, --list [<LIST>]
            List all available devices or the one with given id

    -s, --stdout
            Write to stdout instead of sysfs

    -v, --verbose
            Being verbose about what is going on

    -V, --version
            Print version information
```
