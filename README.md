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

- Rust is a statically typed language
- The compiler can infer types for variables based on their usage

Scalar types:

##### Integer

- signed values: i8, i16, i32, i64, i128, isize
- unsigned values: u8, u16, u32, u64, u128, usize
- literals
  - decimal: 98_222
  - hex: 0xff
  - octal: 0o77
  - binary: 0b1111_0000
  - byte: b'A'

```rust
fn main() {
    let number: u32 = 42;   // type specified explicitly
    let number_auto = 42;   // type inferred (integer)

    let number_convert = "42".parse().expect("Not a number!"); // incorrect: type must be specified
    let number_convert_type: u32 = "42".parse().expect("Not a number!"); // type specified explicitly
}
```

##### Floating point (f32, f64)

```rust
fn main() {
    let x = 2.0;        // f64
    let y: f32 = 3.0;   // f32
}
```

##### Boolean (bool)

```rust
fn main() {
    let t = true;
    let f: bool = false; // with explicit type annotation
}
```

##### Character (char)

- 4 bytes
- Unicode Scalar Value

```rust
fn main() {
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
}
```

##### Tuples (..., ...)

```rust
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);    // element types defined explicitly
    let tup2 = (500, 6.4, 1);                   // element types inferred

    // destructuring - pattern matching
    let (x, y, z) = tup;

    // destructuring - direct access to values
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
}
```

##### Array [..., ...]

```rust
fn main() {
    let a: [i32; 5] = [1, 2, 3, 4, 5];  // element type and number defined explicitly
    let b = [1, 2, 3, 4, 5];            // element type and number inferred
    let c = [3; 5];                     // 3 repeated 5 times: [3, 3, 3, 3, 3]

    // element access
    let first = a[0];
    let second = a[1];

    // invalid element access
    let element = a[10];                // compiles fine; panics at runtime
}
```

##### Exercise 1 (average)

- following the above steps, create a new project
- in main:
  - declare 3 numbers
  - compute the **average** of them and store the result in a variable
  - display the result with

    ```rust
    println!("Result: {}", result);
    ```

#### Functions

- may have a return type or not
- may have none, one or more parameters
- the body is made up of a series of **statements** (no value returned)
- optionally, the body can end in an **expression** (returns a value)
- `WARNING`: **expressions** do not include ending **semicolon**; if semicolon is added to the end of an expression, it is turned into a **statement** and it won't return any value

##### Function without parameters, no return value

```rust
// definition
fn my_func() {
    println!("my_func");
}

// call
my_func();
```

##### Function with parameters, no return value

In a function's signature, the type of the parameters must always be declared.

```rust
// definition
fn my_func1(x: i32) {
    println!("my_func1, x: {}", x);
}

fn my_func2(x: i32, y: u8) {
    println!("my_func2, x: {}, y: {}", x, y);
}

// call
my_func1(42);
my_func2(1, 2);
```

##### Function with parameters, with return value

```rust
// definition
fn my_func(x: i32) -> bool {
    println!("my_func, x: {}", x);

    // explicit return (statement, must end with semicolon)
    return x == 1;
}

fn my_func2(x: i32) -> bool {
    println!("my_func, x: {}", x);

    // implicit return (expression, no ending semicolon)
    x < 5
}

// call
let result = my_func(0);
```

##### Exercise 2 (average with functions)

- modify the code from Exercise 1 (average)
- extract the computing of average in a function
- call the function for multiple sets of values

#### Conditions (if expressions)

- **if** is an expression, thus returning a value
  - the returned value is the one returned from the taken branch
  - both branches must return values of the same type
  - if the branches don't return a value, the return is **()** called **unit type**
- the condition **must** be a **bool**

