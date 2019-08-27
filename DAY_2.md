## Day 2 - Usage patterns & Real life applications

### Passing functions and closures

- Functions and closures can be passed around as arguments
- A function can take an argument, that is another function
- To name the type of the arguments, we usually use generics and trait bounds
    - Common traits are **Fn**, **FnMut**, **FnOnce**
- Closures (aka lambda functions) are defined with:
    - list of arguments between `|` characters
    - body is an expression, and can also be a block

```rust
fn convert_and_call<F>(n: u64, process: F)
where
    F: Fn(&str) -> usize
{
    let text = n.to_string();
    let res = process(&text);
    println!("processing function returned {}", res);
}

fn do_actual_work(s: &str) -> usize {
    println!("pretending to do some work with {}", s);
    s.len()
}

fn main() {
    convert_and_call(1111, do_actual_work);

    convert_and_call(2222, |s| 3); // closures can return a value
    convert_and_call(3333, |s| s.len() * 22); // ...or compute things

    let x = 5;
    convert_and_call(4444, |s| {
        println!("closures can capture their environment");
        s.len() * x // this is the returned value
    }); // ...or be a full block of code
}
```

### Combinators and transforms

- there are many common and repetitive operations done with `Option`, `Result`, etc.
- these types provide small functions, to turn several lines of repeated code into a single call

