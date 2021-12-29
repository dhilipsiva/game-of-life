# game-of-life

Conway's Game of Life

An implementaion of Conway's game of life with Rust, WebAssembly (WASM) and Rayon (Data Parallelism Library)

Wikipedia: [wiki/Conway%27s_Game_of_Life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life)

## Rules

1. Any live cell with fewer than two live neighbours dies, as if by underpopulation.
1. Any live cell with two or three live neighbours lives on to the next generation.
1. Any live cell with more than three live neighbours dies, as if by overpopulation.
1. Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.

These rules, which compare the behavior of the automaton to real life, can be condensed into the following:

1. Any live cell with two or three live neighbours survives.
1. Any dead cell with three live neighbours becomes a live cell.
1. All other live cells die in the next generation. Similarly, all other dead cells stay dead.
