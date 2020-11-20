# Day 1 - Basics

Examples adapted or taken from the [rust book](https://doc.rust-lang.org/book/ch00-00-introduction.html)

## Online Playground

https://play.rust-lang.org/

## Documentation (reference guides)

- the standard library: https://doc.rust-lang.org/std/index.html
- search for crates: https://crates.io/
- read documentation for a specific crate: https://docs.rs/
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

### Basics and Syntax overview

- a function named **main** must exist
- when a program is run, the code in **main** is executed
- must be written in **main**
  - variable declarations
  - function calls
  - other statements and expressions
- should be written outside of **main**
  - structure declarations
  - enum declarations
  - function definitions
- code blocks
  - contained between `{` and `}`
  - optionally, the body can end in an **expression** (returns a value)
  - consists of a series of **statements**, each ending in **semicolon** (does not return a value)
  - can end with an **expression** (returns a value)
    - `WARNING`: **expressions** do not include ending **semicolon**; if semicolon is added to the end of an expression, it is turned into a **statement** and it won't return any value
- comments:
  - line: `//`
  - block: `/* */`
- `!` after an identifier means it is a **macro**; must always include the `!` when called
  - **macros** are not covered here, the important thing is to think about them as functions
  - we will encounter the following macros:
    - `println!`
    - `format!`
    - `vec!`
    - `panic!`

### Variables and mutability

- by default, the variables are immutable (the value cannot be changed after the initialization)
- the compiler will raise an error if we try to change the value of an immutable variable
- variables can be declared mutable using **mut**

```rust
fn main() {
    let /*mut*/ x = 5;
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

    let number_convert/*: u32*/ = "42".parse().expect("Not a number!"); // incorrect: type must be specified explicitely

    // display
    println!("number is {}, number_auto is {}", number, number_auto);
}
```

#### Floating point (f32, f64)

```rust
fn main() {
    let x = 2.0;        // f64
    let y: f32 = 3.0;   // f32
    let z: f32 = 5;     // error; conversion from int to float is not made implicitly
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

    // invalid access
    let error = tup.3;      // does not compile
}
```

#### Array [..., ...]

- has fixed size

```rust
fn main() {
    let a: [i32; 5] = [1, 2, 3, 4/*, 5*/];  // element type and number defined explicitly
                                            // error: the number of elements declared must match the number of elements intialized
    let b = [1, 2, 3, 4, 5];            // element type and number inferred
    let c = [3; 5];                     // 3 repeated 5 times: [3, 3, 3, 3, 3]

    // element access
    let first = a[0];
    let second = a[1];

    // invalid element access
    let idx = 11;
    let element = a[idx];               // compiles fine; panics at runtime
    let element = a[10];                // does not compile; the index is known at compile time

    // the number of elements
    let length = a.len();

    // try to display the elements
    println!("a: {}", a);   // error

    // display using debug
    println!("a: {:?}", a);
}
```

#### Exercise 1 (average)

- following the above steps, create a new project
- in main:

  - declare 3 numbers
  - compute the **average** of them and store the result in a variable
    - average = sum of the values divided by the number of values
  - display the result with

    ```rust
    println!("Result: {}", result);
    ```

  - conversions from **int** to **float** must be done explicitly

    ```rust
    let int_value = 42;
    let float_value = int_value as f32;
    ```

### Functions

- may have a return type or not
- may have none, one or more parameters

#### Function without parameters, no return value

```rust
// definition
fn my_func() {
    println!("my_func");
}

fn main() {
    // call
    my_func();
}
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

fn main() {
    // call
    my_func1(/*42*/);       // error: all the expected arguments must be passed at call time
    my_func2(1/*, 2*/);     // error: all the expected arguments must be passed at call time
}
```

#### Function with parameters, with return value

If a function returns a value, the type of the return value must always be declared in the function's signature.

```rust
// definition
fn my_func(x: i32) -> bool {
    println!("my_func, x: {}", x);

    // explicit return (statement, must end with semicolon)
    /* return x == 1; */    // error: must return a bool value
}

fn my_func2(x: i32) /*-> bool*/ {   // error: the function must declare the return type if the body returns a value
    println!("my_func, x: {}", x);

    // implicit return (expression, no ending semicolon)
    x < 5
}

fn main() {
    // call
    let result = my_func(0);
}
```

#### Exercise 2 (average with functions)

- modify the code from Exercise 1 (average)
- extract the computing of average in a function
- call the function multiple times, with various sets of values

### Conditions (if expressions)

- **if** is an expression, thus returning a value
  - the returned value is the one returned from the taken branch
  - both branches must return values of the same type
  - if the branches don't return a value, the return is **()** called **unit type**
- the condition **must** be a **bool**
- it is not necessary to surround the condition with `()`

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
    for i in 0..x {     // does not include x: 0, 1, ..., x-1
        println!("Iteration {}", i);
    }
}
```

#### Exercise 3 (prime numbers)

- define an **array** variable that contains some **u32** values
- display for each number if it is prime or not (a number is prime if it is divisible only by 1 and itself)
  - for each number **n** in the array, iterate through all the numbers **i** between **2** and **n/2**
    - if `n % i == 0`, then **n** is not prime; continue with the next element
    - if after checking all the possible **i** for **n**, none satisfy the condition, **n** is prime

### Strings

- type: **String**
- a **string literal** is hardcoded into the program binary
  - it is immutable
  - must be known at compile time
- **String** allocates a dynamic buffer, at runtime
  - **owns** the data
  - deallocates the data when it goes out of scope

```rust
fn main() {
    let literal = "hello";              // literal string
    let s = String::from(literal);      // string object
    let mut s: String = "hello";        // error: String must be constructed explicitly from a literal

    s.push_str(" world!");              // append string

    println!("{}", s);

    let s1 = s.clone();                 // duplicate the string data
}
```

### Ownership

- ownership enables Rust to make memory safety guarantees without needing a garbage collector
- none of the ownership features slow down the program while running
- ownership rules
  - each value in Rust has a variable that’s called its owner
  - there can only be one owner at a time
  - when the owner goes out of scope, the value will be dropped
- the scope of a variable starts with the declaration and ends with the closing curly brace `}`
- the simple types are copied instead of moved (integer, float, boolean etc)

```rust
let s1 = String::from("hello");
let s2 = s1;    // s1 is "moved" into s2; s1 is not valid any more

