## Day 2 - Usage patterns & Real life applications

### Conversions

- Sometimes, we have an object of A, and we need to turn it into an object of type B.
- Conversion is expressed by implementing `From` for B (or, rarely, `Into`)
- After we implement `From`, we call the conversion code by calling `B::from(a)`

If we implement `From`, then `Into` is also automatically implemented.

Docs:

- From: <https://doc.rust-lang.org/core/convert/trait.From.html>
- TryFrom: <https://doc.rust-lang.org/core/convert/trait.TryFrom.html>

```rust
#[derive(Debug)]
struct Label {
    text: String,
    point: Point,
}
#[derive(Debug, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl From<Point> for Label {
    fn from(p: Point) -> Self {
        // return an object of our Self type (aka. Label), constructed from p
        Self {
            text: format!("(x={}, y={})", p.x, p.y),
            point: p,
        }
    }
}

let p = Point { x: 3, y: 4 };

let label_1 = Label::from(p.clone()); // this now works !
println!("converted: {:?}", label_1);

let label_2: Label = p.clone().into(); // this works automatically
println!("also converted: {:?}", label_2);
```


```rust
// some implementations of `From` and `Into` are predefined
let s1: &str = "text";
let s2 = String::from(s1); // call impl From<str> for String directly
let s3: String = s1.into(); // convert s1 into into whatever is needed

let large_val = 12345;
println!("conversion result: {:?}", u8::try_from(large_val));
```

#### Conversions that can fail

`From` is for conversions that always work. If our conversion can fail we use `TryFrom`.

```rust
use std::convert::TryFrom;

struct V4Prefix {
    value: u32,
}

#[derive(Debug)]
struct V4Netmask {
    value: u32,
}

impl TryFrom<V4Prefix> for V4Netmask {
    type Error = String; // we must define the type returned on error

    fn try_from(prefix: V4Prefix) -> Result<Self, Self::Error> {
        match prefix.value {
            0..=32 => {
                let little_endian: u32 = !((1 << (32 - prefix.value)) - 1);
                let value = little_endian.swap_bytes();
                Ok(Self { value })
            }
            // here, we can defend against invalid prefixes
            x => Err(format!("{} is too large for an ipv4 netmask", x)),
        }
    }
}

println!("good: {:?}", V4Netmask::try_from(V4Prefix { value: 5 }));
println!("bad: {:?}", V4Netmask::try_from(V4Prefix { value: 52438 }));
```

### Passing Functions and Closures

Functions and closures can be passed around as arguments:

```rust
// a function can take an argument, that is another function !
// (note: we cannot name the type of a function, so we use trait bounds)
//
// available traits are Fn, FnMut, and FnOnce
fn convert_and_call<F>(n: u64, process: F)
where
    F: Fn(&str) -> usize
{
    let s = n.to_string();
    let res = process(&s);
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
        s.len() * x
    }); // ...or be a full block of code
}
```

### Combinators and Transforms

