# Threads

## Threads and Thread Safety

### Creating a new thread with spawn

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

### Sending data between threads

```rust
// the closure for a thread can also capture values
let exclusive = "this_text".to_string();

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

### Sharing data between threads

We cannot share unsynchronized data (see `Sync` explanation below)

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
    let counter = Arc::new(Mutex::new(0)); // creates an integer, wrapped in a mutex, wrapped in a reference-counted shared pointer
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter); // gets a new handle to the shared mutex
        let handle = thread::spawn(move || {
            let mut guard = counter.lock().unwrap(); // locks the mutex

            // update data inside the mutex
            // this can only be done after lock, so it is always safe
            *guard += 1;

            // mutex guard goes out of scope, and the mutex is unlocked
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
```

## The `Send` and `Sync` marker traits

`Send` basic idea:

- when an object is created in a function, it only exists on one thread
  - we can say that thread "owns" the object
- if the type is safe to move to another thread, the compiler marks it with `Send`
  - e.g. `let x = 5;` or `let stuff = vec![1, 2, 3];` are both safe to `Send`
- what is not `Send`:
  - objects with a reference inside (e.g. `Option<&String>` is not send, since the other thread may outlive the reference)
  - e.g. `&mut Vec<T>`, a mutable ref to Vec cannot be `Send`, because 2 threads would be able to mutate it at the same time

`Sync` basic idea:

- a `Vec` is not threadsafe:
  - i.e. it is _not_ safe to `push` or `pop` from 2 threads
  - but it is ok to have many shared references, readonly iteration is safe
- the compiler infers that `Vec` is not `Sync` (i.e. `Vec<T>: !Sync`)
  - the compiler does not allow mutable access from more than one thread
- Some basic types are `Sync` and therefore safe
  - a `std::sync::Mutex<T>` is `Sync`
  - a `std::sync::Arc<T>` is `Sync`
  - a `std::sync::atomic::AtomicBool` is `Sync`
    - but plain `bool` is not


## Exercise: multithreaded server using `serde_json`

_Note_: this is intended to be simple and short, not optimal<br/>
_Note_: the structure of this program is up to you. Feel free to use functions, traits, `Result`s, or anything else

### Part 1

The first goal is to write a network server, that:
- accepts TCP connections
- receives a single-line json representing 2 (x,y) points
- computes the midpoint (average)
- responds with the midpoint, encoded as json
- make it so that **no thread crashes**, on any error (i.e. no uses of `unwrap` or `expect`)

Tasks:
1. create a new project, with serde_derive and serde_json as dependencies
1. define (de)serializable structures (e.g. for Point)
    1. check that they work as expected
1. create a thread (or use the main one) that opens a listening TCP socket on localhost, and accepts new connections
1. for each accepted connection, spawn a new (worker) thread
1. on each worker thread, loop
    1. read a line of input from the socket
    1. deserialize the json request data
    1. compute the midpoint
    1. serialize the data
    1. send it back over the socket

Hints:
- you can test this manually with `telnet` (`sudo apt install telnet`)
- the standard lib has a `TCPListener` type
- in order to read a line (i.e. until the '\n' character) use `std::io::BufReader` and `fn read_until`
- to write, try `fn write_all`
- to use succinct error handling, it may be worth it to place code in a function that returns `Result`, and use `?` inside the function
- if it helps, you can also use tests

### Part 2

Adding basic shared state:
- a Mutex, shared between the threads
- the Mutex wraps a structure with fields that count the number of successful and error requests
- set up a way to request or print the statistics while the server is running

Tasks (using the project from above):
1. create a `Stats` struct with fields for `successful` and `failed` to count the requests
1. early in the program, create a `Mutex<Stats>`
1. wrap the mutex in an `Arc<T>` (threadsafe shared pointer), and hand out references to every worker thread
1. on each worker thread:
    1. when a request succeeds or fails, lock the mutex
    1. increment the corresponding field
    1. remember, that successfully receiving 0 bytes means the connection was closed, and is not an error condition
1. choose and implement a way to print statistics: possible ideas:
    - have a different thread that sleeps for 1 second, locks the mutex, writes the stats to stdout, and repeats
    - introduce a new json request to let clients request the statistics instead of computing a midpoint

Hints:
- if you haven't done so yet, it may be good to factor the parsing and responding logic in a function that returns a `Result` or `Option`. It will be easier to check if the result is success or error to increment the stats
- a simple way to handle several kinds of json requests is to define an (de)serializable `enum`, with each variant holding the request`payload`
