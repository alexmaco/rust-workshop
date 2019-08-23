Symmary of [Appendinx C from the book](https://doc.rust-lang.org/book/appendix-03-derivable-traits.html)

These are the standard traits we can `#[derive(NameOfTheTrait)]`

| Trait | Requires | Useful for | Example |
| --- | --- | --- | --- |
| Copy | Clone | automatic copying of objects | `let x = 3; foo(x); bar(x);` |
| Clone | | deep-copying objects | `let v = vec![1,2]; foo(v.clone()); // v still in scope, only clone was passed` |
| Debug | | debug-printing objects | `println!("{:?}", thing);` |
| PartialEq | | comparing objects with `==` | `if my_obj_a == other_obj_a { ... ` |
| Eq | PartialEq | marks equality as transitive | required when using types in a `HashSet` or as keys in a `HashMap` |
| PartialOrd | | comparing objects for order | `obj_x < obj_y` |
| Ord | PartialOrd | marks ordering as transitive (total) | required when using types in a `BTreeSet` or as keys in a `BTreeMap` (ordered) |
| Hash | | getting the hash of an object | required when using types in a `HashSet` or as keys in a `HashMap` |
| Default | | getting a "default" state for an object | `let s = String::default(); // empty string` |
