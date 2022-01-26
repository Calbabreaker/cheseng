# Cheseng

A chess engine written in Rust.

## Structure

-   [cheseng](cheseng) directory contains the cheseng crate/library that implements a chess engine.
-   [ui](ui) directory contains the ui executable that uses the cheseng crate for the engine stuff.
-   [cli](cli) directory contains the terminal version of the uithat uses the cheseng crate for the engine stuff.

To build and run ui:

```sh
cargo run -r
```

To build and run cli:

```sh
cargo run -p cheseng-cli -r
```

## Credit

-   Chess pieces sprite sheet from [Wikipedia](https://commons.wikimedia.org/wiki/File:Chess_Pieces_Sprite.svg)
