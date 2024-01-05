# Ferrous

Ferrous is a simple shell implemented in Rust. It supports basic commands, changing the current folder with `cf`, and exiting the shell with `exit`. The shell also supports command chaining using pipes. Additional commands include `copy`, `move`, `remove`, `mem` (memory info), `compare`, `dirsize`, and `tree`.

## Project Structure

You can find the detailed code structure [here](https://github.com/KekmaTime/ferrous/blob/main/src/mods/README.md)

## Prerequisites

Before you can run this shell, you need to install Rust. The easiest way to install Rust is with rustup. Follow the instructions [here](https://rustup.rs/) to install rustup. After you've installed rustup, you can install Rust by running the following command:

```bash
rustup install stable
```

You also need to install the Cargo package manager, which is used to build and run the shell. Cargo is included with rustup, so if you installed Rust with rustup, you should already have Cargo.

## Libraries and Crates

* [which](https://crates.io/crates/which)
* [std::io](https://doc.rust-lang.org/std/io/index.html)
* [std::env](https://doc.rust-lang.org/std/env/index.html)
* [std:path](https://doc.rust-lang.org/std/path/index.html)
* [std::process](https://doc.rust-lang.org/std/process/index.html)
* [std::iter](https://doc.rust-lang.org/std/iter/index.html)
* [std::str](https://doc.rust-lang.org/std/str/index.html)
* [std::fs](https://doc.rust-lang.org/std/fs/index.html)
* [Walkdir](https://github.com/BurntSushi/walkdir)
* [colored](https://docs.rs/colored/latest/colored/)
* [dirsize](https://github.com/KekmaTime/ferrous/blob/main/src/mods/dirsize.rs)

## Building

To build the shell, navigate to the project directory and run the following command:

```bash
cargo build
```

This will compile the shell and produce an executable file in the `target/debug` directory.

## Running

To run the shell, you can use the `cargo run` command:

```bash
cargo run
```

This will start the shell. You should see a `>` prompt where you can enter commands.

## Usage

You can use the shell just like any other shell. Here are some examples of commands you can run:

```bash
> ls
> cf /
> exit
> copy src dest
> move src dest
> remove path
> mem
> compare file1 file2
> dirsize path
> tree path
```

## Contributing

Contributions are welcome! If you have a feature you'd like to add, feel free to open a pull request. If you find a bug, please open an issue.

## Future Work

[Here](https://github.com/KekmaTime/ferrous/blob/main/docs/README.md) are some potential improvements that could be made to the shell.
