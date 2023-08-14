# Storyteller

Storyteller is a high-level, dynamically-typed programming language, designed to make your code look like rich, emotional narrative. Gone are the days of dry, sterile syntax. With Storyteller, your programs will start brimming with life.

## Quick Start

The Storyteller compiler and interpreter binary can be downloaded from [here](https://github.com/dhruvjimulia-sys/storyteller/releases/download/v0.1.0/storyteller).

After adding this storyteller binary to the `PATH` environment variable, you can invoke the interpreter on a `.story` file by using the following command. For instance, you can run `sherlock_holmes.story` as follows:
```_
./storyteller sherlock_holmes.story
```
However, in order to use Storyteller in high-performance, low-latency applications, you can also use the Storyteller compiler, which will compile the Storyteller program to fast, efficient C:
```
./storyteller sherlock_holmes.story sherlock_holmes.c
```
The resulting C file can then be compiled to assembly using a C compiler like `gcc` or `clang`.

## Learning Storyteller
Currently, the best resource to learn the Storyteller language is the [Storyteller Language Reference](https://github.com/dhruvjimulia-sys/storyteller/wiki/The-Storyteller-Language-Specification).

For an example of a Storyteller program, you can look at the following [FizzBuzz program](https://github.com/dhruvjimulia-sys/storyteller/blob/main/examples/advanced/fizzbuzz.story), designed to look like the intense, riveting story of Cinderella.

## Build From Source
In order to build the compiler from source, you would need `cargo`, the Rust package manager. The current version of `cargo` used is `v1.70.0`.

After cloning this repository, you can build the compiler from source by using the following command in the root directory of the project:

```
cargo build --release
```

This will build the `storyteller` compiler binary in the `target/release/` directory. You can now use this binary to compile (and interpret) Storyteller programs, as described in the Quick Start section.

Lastly, to run unit and integration tests on the interpreter and compiler, run the following command in the root directory of the project: 
```
cargo test --release
```
In order to run the tests, you would require `gcc` on your machine. The current version of `gcc` is `11.4.0`.