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

## The `Send` and `Sync` marker traits

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


## TODO: server example

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
