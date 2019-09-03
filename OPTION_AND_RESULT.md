# Option and Result

- **Option** and **Result** are very common across most APIs
- there are many common and repetitive operations done with `Option`, `Result`, etc.
- these types provide small functions, to turn several lines of repeated code into a single call

### [**Option::map**](https://doc.rust-lang.org/std/option/enum.Option.html#method.map)

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

### [**Option::and_then**](https://doc.rust-lang.org/std/option/enum.Option.html#method.and_then)

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

### [**Option::unwrap_or_else**](https://doc.rust-lang.org/std/option/enum.Option.html#method.unwrap_or_else)

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

### [**Option::as_ref**](https://doc.rust-lang.org/std/option/enum.Option.html#method.as_ref)

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

### Other useful combinators on **Option** and **Result**:

- **Result::map**, and **and_then** : similar to the Option methods
- **Result::ok** : transform a **Result<T, E>** into an **Option\<T>** (keep the value, throw away the error)
- **Option::ok_or**, and **ok_or_else** : transform an Option to Result, optionally setting an error