[`Option::map`](https://doc.rust-lang.org/std/option/enum.Option.html#method.map) :

```rust
let x: Option<u32> = Some(3);
// let's change the value 'inside' the Option

// basic version
let x2 = match x {
    Some(val) => Some(val + 1),
    _ => None,
};

// equivalent
let x3 = x.map(|val| val + 1);
```

[`Option::and_then`](https://doc.rust-lang.org/std/option/enum.Option.html#method.and_then) :

```rust
fn triple_if_even(n: u32) -> Option<u32> {
    if n % 2 == 0 {
        Some(n * 3)
    } else {
        None
    }
}

let x: Option<u32> = Some(3);
// Task: pass the values inside x thru triple_if_even

// x.map(triple_if_even) would result in Option<Option<u32>>

// basic version
let x2 = match x {
    Some(val) => triple_if_even(val),
    _ => None,
};

// equivalent
let combined = x.and_then(|val| triple_if_even(val));
// equivalent 2
let combined2 = x.and_then(triple_if_even);
```

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

Frequently used methods:

- [`map` : transforms using the given closure](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.map)
- [`filter` : discard elements using closure](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.filter)
- [`find` : get element if it exists](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.find)
- [`collect`: gather elements into Vec/HashMap/String/etc](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.collect)

### Threads and Thread Safety

#### Creating a new thread with spawn

```rust
use std::thread;
use std::time::Duration;

let join_handle = thread::spawn(|| {
    for i in 1..10 {
        println!("hi number {} from the spawned thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
    123
});

// optional, block waiting for thread to finish
match join_handle.join() {
    Ok(ret) => println!("thread returned {}", ret),
    Err(e) => println!("thread panicked: {:?}", e),
}
```

#### Sending data between threads

```rust
// the closure for a thread can also capture values
let exclusive = 4;

let handle_a = thread::spawn(move || {
    println!("took ownership of {}", exclusive);
});

//println!("{}", exclusive); // errors, since `exclusive` was moved
```

Threads can pass messages using channels:

```rust
use std::sync::mpsc;

let (tx, rx) = mpsc::channel();

let handle_a = thread::spawn(move || { // this captures the tx end
    for i in 1..10 {
        println!("sending {}", i);
        tx.send(i).unwrap();
    }
}
let handle_b = thread::spawn(move || { // this captures the rx end
    for val in rx {
        println!("received {}", val);
    }
}

handle_a.join().unwrap();
handle_b.join().unwrap();
```

#### Sharing data between threads

We cannot share unsyncronized data (see `Sync` explanation below)

```rust
use std::thread;

let mut v = vec![1, 2, 3];

// THIS WILL NOT COMPILE
let handle_a = thread::spawn(|| {
    v.push(4);
});
let handle_b = thread::spawn(|| {
    v.push(4);
});
```

But if we create a thread-safe object (like a mutex), we can share it:

```rust
use std::sync::{Mutex, Arc};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0)); // creates an integer, wrapped in a mutex, wrapped in a shareable Arc
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter); // gets a new handle to the shared mutex
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap(); // locks the mutex

            // update data inside the mutex
            // this can only be done after lock, so it is always safe
            *num += 1;

            // mutex guard goes out of scope, and the mutex is closed
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
```

```rust
thread::spawn(move || {
    let listener = TcpListener::bind("127.0.0.1:1234").unwrap();

    loop {
        let conn = listener.accept();

        thread::spawn(move || {
            let (mut stream, remote) = match conn {
                Ok(x) => x,
                _ => return,
            };
            println!("new client connected from {}", remote);

            let mut buf = vec![];

            loop {
                buf.clear();
                let len = std::io::BufReader::new(&mut stream).read_until(b'\n', &mut buf);
                if let Ok(0) = len {
                    // connection closed. goodnight.
                    return;
                }
                if let Err(e) = len {
                    eprintln!("error receiving from client {}: {:?}", remote, e);
                    return;
                }

                let num: i64 = buf
                    .as_slice()
                    .map(|bytes| str::from_utf8(bytes))
                    .and_then(|s| s.parse().ok());

                let reply = num.map(|x| x + 1);

                match reply {
                    Some(num) => stream.write_all(result.as_bytes()).unwrap(),
                    None => stream.write_all("request not understood".as_bytes()),
                }
                stream.write_all(b"\n").unwrap();
            }
        });
    }
});

```

#### The `Send` and `Sync` marker traits

`Send` basic idea:

- when an object is created in a function, it only exists on one thread
  - we can say that thread "owns" the object
- if the type is safe to move to another thread, the compiler marks it with `Send`
  - e.g. `let x = 5;` or `let stuff = vec![1, 2, 3];` are both safe to `Send`
- what is not `Send`:
  - objects with a reference inside
  - e.g. `&mut Vec<T>`, a mutable ref to Vec cannot be `Send`, because 2 threads would be able to mutate it at the same time

`Sync` basic idea:

- a `Vec` is not threadsafe:
  - i.e. it is _not_ safe to mutate it from 2 threads
  - so it is not safe to have 2 `&mut` references to it
  - (but it is ok to have many shared references, readonly access is safe)
- the compiler infers that `Vec` is not `Sync` (i.e. `Vec<T>: !Sync`)
  - the compiler does not allow mutable access from more than one thread
- Some basic types are `Sync` and therefore safe
  - a `std::sync::Mutex<T>` is `Sync`
  - a `std::sync::Arc<T>` is `Sync`
  - a `std::sync::atomic::AtomicBool` is `Sync`
    - but plain `bool` is not

### Serde

- the [`serde_json`](https://crates.io/crates/serde_json) crate has support for (de)serialization to/from JSON
- it can convert rust structs to JSON objects, rust `Vec`s to JSON arrays, etc.
- it works by using `derive` annotations

```toml
# in Cargo.toml
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


## Maybe

### Using `io::Read` and `io::Write` on `slice`
