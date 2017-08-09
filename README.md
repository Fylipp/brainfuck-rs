# brainfuck-rs
[![Build Status](https://travis-ci.org/Fylipp/brainfuck-rs.svg?branch=master)](https://travis-ci.org/Fylipp/brainfuck-rs)

This is a simple Brainfuck interpreter written in Rust. This is my first attempt at using Rust. The CI tests will fail
for as long as `std::ops::RangeInclusive` is only in the nightly build.

## Aim
To aim is to implement a Brainfuck interpreter that has 2^16 cells with 2^8 possible states each. Characters that do
not map to any instruction will be ignored. Please note that the Brainfuck code is treated as ASCII and
exotic unicode characters will probably break the interpreter.

## Building
Since this is a normal crate you can simply do `cargo build`.

## Testing
Like any crate: `cargo test`

## Usage
Use `brainfuck-rs run <BF_FILE>` to execute a Brainfuck file. The `demo` directory contains some basic Brainfuck demos.
To run them directly from cargo you can do `cargo run -- <BF_FILE>`. This will execute the program from the repository
root.
