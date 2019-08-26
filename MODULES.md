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
- to avoid writing `module_name::` repeatedly, you can import with `use module_name;:contained thing`

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

Filename: src/bin.rs
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
            println!("transfering {} to {}", amount, where);
        }
    }
}
```

### Multi file

Directory tree:

```
src/
|-- bin.rs
|-- bank.rs
`-- bank/
    `-- vault.rs
```

Filename: src/bin.rs
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
    println!("transfering {} to {}", amount, where);
}
```


## Modules can make things private
