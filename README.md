# rust-sysctl-loader

Yet another sysctl.conf(5) loader.

## Description

This library supports the following specification of sysctl.conf(5), and reads a file or string and returns a HashMap. It also provides an executable that reads a file or standard input from the command line and outputs it in JSON format.

```txt
endpoint = localhost:3000
# debug = true
log.file = /var/log/console.log
log.name = default.log
log.limit = 1024
```

- A line `token = value` are interpreted as value `value` with key `token`.
- If a token has `.`, it is interpreted as a hierarchical structure separated by `.`.
- The same tokens are overwritten by the last value.
- Tokens which begin or end with a `.` or has continuous `.` are considered an invalid hierarchical structure and failed.
- Tokens which begin with a `-` are considered an invalid format and ignored.
- Lines which begin with a `#` or `;` are considered comments and ignored.
- Whitespace before and after a token or value is ignored.
- Blank lines are ignored.

It also supports validating value in sysctl.conf(5). The schema syntax is similar to sysctl.conf(5), but with `=` being `=>`. A line `token => schema_type` are interpreted as type `schema_type` with key `token`. Supported schema types are `string`, `bool`, and `integer`.

```txt
endpoint => string
debug => bool
log.file => string
log.limit => integer
```

- `string` accepts all value.
- `bool` accepts only `"true"` or `"false"`.
- `integer` positive whole numbers such as `"123"`, up to a maximum of u64.

## Usage

### CLI

```sh
cargo run /path/to/sysctl.conf [/path/to/schema.txt]
```

### Library

- `rust_sysctl_loader::sysctl::parse_str(&str)`: Parser for sysctl.conf(5) string into SysctlParameterHashMap
- `rust_sysctl_loader::schema::parse_str(&str)`: Parser for schema string into SchemaHashMap
- `rust_sysctl_loader::validator::validate(SysctlParameterHashMap, SchemaHashMap)`: Validator for sysctl.conf(5) with schema
- `rust_sysctl_loader::hashmap::SysctlParameterHashMap`: Recursive HashMap representing sysctl.conf(5)
- `rust_sysctl_loader::hashmap::SysctlParameterValue`: Enum for V(literal string value) or M(child HashMap)
- `rust_sysctl_loader::schema::SchemaHashMap`: HashMap representing schema
- `rust_sysctl_loader::schema::SchemaType`: Enum for schema type

## License

MIT License

## Author

Ryunosuke Ito