println!("{}, world!", s1);     // error: value used after move
```

```rust
let s1 = String::from("hello");
let s2 = s1.clone();                    // s2 contains a deep copy of s1

println!("s1 = {}, s2 = {}", s1, s2);
```

- passing as argument to a function moves the value into the function

```rust
fn main() {
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it’s okay to still
                                    // use x afterward

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.
```

- returning a value moves the ownership to the caller; so values can be moved into a function and then returned so the ownership is back to the calling code

```rust
fn main() {
    let s1 = gives_ownership();         // gives_ownership moves its return
                                        // value into s1

    let s2 = String::from("hello");     // s2 comes into scope

    let s3 = takes_and_gives_back(s2);  // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 goes out of scope but was
  // moved, so nothing happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {             // gives_ownership will move its
                                             // return value into the function
                                             // that calls it

    let some_string = String::from("hello"); // some_string comes into scope

    some_string                              // some_string is returned and
                                             // moves out to the calling
                                             // function
}

// takes_and_gives_back will take a String and return one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
                                                      // scope

    a_string  // a_string is returned and moves out to the calling function
}
```

### References and Borrowing

- sometimes a function does not need to get the ownership of a value if it only wants to do some processing based on it
- a reference to the value can be passed to the function, the owner remaining the calling code; this is called borrowing
- references are immutable by default: **&T**
- a reference can be declared mutable with **&mut T**
- restriction: there cannot be mutable and immutable references to a value at the same time; otherwise, code that has an immutable reference cannot assume that the value will never change
  - any number of immutable references can exist at any time
  - a single mutable reference can exist at any time
  - this is enforced by the compiler

```rust
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, nothing happens.
```

- mutable references

```rust
fn main() {
    let mut s = String::from("hello");

    change(&/*mut*/ s);     // error: mut must be specified when the reference is taken
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

- try to get both mutable and immutable references to a value

```rust
let mut s = String::from("hello");

let r1 = &s; // no problem
let r2 = &s; // no problem
let r3 = &mut s; // BIG PROBLEM

println!("{}, {}, and {}", r1, r2, r3);
```

- references live as long as they are used

```rust
let mut s = String::from("hello");

let r1 = &s; // no problem
let r2 = &s; // no problem
println!("{} and {}", r1, r2);
// r1 and r2 are no longer used after this point

let r3 = &mut s; // no problem
println!("{}", r3);
```

- dangling references are not allowed; the compiler ensures that the owner variable lives at least as long as all the references to it

```rust
fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    let s = String::from("hello");

    &s          // error: this reference would be dangling when s goes out of scope

    // s goes out of scope and its value is dropped
}
```

### The Slice Type

#### String slices

- a string slice is a reference to part of a **String**, and it looks like this

```rust
// string literals are slices; s has type &str
let s = "Hello, world!";

let s = String::from("hello world");
let len = s.len();

let hello = &s[0..5];
let world = &s[6..11];

let slice = &s[0..2];   // 0 can be omitted
let slice = &s[..2];

let slice = &s[3..len]; // len can be omitted
let slice = &s[3..];

let slice = &s[0..len]; // len can be omitted
let slice = &s[..];
```

- get the first word from a String

```rust
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {    // enumerate returns a tuple (index, element)
        if item == b' ' {   // ascii byte literal for space
            return &s[0..i];
        }
    }

    &s[..]
}
```

- it is better to declare the argument as &str, as we can pass both String and slices

```rust
fn first_word(s: &str) -> &str {
    ...
}

fn main() {
    let my_string = String::from("hello world");

    // first_word works on slices of `String`s
    let word = first_word(&my_string[..]);

    let my_string_literal = "hello world";

    // first_word works on slices of string literals
    let word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);
}
```

#### Other slices

- slices apply to arrays as well
- similar to the String slices

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];   // slice has type &[i32]
}
```

#### Exercise 4 (string manipulation)

- declare an array of **String** objects, each element representing a word, surrounded by an arbitrary number of spaces before and after
- find a function to trim the surrounding spaces from each word (search the [Documentation for String](https://doc.rust-lang.org/std/string/struct.String.html))
- obtain a new **String** by concatenating all the trimmed words, inserting a single space between them
- example:
  - [" hello ", " world "] => "hello world "

### Structs

- similar with tuples, but have names for the fields
- the whole structure must be declared **mutable** or not; there is no control per field
- similar to **struct** and **class** in other languages

```rust
// definition
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    // instantiation
    let mut user = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        /*sign_in_count: 1,*/   // error: all the fields must be initialized
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
    };

    // try to display the struct
    println!("user1: {}", user1);   // error

    // display using debug; must also add #[derive(Debug)] before struct User
    println!("user1: {:?}", user1);
}
```

#### Exercise 5 (rectangle area)

- define a structure to hold a **Rectangle**, with the following fields
  - width
  - height
- instantiate one **Rectangle**
- display the **Rectangle**
- write a function to compute the area of the **Rectangle**
  - `area = width * height`

#### Methods

- functions defined within the context of a structure
- the structure instance, **self**, can be passed as the first parameter
- similar to normal functions, except for **self**
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
}

fn main() {
    let d = Duration {
        seconds: 120,
        description: String::from("test"),
    };

    println!("{} seconds, {} minutes", d.seconds, d.minutes());
}
```

