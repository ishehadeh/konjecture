# Kōnjecture

Tools for analyzing the game [Kōnane](https://en.wikipedia.org/wiki/K%C5%8Dnane).

## Getting Started


### Tooling

#### Rust
This program is written in the [Rust](https://www.rust-lang.org/) programming language.
Official instructions to install Rust can be found at [rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).

If you're unfamiliar with Rust, but want to take a look at the source code, the [Rust Language Cheat Sheet](https://cheats.rs/) may be helpful.

#### Git

If you would like to help develop the program, you also need [git](https://git-scm.com/), which we use for version control.
Installation instructions for git can be found at [git-scm.com/downloads](https://git-scm.com/downloads).

The main repository is hosted in [GitHub](https://github.com/ishehadeh/konjecture).
To submit patches you'll need a GitHub account.

For basic git and GitHub usage, refer to

1. [Git Basics Episode 3 - Get Going with Git](https://git-scm.com/video/get-going)
2. [Github - Get Started](https://docs.github.com/en/get-started/start-your-journey/about-github-and-git)


### Download

If you have git installed, download the source code by running

```sh
git clone git@github.com:ishehadeh/konjecture
```

Otherwise, you can download the source as a zip file from this here: [main.zip](https://github.com/ishehadeh/konjecture/archive/refs/heads/main.zip).


### Applications

Currently there is only one application: `konane2canonical`.

#### konane2canonical

This program takes a game of Konane and puts it in [canonical form](https://en.wikipedia.org/wiki/Combinatorial_game_theory#Overview), using the [cgt](https://crates.io/crates/cgt) library.
All games are played on a 16x16 grid.

**Input**: There should be a file in the program's working directory (usually you're executing it from) called `konane.txt`.
An example of this file is included in with the source code. It should be 16 lines, each line with 16 characters, where

- `_` is an empty tile
- `x` is a tile with white stone
- `o` is atile with black stone

**Output**: Here is a list of common outputs. Note all results assume perfect strategy.
- If the output is `> 0`, then black wins (`o`)
- If the output is `< 0`, then white wins (`x`)
- If the output begins with `*` than the first player to make a move wins
- If the output is `0` the second player to make a move wins
- See [Wikipedia - Combinatorial Game Theory, Game Abbreviations](https://en.wikipedia.org/wiki/Combinatorial_game_theory#Game_abbreviations) for more


Run "konane2canonical" with the command:

```sh
cargo run --release --features cgt,rayon konane2canonical
```
