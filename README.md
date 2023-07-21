# slight

Linux utility to control backlight brightness smoothly.

## Build
1. [Install rustc & cargo](https://www.rust-lang.org/tools/install).
2. Clone this repository `git clone https://github.com/geekylthyosaur/slight.git`.
3. Build using `cargo build --release`.
4. Grab binary from `target/release/slight`.

## Usage
```
Arguments:
  [INPUT]
          Input string to control backlight brightness
          - set exact absolute brightness value: `n`;
          - increase/decrease current brightness by absolute value: `+n`/`-n`;
          - set exact relative brightness value: `n%`;
          - increase/decrease current brightness by relative value: `+n%`/`-n%`.

Options:
  -i, --id <ID>
          Change brightness of device with given id (use --list to find one)
  -l, --list [<LIST>...]
          List all available devices or the ones with given id
  -e, --exponent [<EXPONENT>]
          Use exponential range with given exponent (or default = 4.0)
  -s, --stdout
          Write to stdout instead of sysfs
  -t, --toggle [<TOGGLE>]
          Toggle value of device with only two available values (0/1)
          [possible values: on, off]
  -v, --verbose
          Being verbose about what is going on
  -h, --help
          Print help (see a summary with '-h')
  -V, --version
          Print version
```

## Permissions
To set up udev rules to manage the kernel sysfs permissions put [90-backlight.rules](90-backlight.rules) into `/etc/udev/rules.d/` or other location of your udev rules and make sure that your user is part of `video` group.

Alternatively make sure your system uses `elogind` or `systemd-logind` (TODO: check if `polkit` is required).