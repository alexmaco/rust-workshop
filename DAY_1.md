# Day 1 - Infrastructure & Basics

## Language overview

## Documentation (reference guides)

- the standard library: https://doc.rust-lang.org/std/index.html
- search for external crates: https://crates.io/
- search documentation for a specific crate: https://docs.rs/
- generate documentation for the current project
  - `cargo doc`
- open the generated documentation:
  - `cargo doc --open`
  - or open **index.html** (in `target/doc/<project_name>`)

## Hello world

### Create a new project

New projects can be created using **cargo**

```bash
cargo new --help    # available options
```

```bash
cargo new hello_world
```

#### hello_world/

```bash
.
├── Cargo.toml      # project configuration file
└── src
    └── main.rs     # source code
```

#### Cargo.toml

```toml
[package]
name = "hello_world"
version = "0.1.0"
authors = ["Mircea Urse <mircea.urse@keysight.com>"]
edition = "2018"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
```

#### src/main.rs

```rust
fn main() {
    println!("Hello, world!");
}
```

### Compile a project

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

### Run the compiled executable

```bash
target/debug/hello_world
```

### Build and run the executable in a single step

```bash
cargo run
```

## Language

### Variables and mutability

- by default, the variables are immutable (the value cannot be changed after the initialization)
- the compiler will raise an error if we try to change the value of an immutable variable
- variables can be declared mutable using **mut**

```rust
fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;  // error if mut is missing from the declaration of x
    println!("The value of x is: {}", x);
}
```

### Data types

- Rust is a statically typed language
- The compiler can infer types for variables based on their usage

Scalar types:

#### Integer

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

#### Floating point (f32, f64)

```rust
fn main() {
    let x = 2.0;        // f64
    let y: f32 = 3.0;   // f32
}
```

#### Boolean (bool)

```rust
fn main() {
    let t = true;
    let f: bool = false; // with explicit type annotation
}
```

#### Character (char)

- 4 bytes
- Unicode Scalar Value

```rust
fn main() {
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
}
```

#### Tuples (..., ...)

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

#### Array [..., ...]

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

#### Exercise 1 (average)

- following the above steps, create a new project
- in main:

  - declare 3 numbers
  - compute the **average** of them and store the result in a variable
  - display the result with

    ```rust
    println!("Result: {}", result);
    ```

### Functions

- may have a return type or not
- may have none, one or more parameters
- the body is made up of a series of **statements** (no value returned)
- optionally, the body can end in an **expression** (returns a value)
- `WARNING`: **expressions** do not include ending **semicolon**; if semicolon is added to the end of an expression, it is turned into a **statement** and it won't return any value

#### Function without parameters, no return value

```rust
// definition
fn my_func() {
    println!("my_func");
}

// call
my_func();
```

#### Function with parameters, no return value

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

#### Function with parameters, with return value

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

#### Exercise 2 (average with functions)

- modify the code from Exercise 1 (average)
- extract the computing of average in a function
- call the function for multiple sets of values

### Conditions (if expressions)

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

### Loops

#### loop

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

#### while

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

#### for

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

#### Exercise 3 (prime numbers)

- define an **array** variable that contains some **u32** values
- display for each number if it is prime or not (a number is prime if it is divisible only by 1 and itself)

### Strings

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

### Ownership

### Structs

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

#### Exercise 4 (rectangle area)

- define a structure to hold a **Rectangle**
- instantiate one **Rectangle**
- display the **Rectangle**
- write a function to compute the area of the **Rectangle**

#### Methods

- functions defined within a context of a structure
- the first parameter is the structure instance, **self**
- called using dot, like in other programming languages

```rust
struct Duration {
    seconds: u64,
    description: String,
}

impl Duration {
    fn minutes(&self) -> u64 {
        self.seconds / 60
    }


fn main() {
    let d = Duration {
        seconds: 120,
        description: String::from("test"),
    };

    println!("{} seconds, {} minutes", d.seconds, d.minutes());
}
```

#### Exercise 5 (rectangle area with method)

- change the **area** function from **Exercise 4** to be a method on the struct **Rectangle**

### Enums

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

#### The Option enum

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

### Pattern matching

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

#### if let

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

#### Exercise 6 (time)

