---
title: Testing in Rust
author: Jimmy Ostler
options:
  implicit_slide_ends: true
theme:
  name: dark
---

lib.rs
---
Calvin doesn't tend to like software, though they are very motivated to make sure everyone here gets a job.
I'm not entirely sure what job they think you'll be getting out of college with a Computer Science degree, but
it probably involves writing software.

But anyways.
<!-- pause -->
<!-- font_size: 2 -->
At some point in large projects, most code you write will be in a library.
<!-- pause -->
This is not a value statement.
<!-- pause -->
If you're using a languge with a built in testing framework, it's likely expected that
you know how to use it. If you're not, I'm so sorry.

`#[cfg(test)] mod tests {}`
---
<!-- font_size: 2 -->
Assume we have some functions we want to test

<!-- font_size: 1 -->
```rust
pub fn calculate(n: f64, i: i32) -> f64 {
  (n * 4.0).powi(i)
}

pub fn is_foo_set_env() -> bool {
  std::env::var("FOO").is_ok()
}
```
<!-- font_size: 2 -->
<!-- pause -->
...but we're not in the context of an executable file
<!-- pause -->
Rust provides a testing framework!

Just Write This!
---
```rust
# pub fn calculate(n: f64, i: i32) -> f64 {
#   (n * 4.0).powi(i)
# }
# 
# pub fn is_foo_set_env() -> bool {
#   std::env::var("FOO").is_ok()
# }
#[cfg(test)]
pub mod tests {
    #[test]
    fn test_one() {
        use crate::calculate;
        assert_eq!(calculate(9.0, -2), 6.0)
    }

    #[test]
    fn test_two() {
        use crate::is_foo_set_env;
        assert!(is_foo_set_env());
    }
}
```
<!-- font_size: 2 -->
The `#[cfg(test)]` is *conditional compilation*. It is only compiled when we run `cargo test`.
Additionally, since we're using `std`, we don't write a test handler. The Rust compiler runs every test function, and reports the result to us.

Why Test?
---
<!-- font_size: 2 -->
This is quite useful! However, it's not really how most businesses use testing.
<!-- pause -->
There's a few different ways of increasing the assurance your code works (using rust,
fuzzing, verification, testing, strict guidelines), and they're all different and come with tradeoffs.

For example:

Why Test? (cont.)
---
<!-- font_size: 2 -->
<!-- incremental_lists: true -->
- Using Rust:
  - Easy if you don't have a legacy code base
  - Not worth rewriting already testing/working code in
- Fuzzing:
  - Useful for some programs (parsers, programs that take in structured input)
  - Not as useful for other kinds of programs
- Verification:
  - Extremely high assurance that the program works
  - Extremely high cost in terms of time and effort
- Testing:
  - Easy, low cost, and can be integrated into deployment (CI/CD)
  - Often utilized incorrectly and frivously, for the sake of "coverage"
- Strict Guidelines:
  - Lower cost than verification, reasonably high assurance
  - More difficult and expensive to write, requires experts
<!-- incremental_lists: false -->

It's possible that you'll enncounter more than one of these at some point.

So... Why Test?
---
<!-- font_size: 2 -->
Testing is the most easily accessible, by far. Every language can do it, and
it *can* be used very effectively. Some people even write tests *before* writing any code,
which is called Test Driven Development (TDD). I've personally never done that, but I can see
how a software engineer could come up with that.

And regardless of whether testing is done well or not, you will encounter it at your job, in some way or another.
It make be a QA tester, it may be unit tests, or it may be TDD, but testing is ubiquitous.

Understanding it is important.

What *Should* We Test?
---
<!-- font_size: 2 -->
When we test, it's unlikely that testing `1 + 1 = 2` is helpful. Many tests end up doing something like this.
<!-- pause -->
We shouldn't test for compiler errors. We should test unenforced *invariants*
<!-- pause -->
If our code makes an assumption about code, but that assumption isn't enforced, we should test for that.
We should also document possible assumptions other code makes about our code.
<!-- pause -->
Once again, we do not need to test the output of a pure function more than once.

Examples
---
<!-- font_size: 2 -->
Let's assume we have some type which we create from an externally provided type.
<!-- font_size: 1 -->
```rust
use external::Type;

/// Must not contain numbers less than 65535
struct MyType(u64);

impl From<Type> for MyType {
  fn from(value: Type) -> MyType {
    MyType(value.into())
  }
}
```
<!-- font_size: 2 -->
Here, we assume that the external dependency `external::Type` enforces some invariant such that
it is impossible to result in a number less than 65535. It's easily possible to change this code to be more resilient
\- however it's likely in more complicated cases that will become difficult. Applying and enforcing assumptions is
perhaps the easiest way to simplify code, and we can do that using tests.

Codebases
---
<!-- font_size: 2 -->
It's hard, unless you've worked in a large codebase, to emphasize how small changes in one area can have
unintended consequences in code you assumed was unrelated.
<!-- pause -->
Testing is a way to try to turn these from subtle bugs that take weeks to track into a test failure that will
at least *alert* you to the fact that something may need to be changed.
<!-- pause -->
When testing, it's also important to understand the environment they run in

Github Actions (CI/CD):
---
<!-- font_size: 2 -->
In modern "devops" testing is integrated into deployment, which is continuous.
Your tests *will* be run when you push a commit. They will be run every time someone pushes a new commit.
It will be run by Github Actions, or some other similar platform.
<!-- pause -->
It can also be used for other things (sending updated code to external repositories such as PyPI).

GitHub Rust Actions
---
<!-- font_size: 2 -->
For a Rust project, this is what I use to run Rust tests when I push to this repository.
<!-- font_size: 1 -->
```yaml
on:
  push:
    branches: [ main ]
jobs:
  build:
    runs-on: ubuntu-latest
    strategy:
      matrix:
        BUILD_TARGET: [release] # refers to a cargo profile
    outputs:
      release_built: ${{ steps.set-output.outputs.release_built }}
    steps:
      - uses: actions/checkout@v5
      - run: rustup override set nightly
      - name: Build binaries in "${{ matrix.BUILD_TARGET }}" mode
        run: cargo build --profile ${{ matrix.BUILD_TARGET }} --no-default-features
      - name: Run tests in "${{ matrix.BUILD_TARGET }}" mode
        run: cargo test --profile ${{ matrix.BUILD_TARGET }} --no-default-features
```
<!-- font_size: 2 -->
You can configure these in so many ways, as Calvin mentioned running them weekly to ensure
updates to software don't break functionality.

Homework
---
<!-- font_size: 2 -->
In the homework for next week, you'll be writing some integration tests. It's different, since
you're writing on a bare-metal target on an unhosted system with a custom tester.

However, the idea is the same.

CPU Execution
---
<!-- font_size: 2 -->
I know you all were waiting for this, because this is so cool!

Let's say we have a keyboard.
It's MMIO. There's an address that represents the most recently pressed key.
How do we read input from it?
