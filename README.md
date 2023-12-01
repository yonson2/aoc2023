# Advent of Code 2023

Rust solutions for this years aoc.


## Project structure

```
.
├── Cargo.lock
├── Cargo.toml
├── LICENSE
├── data
│   └── day01
└── src
    ├── day01.rs
    ├── lib.rs
    ├── main.rs
    ├── timer.rs
    └── tools.rs
```


`main.rs` is the entrypoint of the application, I recommend then going to `lib.rs` and exploring the modules as they are defined to get a grasp of the project and the solutions for the problems.

Just `cargo run` to get all of the solutions or `cargo test` to run the tests, which should be self-contained in each module.
