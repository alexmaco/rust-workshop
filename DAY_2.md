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

#### Exercise: transforming with functions

Part 1:

- define a generic function `apply_transform` that takes 2 arguments:
    - a **Vec\<T>**
    - any function **Fn(T) -> U**
- `apply_transform` should return a **Vec\<U>**
- inside `apply_transform`, for each element **T** in the input vector
    - call the provided closure to construct a new **U**
    - append the new value to the returned vector
- call the function to process a Vec with a simple closure

Part 2: try to make a few changes:

- only pass a reference to the input **Vec\<T>**, and only reference its elements (you will need to change the closure signature)
- when creating the closure, try to capture things from the enclosing scope
    - try to use immutable references to outer objects
    - try to use mutable references to outer objects
    - try to move things into and out of the closure
    - observe the errors


### Iterators

The `Iterator` trait:

```rust
// This is already defined in the standard library
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
let mut it = v.iter(); // it is mutable to be able to advance, but v is not mutable

println!("{:?}", it.next());
println!("{:?}", it.next());
println!("{:?}", it.next());
println!("{:?}", it.next()); // this prints None
```

Frequently used methods:

- [**map** : transforms using the given closure](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.map)
- [**filter** : discard elements using closure](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.filter)
- [**find** : get element if it exists](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.find)
- [**enumerate** : also yields the index when interating](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.enumerate)
- [**collect**: gather elements into Vec/HashMap/String/etc](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.collect)

#### Iterator examples

```rust
let v: Vec<u32> = vec![1, 2, 3];

// frequently, collect()'ing from iterators requires
// annotating the type of the destination collection
// the compiler often suggests the annotation
let squared: Vec<u32> = v.iter()
    .map(|x| x * x)
    .collect();
```

In the example above:
- `iter()` creates an iterator (but it does not advance, noone has called `next()`)
- `map()` creates a new iterator, that is an adapter over the old iterator
  - every time `next` is called on this new iterator, it:
    - calls the `next` of the underlying iterator
    - applies the function
    - then yields the value returned by the function
- `collect()` does not create an iterator. It calls `next` on the iterator it receives, and adds all the elements it yields into the final collection

Practical consequences:
- iterators are "lazy": we can construct complicated iterations, and use them later
- in order to perform an iteration on the spot, we usually use `for`, or call `.collect()`

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

#### Example: parsing "key=value" strings

- Maps and sets (like HashMap) can be used with iterators
- we can **collect()** an iterator of 2-element tuples into a **HashMap**
- **filter_map** combines selection and processing:
    - if the closure returns None, it skips the element
    - it the closure returns Some(new_element), then it accepts new_element
    - "?" error propagation works on **Option**, and inside closures

Example task:
- we have some strings in the "key_name=integer_value" format
- we want to parse them in a HashMap<String, u32>
- we want to skip strings not in the correct format, but not crash

First version:

```rust
// Version 1
use std::collections::HashMap;

let string_pairs = vec!["A=4", "B=X", "C=20", "QWE"];

let mut actual_map: HashMap<String, u32> = HashMap::new();

for pair in string_pairs.iter() {
    let mut pieces = pair.split('=');
    let maybe_key = pieces.next();

    let key = match maybe_key {
        Some(k) => k,
        _ => continue, // this branch does not return, instead continues the loop
    };

    let maybe_val = pieces.next();

    let val = match maybe_val {
        Some(v) => v,
        _ => continue,
    };

    let val_num: u32 = match val.parse() {
        Ok(x) => x,
        Err(_) => continue, // we ignore parsing errors
    };

    actual_map.insert(key.into(), val_num);
}

println!("{:?}", actual_map);
```

We can simplify: we can separate a parsing function, so we can use the "?" error propagation operator on **Option**.  
**"?"** works on **Option** just as it does with **Result**:

- if the option is **Some(val)**, it gets the contained val
- if it is **None**, it returns **None** from the function

```rust
// Version 2
use std::collections::HashMap;

fn parse_pair(pair: &str) -> Option<(String, u32)> {
    let mut pieces = pair.split('=');

    let key = pieces.next()?;
    let val = pieces.next()?;

    let val_num: u32 = val.parse().ok()?; // .ok() on a Result will yield an Option<T>, and discard the error if any

    Some((String::from(key), val_num))
}

let string_pairs = vec!["A=4", "B=X", "C=20", "QWE"];

let mut actual_map: HashMap<String, u32> = HashMap::new();

for pair in string_pairs.iter() {
    let maybe_pair = parse_pair(pair);

    let (k, v) = match maybe_pair {
        Some(tup) => tup,
        _ => continue,
    };

    actual_map.insert(k, v);
}

println!("{:?}", actual_map);
```

One more simplification: we can iterate over the vector, generate pairs, and collect into a HashMap

