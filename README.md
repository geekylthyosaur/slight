# slight

Full-featured Linux utility to control backlight brightness.

## Build
1. [Install rustc & cargo](https://www.rust-lang.org/tools/install).
2. Clone this repository `git clone https://github.com/geekylthyosaur/slight.git`.
3. Build using `cargo build --release`.
4. Get the binary from `target/release/slight`.

## Features
- ***Precise Brightness Control***: Just type your `[INPUT]`, and now you have absolute control over your device's brightness. Set an exact value using `n`, or make incremental adjustments with `+n` or `-n`. Want a relative setting? Use percentages, like `n%`, or change it relatively with `+n%` or `-n%`. And guess what? The `n` can even be a floating-point number!

- ***Device Selection***: The `--id <ID>` option allows you to tailor the brightness to specific device. Easily identify and select device using the next feature.

- ***Explore Available Devices***: Curious about the devices at your disposal? Use `--list [<LIST>...]` to unveil all available devices or view specific ones by providing their IDs.

- ***Illuminating Exponentials***: Want a more granular brightness adjustment? The `--exponent [<EXPONENT>]` option enables you to use exponential ranges, providing fine-tuned control. Don't worry; if you don't specify an exponent, we'll gracefully apply the default value of `4.0`.

- ***Flexible Output Options***: Choose your preferred destination for brightness adjustments. Whether you want to keep an eye on the changes in your terminal or utilize system file operations, we've got you covered with the `--stdout` option.

- ***Toggle the Light***: Sometimes, you just need a quick change. The `--toggle [<TOGGLE>]` option lets you switch between just two available values (0/1). No fuss, no complexity - just swift toggling. Moreover, we've enriched this feature with the ability to specify the possible values as on or off.

- ***Insightful Verbose Mode***: For those who crave knowledge, the `--verbose` option sheds light on every action the CLI takes. Stay informed and enjoy the transparency.

- ***Helpful Assistance***: Feeling a little lost? Use `--help` for a quick summary of the commands and options at your disposal.

- ***Stay Updated***: Wondering which version you're using? A quick `--version` will reveal the current version, keeping you informed and up-to-date.

## Permissions
To set up udev rules to manage the kernel sysfs permissions, perform the following steps:
1. Put the [90-backlight.rules](90-backlight.rules) file into `/etc/udev/rules.d/` or any other location used to store udev rules on your system.
1. Make sure that your user is part of the `video` group.
1. Reload your udev rules or reboot.

Alternatively, the above steps are not required if your system uses `elogind` or `systemd-logind` as they provide a safe interface for unprivileged users to control device's brightness through `dbus`.
