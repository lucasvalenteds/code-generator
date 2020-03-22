# Code generator

It just generates codes randomly or based on user input (prefix or seed).

## Examples

**Random code**

```
$ ./code-generator
$ 1285722b5138179844

$ ./code-generator
$ 74eb32328b2fe37b149

$ ./code-generator
$ 08df7b5cf5d1dffe119
```

**Random code with prefix**

```
$ ./code-generator 03
$ 03cf7adccfa1bdc350
 
$ ./code-generator 03
$ 0330fda886e9f6221311
 
$ ./code-generator 03
$ 036bfeb0dfffeb9b118
```

**Last two digits for known code**

```
$ ./code-generator 00124b00188a7f68
$ 00124b00188a7f6810
 
$ ./code-generator 00124b00188a7f68
$ 00124b00188a7f6810
 
$ ./code-generator 00124b00188a7f68
$ 00124b00188a7f6810
```

## How to run

![rust] ![cargo]

| Description | Command |
| :--- | :--- |
| Run tests | `cargo test`
| Run for development | `cargo run` |
| Generate the binary | `cargo build --release` |

[rust]: https://img.shields.io/badge/rustc-1.43-458AC9.svg?style=for-the-badge "Rust 1.43"
[cargo]: https://img.shields.io/badge/cargo-1.43-E8B24F.svg?style=for-the-badge "Cargo 1.43"
