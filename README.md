# xkcd password generator

Generate passwords that are easy to remember. Inspired by the xkcd webcomic on passwords:

![xkcd-webcomic](https://imgs.xkcd.com/comics/password_strength.png)

## Word Lists

This project uses the [word lists](https://www.eff.org/deeplinks/2016/07/new-wordlists-random-passphrases) for random passphrases provided by EFF (Electronic Frontier Foundation). EFF's long word list is used by default, though all three of their word lists are available.

## Install

Use `cargo` to install `xkpass`:

```bash
cargo install xkpass
```

Alternatively, build from source:

```bash
cargo build --release
```

## Usage

```bash
USAGE:
    xkpass [OPTIONS]

OPTIONS:
    -c, --case <CASE>              Case to use on the words [default: lower] [possible values: upper, lower, capitalized,
                                   mixed]
    -h, --help                     Print help information
    -l, --list <LIST>              List of words to use for random password generation [default: long] [possible values:
                                   long, short1, short2]
    -n, --number <NUMBER>          Number of words to include in the password [default: 6]
    -s, --separator <SEPARATOR>    A separator to use between words [default: " "]
    -V, --version                  Print version information
```

## License

`xkpass` is licensed under the terms of either the MIT license or the Apache License 2.0.