[**Option::map**](https://doc.rust-lang.org/std/option/enum.Option.html#method.map) :

```rust
let x: Option<u32> = Some(3);
// Task: let's increment the value 'inside' the Option

// basic version
let x2 = match x {
    Some(val) => Some(val + 1),
    _ => None,
};

// equivalent, shorter
let x3 = x.map(|val| val + 1);
```

[**Option::and_then**](https://doc.rust-lang.org/std/option/enum.Option.html#method.and_then) :

```rust
fn correct_half(n: u32) -> Option<u32> {
    if n % 2 == 0 {
        Some(n / 2)
    } else {
        None // return nothing instead of rounding values
    }
}

let x: Option<u32> = Some(3);
// Task: pass the values inside x thru correct_half

// x.map(correct_half) would result in Option<Option<u32>>

// basic version
let x2 = match x {
    Some(val) => correct_half(val),
    _ => None,
};

// equivalent, shorter
let combined = x.and_then(|val| correct_half(val));
// equivalent, shorter (2)
let combined2 = x.and_then(correct_half);
```

[**Option::unwrap_or_else**](https://doc.rust-lang.org/std/option/enum.Option.html#method.unwrap_or_else) :

```rust
fn get_answer_from_server() -> u32 {
    56 // just pretend this is complicated
}

let x = Some(3);
// Task: get the 3, otherwise, get the answer from the server

// basic version
let v = match x {
    Some(v) => v,
    None => get_answer_from_server(),
};

// equivalent, shorter
let v2 = x.unwrap_or_else(|| get_answer_from_server());
```

[**Option::as_ref**](https://doc.rust-lang.org/std/option/enum.Option.html#method.as_ref) :

This example uses **as_mut**, which is the mutable counterpart of **as_ref**

```rust
let mut x = Some(3);
// Task: increment the value inside x, but mutating it, not moving as above

// basic version
match &mut x {
    Some(v) => *v += 1,
    _ => {},
}

// equivalent, shorter
x.as_mut().map(|v| *v += 1);

println!("{:?}", x);
```

Other useful combinators on **Option** and **Result**:

- **Result::map**, and **and_then** : similar to the Option methods
- **Result::ok** : transform a **Result<T, E>** into an **Option<T>** (keep the value, throw away the error)
- **Option::ok_or**, and **ok_or_else** : transform an Option to Result, optionally setting an error


### Iterators

The `Iterator` trait :

```rust
pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;

    // ... other methods
}
```

Basic idea:

- `next()` returns the next element and advances iteration
- when `next()` returns `None` iteration has finished
- `Item` is an associated type: the implementation specifies the type of the element

Basic manual usage:

```rust
let v = vec![1, 2, 3];
let mut it = v.iter(); // it is mutable to advance, but v is not mutable

println!("{:?}", it.next());
println!("{:?}", it.next());
println!("{:?}", it.next());
println!("{:?}", it.next()); // this prints None
```

Frequently used methods:

- [**map** : transforms using the given closure](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.map)
- [**filter** : discard elements using closure](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.filter)
- [**find** : get element if it exists](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.find)
- [**collect**: gather elements into Vec/HashMap/String/etc](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.collect)

#### Iterator examples

```rust
let v: Vec<u32> = vec![1, 2, 3];

// frequently, collect()'ing from iterators requires
// annotating the type of the destination collection
// the compiler often suggest the annotation
let squared: Vec<u32> = v.iter()
    .map(|x| x * x)
    .collect();
```

Usual methods to start iteration:

- `iter()` will iterate with references to the original elements
- `into_iter()` will consume the Vec, and iterate the actual elements

```rust
// this leaves v still alive, and iterates with references
let even: Vec<u32> = v.iter()
    .filter(|x| *x % 2 == 0) // dereference x, because x is a reference
    .map(|x| *x) // make a copy of x, to create a vector of value
    .collect();

// this consumes v, and iterates with values
let halves: Vec<u32> = v.into_iter()
    .map(|x| x / 2)
    .collect();
```

A complicated example:
- Maps and sets (like HashMap) can be used with iterators
- we can **collect()** an iterator of 2-element tuples into a **HashMap**
- **filter_map** combines selection and processing:
    - if the closure returns None, it skips the element
    - it the closure returns Some(new_element), then it accepts new_element
    - "?" error propagation works on **Option**, and inside closures

```rust
use std::collections::HashMap;

let string_pairs = vec!["A=4", "B=X", "C=20", "QWE"];

// filter_map combines filter and map in one step
let actual_map: HashMap<String, u32> = string_pairs.iter()
    .filter_map(|s| {
        let mut it = s.split("=");
        let key = it.next()?; // "?" also works on Option, and can return from closures
        let val_str = it.next()?;
        let val_int = val_str.parse().ok()?;

        Some((String::from(key), val_int))
    })
    .collect();

println!("{:?}", actual_map);
```


### External libraries

- in rust, libraries and projects are called **crates**
- we usually want to use available crates instead of reinventing wheels
- **cargo** makes it all very simple:
    - every package has a **Cargo.toml** file (the toml format is like a combination of ini and json)
    - we can add **lib_name = "version_number"** keys to the **[dependencies]** section
    - at build, cargo downloads and compiles the library
- we can use thing
- (advanced: also see **MODULES.md** for how **use** works)

#### Where to find crates

- [**crates.io**](https://crates.io/) : official package index
- [**libs.rs**](https://libs.rs/) : alternative package search tool for information from crates.io

Usually, every package published on crates.io will have automatically generated docs at **docs.rs/package_name**

#### Example crate usage

Task:
- extract words from some text
- capitalize all words shorter than 5 letters, and discard the rest
- collect all words in a vector

We can use **heck**:
- crate page: <https://crates.io/crates/heck>
- api documentation: <https://docs.rs/heck/0.3.1/heck/>

Add in **Cargo.toml**:

```toml
[dependencies]
heck = "0.3.1"
```

```rust
use heck::CamelCase;

fn main() {
    let s = "abc def akjdgnakdjsg";
    let v: Vec<_> = s
        .split_whitespace()
        .filter(|s| s.len() < 4)
        .map(|word| word.to_camel_case())
        .collect();
        
    println!("{:?}", v);
}
```


### Serde

- the [**serde_json**](https://crates.io/crates/serde_json) crate has support for (de)serialization to/from JSON
- it can convert rust structs to JSON objects, rust `Vec`s to JSON arrays, etc.
- it works by using `derive` annotations

Add in **Cargo.toml**:

```toml
[dependencies]
serde_json = "1.0"
```

```rust
use serde_json::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)] // this is where the magic happens
struct TheData {
    x: u32,
    s: String,
    v: Vec<f32>,
}

let d = TheData { x: 5, s: "abcd".into(), v: vec![1,2,3] };

let s = serde_json::to_string_pretty(&d).expect("serialization failed")
println!("{}", s);

let recovered: TheData = match serde_json::from_str(&s) {
    Ok(data) => data,
    Err(e) => {
        println!("deserialization error: {:?}", e);
        return;
    }
};
```

