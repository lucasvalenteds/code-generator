# Code generator

It just generates 18 digit codes randomly or based on user input (prefix or seed).

## Examples

**Random code**

```
$ ./code-generator
$ 8255672eb972161879

$ ./code-generator
$ 71b7f0d843ac077f7f

$ ./code-generator
$ 5504b3cdd6f827fb75
```

**Random code with prefix**

```
$ ./code-generator 03
$ 03081be8e62c9c16b0
 
$ ./code-generator 03
$ 030dfa90085129576f
 
$ ./code-generator 03
$ 0310abb453c2ca9a09
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
| Format source code | `cargo fmt` |

[rust]: https://img.shields.io/badge/rustc-1.51-458AC9.svg?style=for-the-badge "Rust 1.51"
[cargo]: https://img.shields.io/badge/cargo-1.51-E8B24F.svg?style=for-the-badge "Cargo 1.51"
