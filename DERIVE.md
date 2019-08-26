Summary of [Appendix C from the book](https://doc.rust-lang.org/book/appendix-03-derivable-traits.html)

These are the standard traits we can `#[derive(NameOfTheTrait)]`

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
println!("{} {}", p, p2);
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
deep-comparing objects with `==`
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

<tr>
<td>
Eq
</td>
<td>
PartialEq
</td>
<td>
marks equality as transitive (total)<br/>
required when using types in a <b>HashSet</b> or as keys in a <b>HashMap</b>
</td>
<td>

see <b>HashSet</b> example below

</td>
</tr>

<tr>
<td>
PartialOrd
</td>
<td>
PartialEq
</td>
<td>
deep-comparing comparing objects with `<`, `<=`, `>=` and `>`
</td>
<td>

```rust
#[derive(PartialOrd)]
struct MyData {
    v: Vec<u32>,
    s: String,
}

let a = MyData { v: vec![1,2,3], s: "abcd".into() };
let b = MyData { v: vec![1,2,3], s: "xyzw".into() };

// this now works; prints "true"
println!("{}", a < b);
```

</td>
</tr>

</td>
</tr>

<tr>
<td>
Ord
</td>
<td>
PartialOrd
</td>
<td>
marks ordering as transitive (total)<br/>
required when using types in a <b>BTreeSet</b> or as keys in a <b>BTreeMap</b>
</td>
<td>

see <b>BTreeSet</b> example below

</td>
</tr>

<tr>
<td>
Hash
</td>
<td>

</td>
<td>
getting the hash of an object<br/>
required when using types in a <b>HashSet</b> or as keys in a <b>HashMap</b>
</td>
<td>

see <b>HashSet</b> example below

</td>
</tr>

<tr>
<td>
Default
</td>
<td>

</td>
<td>
constructing an object in a basic, "default" state
</td>
<td>

```rust
#[derive(Default)]
struct MyData {
    v: Vec<u32>,
    s: String,
}

// this now works; v is an empty Vec, and s is an empty String
let a = MyData::default();

// alternative way to write
let b: MyData = Default::default();
```

</td>
</tr>

</tbody>
</table>

### HashSet usage

```rust
#[derive(Hash, Eq, PartialEq)]
struct MyData {
    v: Vec<u32>,
    s: String,
}

use std::collections::HashSet;

// this now works
let mut set: HashSet<MyData> = HashSet::new();
```

### BTreeSet usage

```rust
#[derive(Eq, PartialEq, Ord, PartialOrd)]
struct MyData {
    v: Vec<u32>,
    s: String,
}

use std::collections::BTreeSet;

// this now works
let mut set: BTreeSet<MyData> = BTreeSet::new();
```
