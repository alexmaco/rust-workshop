Summary of [Appendinx C from the book](https://doc.rust-lang.org/book/appendix-03-derivable-traits.html)

These are the standard traits we can `#[derive(NameOfTheTrait)]`

| Trait | Requires | Useful for | Usage example |
| --- | --- | --- | --- |
| Clone | | deep-copying objects | `let v = vec![1,2]; foo(v.clone()); // v still in scope, only clone was passed` |
| Copy | Clone | automatic copying of objects | `let x = 3; foo(x); bar(x);` |
| Debug | | debug-printing objects | `println!("{:?}", thing);` |
| PartialEq | | comparing objects with `==` | `if my_obj_a == other_obj_a { ... ` |
| Eq | PartialEq | marks equality as transitive | required when using types in a `HashSet` or as keys in a `HashMap` |
| PartialOrd | | comparing objects for order | `if obj_x < obj_y { ...` |
| Ord | PartialOrd | marks ordering as transitive (total) | required when using types in a `BTreeSet` or as keys in a `BTreeMap` (ordered) |
| Hash | | getting the hash of an object | required when using types in a `HashSet` or as keys in a `HashMap` |
| Default | | getting a "default" state for an object | `let s = String::default(); // empty string` |

<table>
<thead><tr>

<td>

**Trait**

</td>
<td>

**Requires**

</td>
<td>

**Useful for**

</td>
<td>

**Usage example**

</td>

</tr></thead>
<tbody>

<tr>
<td>
Clone
</td>
<td>

</td>
<td>
deep-copying objects
</td>
<td>

```rust
#[derive(Clone)]
struct MyData {
    v: Vec<u32>,
    s: String,
}

let obj = MyData { v: vec![1,2,3], s: "abcd".into() };

// this now works; duplicate is a deep copy
let duplicate = obj.clone();
```

</td>
</tr>

<tr>
<td>
Copy
</td>
<td>
Clone
</td>
<td>
automatic bit-copying objects, instead of moving
</td>
<td>

```rust
#[derive(Copy, Clone)]
struct Point {
    x: f32,
    y: f32,
}

let p = Point { x: 0.3, y: 0.67 };

// this now works; p2 is a copy, p was not moved
let p2 = p;
```

</td>
</tr>

<tr>
<td>
Debug
</td>
<td>

</td>
<td>
debug-printing objects
</td>
<td>

```rust
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

let p = Point { x: 0.3, y: 0.67 };

// this now works; notice the {:?} debug-format
// prints "Point { x: 0.3, y: 0.67 }"
println!("{:?}", p);
```

</td>
</tr>

<tr>
<td>
PartialEq
</td>
<td>

</td>
<td>
comparing objects with `==`
</td>
<td>

```rust
#[derive(PartialEq)]
struct MyData {
    v: Vec<u32>,
    s: String,
}

let a = MyData { v: vec![1,2,3], s: "abcd".into() };
let b = MyData { v: vec![1,2,3], s: "xyzw".into() };

// this now works; prints "false"
println!("{}", a == b);
```

</td>
</tr>

</tbody>