#### Exercise 6 (rectangle area with method)

- change the **area** function from **Exercise 4** to be a method on the struct **Rectangle**

### Enums

- enumerates the possible values for a type

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

    let home2 = IpAddrEnum::V4(/*String::from("127.0.0.1")*/);
}
```

#### The Option enum

- an enum provided in the standard library, widely used
- the way to express the absence of a value (equivalent to **null** in other languages)
- **None** means the value of type T is not present, cannot be used as a valid value

```rust
// this is alreay defined in the standard library
// T is replaced with any type we need when using the enum
//
// enum Option<T> {
//     Some(T),
//     None,
// }

fn positive(n: i32) -> Option<i32> {
    if n > 0 {
        Some(n)
    } else {
        None
    }
}

fn main() {
    let some_number = Some(5);
    let some_string = Some("a string");

    let no_number: Option<i32> = None;  // must specify the type, the compiler cannot infer it

    // Option<T> cannot be used directly as T
    // the None case must be explicitly handled when the value is actually used
    let sum = 42 + some_number;         // error

    // returned values
    let r1 = positive(-1);
    let r2 = positive(2);

    println!("result1: {:?}", r1);
    println!("result2: {:?}", r2);
}
```

### Pattern matching

- compare a value against possible patterns; the code corresponding to the first pattern that matches is executed
- the compiler checks that all the possible cases are handled
- useful for destructuring types
- returns the value from the branch that was taken
- all the branches must return values of the same type

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
        /*Coin::Quarter => 25,*/    // error: must handle all the possible enum values
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

fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}
```

#### if let

- more concise way to write a match where all branches but one are ignored
- useful when we want to do something only on one branch, the other being
  - `_ => ()`
- can optionally have an **else**

```rust
fn main() {
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
}
```

#### Exercise 7 (time)

- define an enum **Time** for various time units, each variant containing an integer value:
  - seconds
  - minutes
  - hours
- write a function that receives a **Time** value and computes the number of seconds corresponding to it
  - `seconds = minutes * 60`
  - `seconds = hours * 3600`

### Collections

#### Vector: Vec\<T>

- stores elements of the same type, contiguous in the memory
- if multiple types need to be stored in a vector, a vector of **enum** can be used (with inner types)

```rust
// create an empty vector
fn main() {
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
    for i in v3.iter() {
        println!("{}", i);
    }

    // iterate and modify each element in the vector
    let mut v4 = vec![100, 32, 57];
    for i in v4.iter_mut() {
        *i += 50;   // add 50 to each element
    }
}
```