- define an enum for various time units (seconds, minutes, hours etc), each containing an integer value
- write a function that receives a time value and computes the number of seconds corresponding to it

### Modules

### Collections

#### Vector: Vec\<T>

- stores elements of the same type, contiguous in the memory
- if multiple types need to be stored in a vector, a vector of **enum** can be used (with inner types)

```rust

// create an empty vector
let v1: Vec<i32> = Vec::new();

// create a vector with initial values
let v2 = vec![1, 2, 3];

// insert elements
let mut v2 = Vec::new();

v2.push(5);
v2.push(6);
v2.push(7);
v2.push(8);

// remove last element
match v2.pop() {
    Some(last) => println!("The last element was {}", last),
    None => println!("Empty vector"),
}

// access elements
let third: &i32 = &v2[2];   // crash if index is out of bounds
println!("The third element is {}", third);

match v2.get(2) {           // returns None if index is out of bounds
    Some(third) => println!("The third element is {}", third),
    None => println!("There is no third element."),
}

// iterate over the values
let v3 = vec![100, 32, 57];
for i in &v3 {
    println!("{}", i);
}

// iterate and modify each element in the vector
let mut v4 = vec![100, 32, 57];
for i in &mut v4 {
    *i += 50;   // add 50 to each element
}
```

#### Hash Map: HashMap\<K, V>

