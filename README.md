# Seek 

A no-nonsense grep-like tool.

Seek is a search tool designed for modern terminals and integrations with other software.
It provides a simple and efficient way to search through files and directories, similar to the traditional `grep` command, but with enhanced features and usability.

## Features

- Simple and intuitive command-line interface.
- Fast as possible search performance.
- Enhanced user experience with modern terminal capabilities.

## Installation

The recommended way to install Seek is to use `cargo`.

```shell 
cargo install seek-finder

# And you can start using it!

echo "Hello world!" | seek Hello
```

## Usage

You can use `seek` as you use `grep`. It accepts piped input and then performs search by given pattern:

```shell
cat Cargo.lock | seek clap
```

If you want to integrate it with another program then you can get output in JSON format with `-j` flag.

```shell
cat Cargo.lock | seek -j clap
```


## License 

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
