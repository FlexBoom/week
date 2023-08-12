# Week

[![build](https://github.com/FlexBoom/week/actions/workflows/rust.yml/badge.svg?event=push)](https://github.com/FlexBoom/week/actions/workflows/rust.yml)

The main purpose of `week` is to output the current week number.

You can also output the start and end dates of the week. It is possible to get week number, start and end dates as JSON.

The most basic way to use week:

```
$ week
31
```

Use any of the following options:

```
$ week -h
Show week number dy default with options for further information about the week.

Usage: week [OPTIONS]

Options:
  -o, --omit     Do not show week number
  -d, --dates    Show start and end dates for the week
  -j, --json     Output everything as JSON and ignore all other options
  -h, --help     Print help
  -V, --version  Print version
```

## License

Week is licensed under either of the following licenses, you choose which:

* [Apache License, Version 2.0](https://www.apache.org/licenses/LICENSE-2.0)
* [MIT License](https://opensource.org/licenses/MIT)