- stores a mapping of keys of type **K** to values of type **V**
- uses a hashing function to determine the place for a specific element
- custom types can be used as keys; the **Hash** trait must be implemented (usually via #[derive(Hash)])

```rust
use std::collections::HashMap;

// create an empty map
let mut scores = HashMap::new();

// insert elements
scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

// overwrite values
scores.insert(String::from("Blue"), 25);

// insert only if the key has no value
scores.entry(String::from("Yellow")).or_insert(50);
scores.entry(String::from("Green")).or_insert(50);

// remove elements
let team_name1 = String::from("Green");
scores.remove(&team_name1);  // returns an Option<V>

// access values
let team_name2 = String::from("Blue");
let score = scores.get(&team_name2);     // returns an Option<&V>

// iterate through the values
for (key, value) in &scores {
    println!("{}: {}", key, value);     // the order is arbitrary
}
```

#### Exercise (number of occurrences)

- declare a vector containing some integers
- compute the number of occurrences for each element in the vector

### Error handling

- rust requires the programmer to acknowledge the existence of errors and treat all the cases where an error may occur; the code won't compile unless all the error cases are handled

#### Unrecoverable errors (panic!)

- when **panic!** is called, the program prints an error message, does some cleanup and quit
- panic can be called direclty by our code or indirectly by other elements that we call
- by default the stacktrace of the program is not displayed at panic; it can be enabled with:
  - `RUST_BACKTRACE=1 cargo run`

```rust
fn main() {
    // direct call
    panic!("crash and burn");

    // indirect call
    let v = vec![1, 2, 3];

    v[99];
}
```

#### Recoverable errors (Result\<T, E>)

- the **Result** enum is defined in the standard library
- shortcuts for the case when the error case should cause panic
  - unwrap
  - expect (similar to unwrap, allows specifying an error message in case of failure)
- errors can be propagated using **?**
  - if the result is **Ok**, the execution continues with the value contained inside
  - if the result is **Error**, the result is returned from the current function, for the calling code to process it
  - it must be called inside a function that returns **Result**

```rust
enum Result<T, E> {
    Ok(T),          // T is the type of the value returned in case of success
    Err(E),         // E is the type of the value returned in case of failure
}
```

```rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };
}
```

- unwrap

```rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt").unwrap();
}
```

- expect

```rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt").expect("Failed to open hello.txt");
}
```

- propagate errors

```rust
use std::io;
use std::io::Read;
use std::fs::File;

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}
```

- simpler and equivalent

```rust
use std::io;
use std::io::Read;
use std::fs::File;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

// shorter
fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}

// even shorter
use std::io;
use std::fs;

fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
```

### Generic types

- reduce duplication and reusability by parametrizing the type in the definitions of functions, structs, enums
- examples:
  - Option\<T>
  - Result\<T, E>
  - Vec\<T>
  - HashMap\<K, V>
- the compiler generates code for each instantiated type

```rust
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let point_int = Point{ x: 1, y: 2 };
    let point_float = Point{ x: 1.2, y: 2.5 };
    let point_wrong = Point{ x: 1, y: 2.5};     // error: x and y must have the same type
}
```

```rust
// does not work yet: all the possible types T must know how to handle '>'
fn largest<T>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}
```

### Traits

- describe the shared behavior of types
- the behavior of a type is the collection of methods that we can call on that type
- group method signatures together (similar to an interface in other languages)
- when implementing the trait on a type all the methods of the trait must be defined
- supported cases:
  - local trait definition, local type definition
  - local trait definition, external type definition (e.g. implement **Summarize** on **Vec\<T>**)
  - external trait definition, local type definition (e.g. implement **Display** on **NewsArticle**)
- unsupported case: external trait definition and external type definition (e.g. implement **Display** on **Vec\<T>**)

```rust
pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
```

- default implementations can be provided in the trait definition
- the implementation on a specific type can skip defining the methods that have default implementations or can override it
- the default implementation cannot be called from the overriden one

```rust
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

impl Summary for NewsArticle {}
```

#### Trait bounds

- use traits in functions definitions

```rust
// these 2 functions are equivalent
// simpler form (impl trait)
pub fn notify(item: impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// the more generic form (trait bound); useful if T must be used in more than one place
pub fn notify<T: Summary>(item: T) {
    println!("Breaking news! {}", item.summarize());
}
```

- specifying multiple trait bounds

```rust
pub fn notify(item: impl Summary + Display) {
    ...
}

pub fn notify<T: Summary + Display>(item: T) {
    ...
}
```

- using the **where** clause to make the declarations clearer

```rust
fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 {
    ...
}

fn some_function<T, U>(t: T, u: U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{
    ...
}
```

- fix the **largest** function defined previously

```rust
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}
```

- methods can be conditionally implemented based on the trait bound

```rust
use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

// always implement method 'new' for Pair<T>
impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self {
            x,
            y,
        }
    }
}

// implement method 'cmd_display' only if T has comparison and printing
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
```

### Tests

- built-in support for unit tests, no need to include a 3rd party framework
- a test is a function annotated with the **test** attribute
  - `#[test]`
- assertions can be checked with:
  - `assert!`
  - `assert_eq!` and `assert_ne!`
- the tests are run with `cargo test`

```bash
$ cargo test
...
running 3 tests
test add::tests_1::adds_2_and_2 ... ok
test add::tests_1::panics_when_called ... ok
test add::tests_1::panics_when_called_specific_message ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn panics() {
    panic!("panic message");
}

#[cfg(test)]        // the tests sit in a test module
mod tests_1 {
    use super::*;   // import everything in the parent module

    #[test]         // test
    fn adds_2_and_2() {
        assert!(add(2, 2) == 4);
        assert_eq!(add(2, 2), 4);   // alternative
    }

    #[test]
    #[should_panic]
    // test passes if the function panics for any reason
    fn panics_when_called() {
        panics();
    }

    #[test]
    #[should_panic(expected = "panic message")]
    // test passes if the function panics with the failure message containing the 'expected' text
    fn panics_when_called_specific_message() {
        panics();
    }
}
```

#### Exercise (big numbers)

- simulate a 128 bit unsigned integer with 2 64 bit values:
- define a struct **BigNumber** with the following fields:
  - low: u64
  - high: u64
- the 128 bit number consists of the 2 fields, concatenated (**high**, then **low**)
- define a function **add** that receives 2 **BigNumber** parameters and returns a third one, which is the sum of the other 2
- write a set of tests to verify the behavior of the written function for various inputs

### Conversions

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

#### Conversions that can fail

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

##### Specialized `ToString` conversion

For the common case of converting a type to `String`, there exists the trait [ToString](https://doc.rust-lang.org/std/string/trait.ToString.html).

As in the examples above, it is preferable to implement a known trait, than to have custom conversion functions.

```rust
use std::string::ToString;

impl ToString for Cents {
    fn to_string(&self) -> String {
        format!("{} cents", self.0)
    }
}

let out = Cents(45).to_string();

println!("the value is: {}", out);
```
