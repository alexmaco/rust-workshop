# Rust workshop

## Day 1 - Infrastructure & Basics

### Language overview

### Infrastructure

#### Setup

The Rust compiler (**rustc**), the build and package manager (**cargo**) and other tools can be installed and managed using the **rustup** script

Installing: https://www.rust-lang.org/tools/install

- on Linux/MacOS:

```bash
curl https://sh.rustup.rs -sSf | sh
```

- on Windows: Download and run [rustup](https://win.rustup.rs) and follow the instructions

Updating to the latest version:

```bash
rustup update
```

Verifying the installed version:

```bash
rustc --version
cargo --version
```

#### IDE

Various [IDEs/text editors](https://areweideyet.com/) can be used to edit Rust source files.

We recommend using [Visual Studio Code](https://code.visualstudio.com/)

Install the **Rust (rls)** extension

- code completion
- jump to definition, peek definition, find all references, symbol search
- types and documentation on hover
- code formatting
- refactoring (rename, deglob)
- error squiggles and apply suggestions from errors
- snippets
- build tasks

Other useful extensions:

- **Cargo**
- **Better TOML**
- **Bracket Pair Colorizer**

#### Tools

##### rustfmt

A tool for formatting Rust code.

Documentation can be found [here](https://github.com/rust-lang/rustfmt)

Install:

```bash
rustup component add rustfmt
```

Run in the current directory:

```bash
cargo fmt
```

##### clippy

A tool for static checking Rust code using various lints.

Documentation can be found [here](https://github.com/rust-lang/rust-clippy)

Install:

```bash
rustup component add clippy
```

Run in the current directory:

```bash
cargo clippy
```

#### Documentation

### Hello world

#### Create a new project

New projects can be created using **cargo**

```bash
cargo new --help    # available options
```

```bash
cargo new hello_world
```

##### hello_world/

```bash
.
├── Cargo.toml      # project configuration file
└── src
    └── main.rs     # source code
```

##### Cargo.toml

```toml
[package]
name = "hello_world"
version = "0.1.0"
authors = ["Mircea Urse <mircea.urse@keysight.com>"]
edition = "2018"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
```

##### src/main.rs

```rust
fn main() {
    println!("Hello, world!");
}
```

#### Compile a project

```bash
cd hello_world
cargo build
```

The files resulting after build

```bash
.
├── Cargo.lock          # contains the exact versions of the dependencies used
├── Cargo.toml
├── src
│   └── main.rs
└── target
    ├── debug
    │   ├── ...
    │   ├── hello_world     # the executable
    │   ├── ...
    └── .rustc_info.json
```

#### Run the compiled executable

```bash
target/debug/hello_world
```

#### Build and run the executable in a single step

```bash
cargo run
```

### Language

#### Data types

#### Functions

#### Conditions

#### Loops

#### Ownership

#### Structs

#### Enums

#### Pattern matching

#### Crates

#### Modules

#### Collections

#### Error handling

#### Generic types

#### Traits

#### Tests

## Day 2 - Usage patterns & Real life applications
