# Zoxide

zstd written in rust

## Why?

This is a pet project so that I can learn more about rust and compression algorithms in general. It's primary intention is not performance
but clear and easy to understand code.

## Getting started

Run the project with
```shell
cargo run -- filehere
```

## Note

- While the [zstd IETF standard](https://datatracker.ietf.org/doc/html/rfc8878) is clear, it's still quite long and difficult to understand. The c reference implementation is also very very long. This project base off of the [pure Go implementation](https://github.com/klauspost/compress/tree/master/zstd) by @klauspost
