# Conversions

- Sometimes, we have an object of A, and we need to turn it into an object of type B.
- Conversion is expressed by implementing `From<A>` for B (or, rarely, implementing `Into<B>` for A)
  - If we implement `From<A> for B`, then `Into<B> for A` is also automatically implemented.
- After we implement `From`, we call the conversion code with `B::from(a)`

Docs:

- From: <https://doc.rust-lang.org/core/convert/trait.From.html>
- TryFrom: <https://doc.rust-lang.org/core/convert/trait.TryFrom.html>

```rust
#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

#[derive(Debug)]
struct Cents(u32); // alternative way to define a struct, like a tuple but named

impl From<Coin> for Cents {
    fn from(coin: Coin) -> Self {
        let value = match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        };

        // return an object of our Self type (i.e. Cents), constructed from coin
        Cents(value)
    }
}

let cents_1 = Cents::from(Coin::Quarter); // Cents::from works !
println!("converted: {:?}", cents_1);

let cents_2: Cents = Coin::Penny.into(); // .into() works automatically
println!("also converted: {:?}", cents_2);
```

Some implementations of `From` and `Into` are predefined, on types like `String` and `&str`

```rust
let s1: &str = "text";
let s2 = String::from(s1); // call impl From<str> for String
let s3: String = s1.into(); // convert s1 into whatever is needed

let large_val = 12345;
println!("conversion result: {:?}", u8::try_from(large_val));
```

## Conversions that can fail

`From` is for conversions that always work. If our conversion can fail we use `TryFrom`.

```rust
use std::convert::TryFrom;

impl TryFrom<Cents> for Coin {
    type Error = String; // we must define the type returned on error

    fn try_from(cents: Cents) -> Result<Self, Self::Error> {
        match cents.0 {
            // report the Coin for good values in Ok
            1 => Ok(Coin::Penny),
            5 => Ok(Coin::Nickel),
            10 => Ok(Coin::Dime),
            25 => Ok(Coin::Quarter),

            // report Err (of the type defined above, String) for values that are not coins
            x => Err(format!("there is no coin that is worth {} cents", x)),
        }
    }
}

println!("good: {:?}", Coin::try_from(Cents(10)));
println!("bad: {:?}", Coin::try_from(Cents(15)));
```

## Specialized `ToString` conversion

For the common case of converting a type to `String`, there exists the trait [ToString](https://doc.rust-lang.org/std/string/trait.ToString.html).

As in the examples above, it is preferable to implement a known trait, than to have custom conversion functions.

```rust
use std::string::ToString;

impl ToString for Cents {
    fn to_string(&self) -> String {
        format!("{} cents", self.0)
    }
}

let out = Cents(45).to_string();

println!("the value is: {}", out);
```
