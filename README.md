# rusty-advent

Heyo! This is Niche's Advent-of-Code repo for 2021, the Year of our Lord. Written in Rust!

## Tools

Some resources I've enjoyed using while learning Rust:

 - https://www.youtube.com/c/JonGjengset <- Jon Gjengset, a fantastic Rust streamer!
 - https://fasterthanli.me/series/advent-of-code-2020 <- @fasterthanlime's series on tackling Advent of Code w/Rust in 2020
 - https://doc.rust-lang.org/book/ <- The one an only (not really) Rust book

Some tips and tricks I'll be utilizing during this challenge:
 - VSCode with the:
   - `rust-analyzer` extension (https://rust-analyzer.github.io/), and enable Clippy's "check on save" feature (https://github.com/rust-lang/rust-clippy)
     - Add to VSCode: `"rust-analyzer.checkOnSave.command": "clippy",`
   - `Error Lens` extension (https://marketplace.visualstudio.com/items?itemName=usernamehw.errorlens)
 - `cargo install cargo-edit`
   - So we can install `anyhow` with `cargo add anyhow` (improved Error handling / messaging)

## Notes

Ideas / tidbits / etc. from each (or some) days(s)

### Day 1: Sonar Sweep
 - Newlines!! Trim the last newline of the output! I'm positive this happened to me last year too...
 - For a while I was fighting with a function called `tuple_combinations`, which I thought I wanted...but nope. I wanted `windows` instead. Reading the docs for a function (and looking at examples) is a smart thing to do. I'll learn eventually.