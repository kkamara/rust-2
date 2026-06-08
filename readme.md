# rust-2

This repository follows Boris Paskhaver's course "Learn to code with Rust" at https://www.udemy.com/course/learn-to-code-with-rust .

## Notes

```

```

## Requirements

* [Rust](https://rust-lang.org/learn/get-started)

## Running the app

```bash
cargo run
cargo r
```

A binary will be produced to the `./target/debug` directory.

## Building the app (debugging version)

```bash
cargo build
cargo b
```

A binary will be produced to the `./target/debug` directory.

## Checking code compiles

```bash
cargo check
cargo c
```

No binary will be produced.

## Building the app (production version)

```bash
cargo build --release
cargo b -r
```

A binary will be produced to the `./target/release` directory.

## Cargo init a new project

```bash
cargo new [app_name]
```

Creates `src/main.rs` and `Cargo.toml` files. If you're not already within a Git folder, a `.gitignore` file and an empty `.git/` directory will be created.

When within a Git folder and you want your new Cargo project to be version-controlled, run `cargo new --vcs git [app_name]` .

## Cargo init an empty project directory

```bash
cargo init
```

Creates `src/main.rs` and `Cargo.toml` files. If you're not already within a Git folder, a `.gitignore` file and an empty `.git/` directory will be created.

When within a Git folder and you want your new Cargo project to be version-controlled, run `cargo init --vcs git` .

## Create a library

```bash
cargo new --lib [package-name]
```

## Clear /target directory

```bash
cargo clean
```

## Build package's documentation

```bash
cargo doc
cargo d
```

## Run a file in the src/bin folder

```bash
cd variables-and-mutability
cargo r --bin intro_to_variables
```

## Run src/main.rs when src/bin files are present

```bash
cd variables-and-mutability
cargo r --bin variables-and-mutability
```

## Misc.

See "Rust 1" at https://github.com/kkamara/rust .