```rust
fn main() {
    let val = 9;

    // if without else
    if val > 0 {
        println!("positive value {}", val);
    }

    // if else
    if val < 10 {
        println!("{} is a digit", val);
    } else {
        println!("{} is not a digit", val);
    }

    // multiple cases
    if val < 10 {
        println!("{} is lower than 10", val);
    } else if val < 100 {
        println!("{} if lower than 100", val);
    } else if val == 100 {
        println!("{} is 100", val);
    } else {
        println!("{} is greater than 100", val);
    }

    // incorrect - the condition must be bool => compilation error
    if val {
        println!("condition is true");
    }

    // assign the value of if to a variable
    let result = if true {
        5
    } else {
        6
    }; // note the ending semicolon


    // incorrect - the branches return different types => compilation error
    let result = if true {
        5       // integer type
    } else {
        "abc"   // string type
    };
}
```

#### Loops

##### loop

- **infinite loop**: repeats executing the body until explicitly stopped
- options to stop the execution
  - kill the program (**CTRL + C**)
  - use **break** in the code
- a value can be returned after **break**

```rust
fn main() {
    loop {
        println!("infinite loop");
    }
}

fn iterate_x_times(x: u32) {
    let mut i = 0;

    let result = loop {
        if i == x {
            break i;
        }

        println!("Iteration {}", i);

        i += 1;
    };

    println!("result is {}", result);
}
```

##### while

- loop while condition is true, then break
- equivalent to **loop** with **break** after an **if** that checks the condition
- does not return a value

```rust
fn iterate_x_times_while(x: u32) {
    let mut i = 0;

    while i < x {
        println!("Iteration {}", i);
        i += 1;
    };
}
```

##### for

- iterate over a collection's elements
- equivalent to **while**, checking if there are more elements
- does not return a value

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];

    for n in a.iter() {
        println!("value {}", n)
    }
}

fn iterate_x_times_for(x: u32) {
    for i in (0..x) {
        println!("Iteration {}", i);
    }
}
```

##### Exercise 3 (prime numbers)

- define an **array** variable that contains some **u32** values
- display for each number if it is prime or not (a number is prime if it is divisible only by 1 and itself)

#### Strings

- type: **String**

```rust
fn main() {
    let literal = "hello";              // literal string
    let mut s = String::from(literal);  // string object

    s.push_str(" world!");              // append string

    println!("{}", s);

    let s1 = s.clone();                 // duplicate the string data
}
```

#### Ownership

#### Structs

- similar with tuples, but have names for the fields
- the whole structure must be declared **mutable** or not; there is no control per field

```rust
// definition
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// instantiation
let mut user = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
};

// member access
let active = user.active;
user.email = String::from("other@example.com");

// instantiate using variables with the same name as the fields
let username = String::from("user");
let email = String::from("email");

let user1 = User {
    username,           // equivalent to username: username
    email,              // email: email
    active: true,
    sign_in_count: 1,
};

// instantiate with some values from other instance
let user2 = User {
    username: String::from("foo"),
    email::String::from("bar"),
    ..user1                     // take the remaining values from user1
}

// try to display the struct
println!("user1: {}", user1);   // error

// display using debug; must also add #[derive(Debug)] before struct User
println!("user1: {:?}", user1);
```

##### Exercise 4 (rectangle area)

- define a structure to hold a **Rectangle**
- instantiate one **Rectangle**
- display the **Rectangle**
- write a function to compute the area of the **Rectangle**

#### Enums

- define the possible values for a type

```rust
// declaration
enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

// better alternative
enum IpAddrEnum {
    V4(String),
    V6(String),
}

// the variants can contain any type
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let v4 = IpAddrKind::V4;
    let v6 = IpAddrKind::V6;

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let home2 = IpAddrEnum::V4(String::from("127.0.0.1"));
}
```

##### The Option enum

- special enum, widely used
- alternative to returning a **Null** value
- **Null** means the value is not present, not a value that is invalid

```rust
// T is replaced with any type we need when using the enum
enum Option<T> {
    Some(T),
    None,
}