#### Hash Map: HashMap\<K, V>

- stores a mapping of keys of type **K** to values of type **V**
- uses a hashing function to determine the place for a specific element
  - the order of the elements may appear random
- custom types can be used as keys; the **Hash** trait must be implemented (usually via #[derive(Hash)])
- similar to **map**, **dict** in other languages

```rust
use std::collections::HashMap;

fn main() {
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
}
```

#### Exercise 8 (number of occurrences)

- declare some **Vec** instances containing some integers
- obtain a larger **Vec** that contains the elements in all the smaller **Vec** instances (see [Documentation for Vec](https://doc.rust-lang.org/std/vec/struct.Vec.html))
- compute the number of occurrences for each element in the vector
  - initialize a **HashMap** to count the occurences (see [Documentation for HashMap](https://doc.rust-lang.org/std/collections/struct.HashMap.html))
  - for each **n** in the vector
    - if **n** is not in the **HashMap**, add a value of **1** corresponding to **n**
    - if **n** is in the **HashMap**, read the corresponding value, then increment it and store it back in the **HashMap**

### Error handling

- rust requires the programmer to acknowledge the existence of errors and treat all the cases where an error may occur; the code won't compile unless all the error cases are handled

#### Unrecoverable errors (panic!)

- when **panic!** is called, the program prints an error message, does some cleanup and quit
- panic can be called directly by our code or indirectly by other code that we call
- by default the stacktrace of the program is not displayed at panic; it can be enabled with:
  - `RUST_BACKTRACE=1 cargo run`
- similar to **assert** in other languages

```rust
fn main() {
    // direct call
    panic!("crash and burn");

    let v = vec![1, 2, 3];

    // indirect call to panic!
    v[99];
}
```

#### Recoverable errors (Result\<T, E>)

- the **Result** enum is defined in the standard library
- shortcuts for the case when the error case should cause panic
  - unwrap
  - expect (similar to unwrap, allows specifying an error message in case of failure)
- **unwrap** and **expect** should be used when prototyping or in test code
- errors can be propagated using `?`
  - if the result is **Ok**, the execution continues with the value contained inside
  - if the result is **Error**, the result is returned from the current function, for the calling code to process it
  - it must be called inside a function that returns **Result**

```rust
// this is already defined in the standard library
//
// enum Result<T, E> {
//     Ok(T),          // T is the type of the value returned in case of success
//     Err(E),         // E is the type of the value returned in case of failure
// }
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

- reduce duplication and increases reusability by parametrizing the type in the definitions of functions, structs, enums
- examples:
  - Option\<T>
  - Result\<T, E>
  - Vec\<T>
  - HashMap\<K, V>
- the compiler generates code for each instantiated type (similar to templates in C++)

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

- define functions to compute the largest element in an array of
  - i32
  - char

```rust
fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
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
- group method signatures together (similar to an **interface** in other languages)
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
- the default implementation cannot be called from the overridden one

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
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list.iter() {
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

- advanced: methods can be conditionally implemented based on the trait bound

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

#### Exercise 9 (shapes)

- define a trait **Shape** with the following methods:
  - **area** (compute the area of a shape)
  - **perimeter** (compute the perimeter of a shape)
- use: `use std::f32;`
- use **f32** for the type of the fields
- define a set of **struct** for shapes and implement **Shape** for each of them:
  - **struct Circle**
    - fields: **radius**
    - area: `f32::consts::PI * radius * radius`
    - perimeter: `2.0 * f32::consts::PI * radius`
  - **struct Rectangle**
    - fields: **width**, **length**
    - area: `width * length`
    - perimeter: `2.0 * (width + length)`
  - **struct Triangle**
    - fields: **a**, **b**, **c**
    - area: `(s * (s - a) * (s - b) * (s - c)).sqrt()`
      - where `s = (a + b + c) / 2.0)`
    - perimeter: `a + b + c`
- declare a **Vec\<Circle>**, a **Vec\<Rectangle>** and a **Vec\<Triangle>** and populate them with objects
- write a function that receives a **Vec** of elements that implement **Shape**, iterates over the elements in the vector and:
  - compute the total perimeter (sum of the perimeter of all the shapes)
  - total area (sum of the areas of all the shapes)

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
    // test passes if the function panics with the failure message containing the 'panic message' text
    fn panics_when_called_specific_message() {
        panics();
    }
}
```

#### Exercise 10 (factorial)

- define a function to compute the factorial of n, with argument n received as a signed value
  - if n < 0, return None
  - if n == 0, return Some(1)
  - if n >= 0, return Some(1 \* 2 \* 3 \* ... \* (n-1))
- write a set of tests to verify the behavior of the written function for various inputs
