# Advent of Code - 2023

An attempt at [Advent of Code](https://adventofcode.com/2023) using [Rust 🦀](https://rustlang.org)!

![[Workflow Status](https://github.com/shaunburdick/advent-of-code-2023/actions/workflows/rust.yml)](https://github.com/shaunburdick/advent-of-code-2023/actions/workflows/rust.yml/badge.svg)

Each day will be setup as a separate item in the [Cargo Workspace](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html).

-   [Day 0](/day-00/) - Calorie Counting (2022 Day 1)
-   [Day 1](/day-01/) - Trebuchet?!
-   [Day 2](/day-02/) - Cube Conundrum
-   [Day 3](/day-03/) - Gear Ratios
-   [Day 4](/day-04/) - Scratchcards
-   [Day 5](/day-05/) - If You Give A Seed A Fertilizer
-   [Day 6](/day-06/) - Wait For It
-   [Day 7](/day-07/) - Camel Cards
-   [Day 8](/day-08/) - Haunted Wasteland
-   [Day 9](/day-09/) - Mirage Maintenance
-   [Day 10](/day-10/) - Mirage Pipe Maze
-   [Day 11](/day-11/) - Cosmic Expansion
-   [Day 12](/day-12/) - Hot Springs

## Environment Setup

To setup your environment:

1. [Install](https://www.rust-lang.org/learn/get-started) Rust
2. Install [Rust Analyzer](https://rust-analyzer.github.io/) in your favorite IDE
3. Install [Just](https://github.com/casey/just): `cargo install just`
4. Install [Cargo Generate](https://github.com/cargo-generate/cargo-generate): `cargo install cargo-generate`
5. Clone this repository
6. Create a new day from template: `just create day-X`
7. Be Merry! 🎄

## Testing

To run tests for all days, run `cargo test --workspace`

To run tests for an individual day X, run `just test day-X`

## Thanks

-   [Christopher Biscardi](https://github.com/ChristopherBiscardi) for his [videos](https://www.youtube.com/@chrisbiscardi) and repository [setup](https://github.com/ChristopherBiscardi/advent-of-code/tree/main/2023/rust)
