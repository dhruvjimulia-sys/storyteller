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
./storyteller sherlock_holmes.story -c sherlock_holmes.c
```
The resulting C file can then be compiled to assembly using a C compiler like `gcc` or `clang`.

## Build From Source
In order to build the compiler from source, you would need `cargo`, the Rust package manager. The current version of `cargo` used is `v1.70.0`.

After cloning this repository, you can build the compiler from source by using the following command in the root directory of the project:

```
cargo build --release
```

This will build the `storyteller` compiler binary in the `target/release/` directory. You can now use this binary to compile (and interpret) Storyteller programs, as described in the Quick Start section.

