# Modules

(based on [Chapter 7 from the official book](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html))

## Modules group things

- modules are defined with the `mod` keyword
- useful for grouping logically-related things

```rust
fn main() {
    bank::pay(bank::Payment::Loan);
}

mod bank {
    pub fn pay(p: Payment) {
        match p {
            Payment::Loan => println!("paying a loan"),
            Payment::Credit => println!("paying off the credit card"),
        }
    }

    pub enum Payment {
        Loan,
        Credit,
    }
}
```

### Importing from a module

- to use things from a module prepend the module name and `::`
- to avoid writing `module_name::thing` repeatedly, you can import `thing` with `use module_name::thing`

```rust
// for the bank module defined above
use bank::pay;
use bank::Payment;

fn main() {
    pay(Payment::Loan);
}
```

## Modules for multifile projects

- modules can be written (and nested) in the same file
- when splitting code into multiple files, one file == one module (with the same name)
- submodules always go in a directory named as the parent module

The following 2 layouts are equivalent:

### Single file

Filename: src/main.rs

```rust
fn main() {
    bank::pay(bank::Payment::Loan);
}

mod bank {
    pub fn pay(p: Payment) {
        match p {
            Payment::Loan => println!("paying a loan"),
            Payment::Credit => vault::transfer(5, "the boss"),
        }
    }

    pub enum Payment {
        Loan,
        Credit,
    }

    mod vault {
        pub fn transfer(amount: u32, where: &str) {
            println!("transferring {} to {}", amount, where);
        }
    }
}
```

### Multi file

Directory tree:

```
src/
|-- main.rs
|-- bank.rs
`-- bank/
    `-- vault.rs
```

Filename: src/main.rs

```rust
mod bank; // this is required - it includes `bank.rs` in the project

fn main() {
    bank::pay(bank::Payment::Loan);
}
```

Filename: src/bank.rs

```rust
mod vault; // this is required - it includes `bank/vault.rs` in the project

pub fn pay(p: Payment) {
    match p {
        Payment::Loan => println!("paying a loan"),
        Payment::Credit => vault::transfer(5, "the boss"),
    }
}

pub enum Payment {
    Loan,
    Credit,
}
```

Filename: src/bank/vault.rs

```rust
pub fn transfer(amount: u32, where: &str) {
    println!("transferring {} to {}", amount, where);
}
```

## Modules control visibility

- by default, everything in a module is private to the outside
  - everything, including struct fields
- to make things visible to the parent module, use the keyword `pub` before the item

```rust
fn main() {
    let mut trn = bank::Transaction::new(20);
    //trn.euros = 0; // error: the euros field is private
    trn.description = String::from("just a small donation");

    bank::send(trn);
}

mod bank {
    // this struct is public
    pub struct Transaction {
        pub description: String, // this can be read and changed
        euros: u32, // this is private to the module
        state: State, // this is also private
    }

    // this is private to the module
    enum State {
        Pending,
        Resolved
    }

    impl Transaction {
        // the `new` function is also public so we can use it from `main`
        pub fn new(euros: u32) -> Self {
            Self { euros, description: "".into(), state: State::Pending }
        }
    }

    pub fn send(mut t: Transaction) {
        // this is inside the module, so we can access `description`
        println!("sending {} euros; ({})", t.euros, t.description);

        // we can also use the private types here
        t.state = State::Resolved;
    }
}
```