fn main() {
    let some_number = Some(5);
    let some_string = Some("a string");

    let no_number: Option<i32> = None;  // must specify the type, the compiler cannot infer it

    // Option<T> cannot be used directly as T
    // the None case must be explicitly handled when the value is actually used
    let sum = 42 + some_number;         // error
}
```

#### Pattern matching

- compare a value against possible patterns; the code corresponding to the first pattern that matches is executed
- returns the value from the branch that was taken
- all the branches must return values of the same type
- the compiler checks that all the possible cases are handled
- useful for destructuring types

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

- the containing values can be used in match branches

```rust
enum IpAddr {
    V4(String),
    V6(String),
    Wrong
}

fn ip_to_string(ip: IpAddr) -> String {
    match ip {
        IpAddr::V4(ipv4_string) => ipv4_string,
        IpAddr::V6(ipv6_string) => ipv6_string,
        _ => String::from("wrong format"),          // default case; matches anything
    }
}
```

- get inner value for Option\<T>

```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

let five = Some(5);
let six = plus_one(five);
let none = plus_one(None);
```

##### if let

- more concise way to write a match where all branches but one are ignored
- useful when we want to do something only on one branch, the other being
  - `_ => ()`
- can optionally have an **else**

```rust

// match
let some_u8_value = Some(0u8);
match some_u8_value {
    Some(3) => println!("three"),
    _ => (),
}

// equivalent if let
if let Some(3) = some_u8_value {
    println!("three");
}
```

#### Crates

#### Modules

#### Collections

#### Error handling

#### Generic types

#### Traits

#### Tests

#### Conversions

- Sometimes, we have an object of A, and we need to turn it into an object of type B.
- Conversion is expressed by implementing `From<A>` for B (or, rarely, implementing `Into<B>` for A)
- After we implement `From`, we call the conversion code with `B::from(a)`

If we implement `From<A> for B`, then `Into<B> for A` is also automatically implemented.

Docs:

- From: <https://doc.rust-lang.org/core/convert/trait.From.html>
- TryFrom: <https://doc.rust-lang.org/core/convert/trait.TryFrom.html>

```rust
#[derive(Debug, Clone)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

#[derive(Debug)]
struct Cents(u32); // alternative way to define a struct, like a tuple but named

impl From<Coin> for Cents {
    fn from(coin: Coin) -> Self {
        let value = match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        };

        // return an object of our Self type (i.e. Cents), constructed from coin
        Cents(value)
    }
}

let coin = Coin::Quarter;

let cents_1 = Cents::from(coin.clone()); // this now works !
println!("converted: {:?}", cents_1);

let cents_2: Cents = coin.clone().into(); // this works automatically
println!("also converted: {:?}", cents_2);
```

Some implementations of `From` and `Into` are predefined, on types like `String` and `&str`

```rust
let s1: &str = "text";
let s2 = String::from(s1); // call impl From<str> for String
let s3: String = s1.into(); // convert s1 into whatever is needed

let large_val = 12345;
println!("conversion result: {:?}", u8::try_from(large_val));
```

##### Conversions that can fail

`From` is for conversions that always work. If our conversion can fail we use `TryFrom`.

```rust
use std::convert::TryFrom;

impl TryFrom<Cents> for Coin {
    type Error = String; // we must define the type returned on error

    fn try_from(cents: Cents) -> Result<Self, Self::Error> {
        let Cents(val) = cents;
        match val {
            // report the Coin for good values in Ok
            1 => Ok(Coin::Penny),
            5 => Ok(Coin::Nickel),
            10 => Ok(Coin::Dime),
            25 => Ok(Coin::Quarter),

            // report Err (of the type defined above, String) for values that are not coins
            x => Err(format!("there is no coin that is worth {} cents", x)),
        }
    }
}

println!("good: {:?}", Coin::try_from(Cents(10)));
println!("bad: {:?}", Coin::try_from(Cents(15)));
```
