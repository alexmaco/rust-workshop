# Rust Crash Course

This is a quick and fairly broad set of documents to get you started learning [Rust](https://www.rust-lang.org/).

This is targeted at people who already know at least one programming language, and have a working knowledge of common programming concepts, like lists, iteration, functions, etc.
<br/>
It's intended to give a hard-and-fast practical overview of the language, focused on laying out common practical issues rather than diving deep into the theory.

## Overview

This is intended as a crash course into the Rust language, for people who otherwise know how to program, but are not necessarily experts.
It is largely example-driven, with short explanation snippets, and small exercises for each section.

Although this was structured initially to support a live workshop, it can be used by oneself. In a live setting an instructor should be present to answer any questions. When used alone, some extra searching or asking a friend may prove useful instead.

## Index:

* [`SETUP.md`](./SETUP.md) for installation instructions
* [`DAY_1.md`](./DAY_1.md) covers the language basics
* [`APPS_1.md`](./APPS_1.md) has some more advanced applicative exercises, which only rely on the info from `DAY_1.md`
* [`OPTION_AND_RESULT.md`](./OPTION_AND_RESULT.md) shows common usage examples of the `Option` and `Result` types from the standard library
* [`CONVERT.md`](./CONVERT.md) demonstrates conversions using the common `From`, `TryFrom`, `Into`, and `TryInto` traits
* [`DAY_2.md`](./DAY_2.md) looks at common patterns and real life applications
* [`THREADS.md`](./THREADS.md) shows thread and channel usage, and has a large exercise to practice almost everything covered so far
* extra useful topics:
  * [`DERIVE.md`](./DERIVE.md) looks at common `#[derive(...)]` annotations and their usage
  * [`MODULES.md`](./MODULES.md) covers splitting a program into multiple files and modules

## Acknowledgements

This documentation was initially developed during the fall of 2019 at Keysight Technologies, in order to serve as support for an internal workshop on the Rust programming language.

The primary motivation was that, generally a company wanting to adopt Rust for a project wants to make its people productive as quickly as possible.

We developed this because we felt that a lot of other resources were either aimed at beginners, and thus too detailed for intermediate programmers, or were aimed at very advanced users, and hence were too brief. A lot of programmers know several languages, and would like a quick way to become acquainted with Rust and be productive.

The model we tested was a 2-day workshop, with 6 hours per day, in which attendants would have a working Rust setup configured beforehand.
<br/>
Throughout, we assumed the presence of 2 instructors per 12-16 attendants. The instructors would monitor the attendant's progress, offering extra explanations for points that prove difficult, and pacing the workshop by live-coding some examples.

This proved a auccessful approach, with other team members quickly becoming able to contribute to production Rust code.
