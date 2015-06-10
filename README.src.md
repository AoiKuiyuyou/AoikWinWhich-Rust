[:var_set('', """
#/ Compile command
aoikdyndocdsl -s README.src.md -n aoikdyndocdsl.ext.all::nto -g README.md
""")]\
[:var_set('source_file_name', 'aoikwinwhich.rs')]\
[:var_set('source_file_url', '/src/aoikwinwhich/aoikwinwhich.rs')]\
[:var_set('program_file_url', './aoikwinwhich.exe')]\
[:HDLR('heading', 'heading')]\
# AoikWinWhich
[AoikWinWhich](https://github.com/AoiKuiyuyou/AoikWinWhich) written in Rust.

Tested working with:
- Rust 1.0.0
- Windows 8.1
- Windows earlier versions should work but not tested

## Table of Contents
[:toc(beg='next', indent=-1)]

## Setup
Clone this git repository to local:
```
git clone https://github.com/AoiKuiyuyou/AoiWinWhich-Rust
```

In the local repository directory, the source file is
[[:var('source_file_name')]]([:var('source_file_url')]).

Use program **rustc** to compile:
```
rustc src/aoikwinwhich/aoikwinwhich.rs
```
- The generated program file is **[:var('program_file_url')]**.

## Usage
See [usage](https://github.com/AoiKuiyuyou/AoikWinWhich#how-to-use) in the
general project [AoikWinWhich](https://github.com/AoiKuiyuyou/AoikWinWhich).