```rust
// Version 3
use std::collections::HashMap;

fn parse_pair(pair: &str) -> Option<(String, u32)> {
    let mut pieces = pair.split('=');

    let key = pieces.next()?;
    let val = pieces.next()?;

    let val_num: u32 = val.parse().ok()?;

    Some((String::from(key), val_num))
}

let string_pairs = vec!["A=4", "B=X", "C=20", "QWE"];

let actual_map: HashMap<String, u32> = string_pairs.iter()
    .filter_map(|pair| parse_pair) // now the iterator's elements are (String, u32)
    .collect(); // we can collect an iterator with elements (K, V) into a HashMap

println!("{:?}", actual_map);
```

Most compact version:

```rust
// Version 4
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

#### Exercise: iterators

Given the resulting map from the exercise above, reverse the process
- create a `HashMap<String, u32>`
- fill it with some values
- iterate the map
- apply some transformations as above to create strings of the form "key=value"
- concatenate all created strings, with a `\n` in-between

Hint: if you have an iterator over **String** or **&str** items, you can use **collect()** to concatenate it all into a single string


### External libraries

- in rust, libraries and projects are called **crates**
- we usually want to use available crates instead of reinventing wheels
- **cargo** makes it all very simple:
    - every package has a **Cargo.toml** file (the toml format is like a combination of ini and json)
    - we can add **lib_name = "version_number"** keys to the **[dependencies]** section
    - at build, cargo downloads and compiles the library
- we can use things from the libraries as `crate_name::stuff_from_the_crate`
- (advanced: also see **MODULES.md** for how **use** works)

#### Where to find crates

- [**crates.io**](https://crates.io/) : official package index
- [**lib.rs**](https://lib.rs/) : alternative package search tool (same crates as above)

Usually, every package published on crates.io will have automatically generated docs at **https://docs.rs/package_name**

#### Example crate usage

Task:
- extract words from text
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
use heck::CamelCase; // this imports the CamelCase trait from the heck crate

fn main() {
    let s = "abc def akjdgnakdjsg";
    let v: Vec<_> = s
        .split_whitespace()
        .filter(|s| s.len() < 5)
        .map(|word| word.to_camel_case())
        .collect();
        
    println!("{:?}", v);
}
```

### JSON with the Serde crates

- the [**serde_json**](https://crates.io/crates/serde_json) crate has support for (de)serialization to/from JSON
- it can convert rust structs to JSON objects, rust `Vec`s to JSON arrays, etc.
- it works by using `derive` annotations

Add in **Cargo.toml**:

```toml
[dependencies]
serde = "1.0" # the serde core crate
serde_derive = "1.0" # this is needed for Serialize and Deserialize below
serde_json = "1.0"
```

```rust
use serde_derive::{Serialize, Deserialize};
use serde_json;

#[derive(Serialize, Deserialize)] // this is where the magic happens
struct TheData {
    x: u32,
    s: String,
    v: Vec<i32>,
}

fn main() {
    let d = TheData { x: 5, s: "abcd".into(), v: vec![1,2,3] };

    let s = serde_json::to_string_pretty(&d).expect("serialization failed");
    println!("{}", s);

    let recovered: TheData = match serde_json::from_str(&s) {
        Ok(data) => data,
        Err(e) => {
            println!("deserialization error: {:?}", e);
            return;
        }
    };
}
```

#### Manipulating JSON values

- the `serde_json` crate defines the `Value` enum
    - it represents a JSON object
    - `Value` is a regular enum, nothing special
    - has many convenience methods
- it is useful for exploring or creating a json value programmatically
- see the <https://docs.rs> page !

```rust
use serde_json::Value;

fn main() {
    // this is the syntax for raw strings
    // raw strings can be multiline, and can contain quotes
    let text = r#"{
        "a_key": {
            "subkey_1": true,
            "subkey_2": [15, "description"]
        }
    }"#;

    // we can parse a value
    let j: Value = serde_json::from_str(text).unwrap();

    // we can explore with match
    match j {
        // :#? is also debug-printing, except the text is indented and not single-line
        Value::Object(o) => println!("json root is an object {:#?}", o),
        Value::Array(vec) => println!("json root is an array with {} elements", vec.len()),
        _ => println!("json is something else"),
    }

    // we can construct values
    let leaf = Value::Bool(false);
    let num_leaf = Value::from(3);

    let array_val = Value::Array(vec![leaf, num_leaf]);

    // and serialize
    println!("{}", serde_json::to_string(&array_val).unwrap());
}
```

## Applications

### CSV to HTML

- use the code skeleton in the `convert` directory
- it contains a simple HTML implementation
- we want to parse a csv document (using the `csv` crate), and print an HTML document containing a table with the csv data
- instructions are present as comment blocks in `convert/src/main.rs`

### HTML to JSON

- start with the code from the solved exercise above
- instead of printing the HTML at the end, write a function that walks the HTML nodes, and constructs a JSON value
    - add `serde_json` as a dependency
    - use `serde_json::Value` to represent the constructed JSON
    - output the JSON text
