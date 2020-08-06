# GitHub Stars

Fetch repositories and how many ⭐️ they have. Useful if you don't get any likes
on Twitter or Instagram but can write good code. Will probably help if your
coding technique is hype driven so it always uses The Latest and Greatest™
software!

## Usage

Show help for flags and arguments.

```sh
$ github-stars --help
GitHub Stars 0.1
Simon Sawert <simon@sawert.se>
Get stars from GitHub user repositories

USAGE:
    github-stars [OPTIONS] <username>

ARGS:
    <username>    GitHub username

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -t, --threshold <threshold>    Minimum stars to show [default: 1]
```

Fetch user stars. Set threshold to 0 to fetch all repositories no matter if
they're starred or not.

```sh
$ github-stars sharkdp --threshold 100
⭐️ 21939 | bat              | A cat(1) clone with wings.
⭐️ 14500 | fd               | A simple, fast and user-friendly alternative to 'find'
⭐️  5833 | hyperfine        | A command-line benchmarking tool
⭐️  5467 | hexyl            | A command-line hex viewer
⭐️  2910 | pastel           | A command-line tool to generate, analyze, convert and manipulate colors
⭐️  2122 | insect           | High precision scientific calculator with support for physical units
⭐️  1440 | cube-composer    | A puzzle game inspired by functional programming
⭐️  1330 | dbg-macro        | A dbg(…) macro for C++
⭐️   908 | shell-functools  | Functional programming tools for the shell
⭐️   606 | diskus           | A minimal, fast alternative to 'du -sh'
⭐️   426 | great-puzzles    | A curated list of great puzzles
⭐️   358 | vivid            | A generator for LS_COLORS with support for multiple color themes
⭐️   263 | purescript-flare | A special-purpose UI library for Purescript
⭐️   154 | trigger          | Run a user-defined command on file changes

 Total stars: 59254
```
