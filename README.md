# Game of Life in Rust

A simple Game of Life implementation in Rust for practicing Rust skills.

## Based on
[Rust WASM Game of Life](https://rustwasm.github.io/book/game-of-life/implementing.html)

## Rendering
[Macroquad](https://macroquad.rs/)

## How to Run
```sh
cargo run
```

## Performance Needs Work
Right now, large worlds slow things down. Some ideas for optimization:
- Parallelization
- Better rendering techniques
- Reduce unnecessary cloning & copying
