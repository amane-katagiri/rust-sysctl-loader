# rust-sysctl-loader

Yet another sysctl.conf(5) loader.

## Description

This library supports the following specification of sysctl.conf(5), and reads a file or string and returns a HashMap. It also provides an executable that reads a file or standard input from the command line and outputs it in JSON format.

- A line `token = value` are interpreted as value `value` with key `token`.
- If a token has `.`, it is interpreted as a hierarchical structure separated by `.`.
- The same tokens are overwritten by the last value.
- Tokens which begin or end with a `.` or has continuous `.` are considered an invalid hierarchical structure and failed.
- Tokens which begin with a `-` are considered an invalid format and ignored.
- Lines which begin with a `#` or `;` are considered comments and ignored.
- Whitespace before and after a token or value is ignored.
- Blank lines are ignored.

## Usage

### CLI

```sh
cargo run /path/to/sysctl.conf
```

### Library

- `rust_sysctl_loader::parse_sysctl_conf_str(&str)`: Parser for sysctl.conf(5) string into SysctlParameterHashMap
- `rust_sysctl_loader::SysctlParameterHashMap`: Recursive HashMap representing sysctl.conf(5)
- `rust_sysctl_loader::SysctlParameterValue`: Enum for V(literal string value) or M(child HashMap)

## License

MIT License

## Author

Ryunosuke Ito
