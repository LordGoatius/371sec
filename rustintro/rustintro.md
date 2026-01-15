---
title: Intro to Advanced Rust
author: Jimmy Ostler
---

Rust in Systems Programming
----

<!-- font_size: 2 -->

Q: What makes Rust capable of doing systems programming (writing an operating system)?

What differentiates it from other systems languages, such as C?

<!-- end_slide -->

<!-- font_size: 2 -->

Rust topics you should already be very familar with:
- Functions in Rust
- User defined types (`struct`, `enum`)
- Methods and Traits in Rust
- Control flow in Rust (`if`, `for`, `match`) (maybe not `while let` or `if let`)

Rust topics you should be somewhat familar with:
- References and Smart Pointers
- Lifetimes
- `Clone` vs `Copy`
- Mutable xor aliasing (`&mut` vs `&`)

Rust topics you should not be familar with:
- Unsafe Rust
- Pointers
- Higher order trait bounds
- Subtype Variance
- Threads and Atomics
- Concurrency Primitives
- `core` vs `std`
- FFI / ABI

<!-- end_slide -->

<!-- font_size: 2 -->
<!-- column_layout: [3, 3] -->

<!-- column: 0 -->
My Job as Section Leader:
----

See what Calvin teaches in class, and then
- Fill in the missing pieces
- Explain concepts that people didn't understand
- Prepare you for the next classes
- Explain the Labs

<!-- pause -->
<!-- column: 1 -->
My Job as Tutor:
----

See what homework has been assigned, and then
- Explain concepts that allow you to complete the homework
- Assist in understanding the mechanics of writing Rust
- ***Not*** to complete the assignments for you

<!-- end_slide -->

`wc`
----

<!-- font_size: 2 -->
The homework for next week is to implement `wc`.

The goal is to write a program that takes in a file name and can output
- the newline count
- the word count
- the byte count

The obvious first step is to take in an argment passed to the program.

```rust +exec
# fn main() {
let args = std::env::args().collect::<Vec<_>>();
println!("{:?}", args.get(0));
println!("{:?}", args.get(1));
# }
```

<!-- end_slide -->
<!-- font_size: 2 -->

Q: What does `std::env::args()` return and what does the `collect::<Vec<_>>` method do?

<!-- pause -->

<!-- font_size: 1 -->
Let's look at the docs:
```rust
pub struct Args {
  inner: ArgsOs,
}
/// Implements notable traits: Iterator<Item = String>
```
Helpful, but could be more helpful. Maybe `collect` will clear things up more.

<!-- pause -->
```rust
// pub trait Iterator
fn collect<B>(self) -> B
where
    B: FromIterator<Self::Item>,
    Self: Sized,
/// Transforms an iterator into a collection.
///
/// `collect()` can take anything iterable, and turn it into a relevant
/// collection. This is one of the more powerful methods in the standard
/// library, used in a variety of contexts.
```
<!-- alignment: center -->
(Let's break this down.)
<!-- pause -->
<!-- font_size: 2 -->
So `collect` takes any type which implements the `Iterator` trait, and returns a collection.
We can see what a collection is by looking at the docs of the `std::collections` module in the standard lib.

We can find this here:

[Collections](https://doc.rust-lang.org/std/collections/index.html)

<!-- end_slide -->

<!-- font_size: 2 -->

Why is this important?

Because `Iterator`s are *incredibly* powerful in Rust.

I use iterator methods more than I use `for` loops.

It takes some time to get used to, but is expected for someone who knows Rust to understand.

For `wc`, many useful `String` and `Vec` functions return iterators.

<!-- end_slide -->
<!-- font_size: 2 -->

There are 2 main ways to read a file in Rust.

<!-- column_layout: [1, 1] -->

<!-- column: 0 -->
```rust
# fn main() {
fs::read_to_string(&path)
# }
```

This function returns a `String`, which is always a utf-8 encoded byte string.

<!-- column: 1 -->
```rust
# fn main() {
fs::read(&path)
# }
```

This function returns a `Vec<u8>`, which is an array of any bytes

<!-- reset_layout -->

What is the difference? Well, the type `String` has an invariant, and the standard library
has to uphold these invariants. This means if the file *isn't* valid utf-8, `read_to_string` will fail,
but `read` will still succeed.

Which one should we use here?

<!-- end_slide -->

Basic Iterator Functions
----

<!-- font_size: 2 -->
Every time I'm working with one iterator, there's a few methods that come up very consistently.

- `enumerate`
- `filter`
- `map`
- `take`
- `fold`
- `reduce`
- `split`
- `count`
- and of course, `collect`

When I'm working with 2, the following methods tend to be the most helpful

- `zip`
- `chain`

There exist more, but this is sufficient for now.

You may ask: Why?

<!-- end_slide -->

Functional Programming
----

<!-- font_size: 2 -->
Let's say we have 2 blocks of code we need to understand

<!-- column_layout: [1, 1] -->

<!-- column: 0 -->
```rust
# fn main() {
let mut sum = 0;
for i in 0..b {
  sum += b;
}
# }
```

<!-- column: 1 -->
```rust
# fn main() {
let sum = (0..b).sum();
# }
```

<!-- reset_layout -->
Which one is easier to understand? As the code got more complicated, which style do you think you could read and understand better?

There's a name for this difference. The left is *imperative*, and the right is *declarative*.

Traditionally, procedural (and OOP) languages are *imperative*, and functional programming languages are *declarative*.

Rust is a multi-paradigm language, and therefore has elements of both.

<!-- end_slide -->

Functional Programming Cont.
----

<!-- font_size: 2 -->

Functional programming usually avoids *mutation*. In Rust, mutation is *opt-in* using
the `mut` keyword.

Functional programming is usually done through the combination of *pure functions* using functions called **combinators**.
A pure function does not mutate anything or change any state. Instead, it returns a value based on the input.

<!-- new_line -->
<!-- column_layout: [3, 5] -->

<!-- column: 0 -->
```rust
fn sum(n: usize) -> usize {
  (0..n).sum()
}
```

<!-- column: 1 -->
```rust
fn sum(n: usize, sum: &mut usize) {
  *sum = (0..n).num();
}
```

<!-- reset_layout -->
<!-- font_size: 2 -->
<!-- alignment: center -->
<!-- new_line -->

Which of these is a pure function and why?

<!-- end_slide -->

Functional Programming Cont.
----

<!-- font_size: 2 -->
Functional programming, while very different from what you may be used to, is widely considered to
be harder to write bugs in, especially when combined with a strict type system such as Rust's.

<span style="color: blue">**IMPORTANT**</span>: Rust is *not* a functional programming language.
It is, however, heavily inspired by them, and the presence of iterators is a big part of that.

Let's write some iterator style code.

<!-- end_slide -->

FooBarBaz
---

<!-- font_size: 2-->
FooBar: FooBar is a technique for learning to use control flow. Iterate from 1 to n, and for each number, if it
- divides 3: print "Foo"
- divides 5: print "Bar"
- divides 3 and 5 (15): print "FooBar"
- anything else: print number (in our case, for learning purposes, nothing)

<!-- end_slide -->
<!-- font_size: 2-->
Let's start with a Rusty Procedural solution
<!-- pause -->
<!-- font_size: 1-->
```rust +exec
# pub fn main() {
fn foobar(n: usize) {
  for i in 1..=n {
    match (i % 3, i % 5) {
      (0, 0) => println!("FooBar"),
      (0, _) => println!("Foo"),
      (_, 0) => println!("Bar"),
      _ => println!("{i}"),
    }
  }
}

foobar(15);
# }
```

<!-- pause -->
<!-- font_size: 2 -->
We do not use if statements because we are using a superior language, and can pattern match.

<!-- end_slide -->

<!-- font_size: 2 -->
In the beginning, there was `core::ops::range::Range<Idx>`.

This is the type returned by the `..` binary operator. This type implements the `IntoIterator` trait, and therefore can be turned into an iterator and used in `for` loops.

```rust +exec
# fn main() {
# use std::any::type_name_of_val;
println!("{}", type_name_of_val(&(0..5)));
# }
```

This will be the starting point of our *new, better* foobar.

<!-- end_slide -->

<!-- font_size: 2 -->

```rust +exec
# fn main() {
fn foobar(n: usize) {
  let range = 1..=n;
  println!("{range:?}");
}

foobar(5);
# }
```

<!-- pause -->
You'll notice that it doesn't print every number in between, just the bounds. This is because iterators are *lazily evaluated*, meaning that until we actually use the iterator, it doesn't do anything.

This is part of Rust's principle of *zero-cost abstractions*, meaning that using iterators should be *as efficient or more* than using a for loop.

<!-- end_slide -->

<!-- font_size: 2 -->

```rust +exec
# fn main() {
fn foobar(n: usize) {
  (1..=n).for_each(|val| {
    println!("{val}");
  });
}

foobar(5);
# }
```
<!-- pause -->
The simplest function we can do on our iterator is the `for_each` function. It simply calls the function passed into it for each element of the iterator.

<!-- end_slide -->

(Diatribe)
---

<!-- font_size: 2 -->
This syntax may be a little unfamilar. I said we passed a function into `for_each`, but that didn't look like a function.

However: it is! It's an anonymous function called a "closure" (slight misnomer actually). I prefer calling them lambdas.

It allows creating a function inline, instead of giving it a name.

```rust +exec
fn main() {
  let print = |val| println!("{:?}", val);
  print(5);
}
```

<!-- pause -->
You may be familar with lambdas from Calvin's awful python style.
<!-- end_slide -->
(Diatribe Pt 2)
---

<!-- font_size: 2 -->
Essentially:
<!-- column_layout: [12, 1, 12] -->
<!-- column: 0 -->
```rust
fn test(val: Ty) -> Ty2 {
  ...
}
```
<!-- column: 1 -->
<!-- font_size: 3 -->
=
<!-- column: 2 -->
<!-- font_size: 2 -->

```rust
let test = |val| {
  ...
}
```
<!-- pause -->
<!-- reset_layout -->
Type labels can usually be inferred and are not required by lambdas, but they are *not* dynamic.

You cannot use the same lambda with 2 different input or output types, even if it is unlabelled.

Lambdas are used in lots of iterator functions, so they are good to understand.

<!-- end_slide -->

FooBar Cont.
----
<!-- font_size: 2 -->
The first function we'll use is called `filter`. It takes in a function that takes in a
reference to the iterator value, and returns a boolean. Any value which produces a `false` is
not included in subsequent iteration.

```rust +exec
# fn main() {
fn foobar(n: usize) {
  (1..=n)
    .filter(|val| val % 3 == 0 || val % 5 == 0)
    .for_each(|val| println!("{val}"));
}

foobar(5);
# }
```
<!-- end_slide -->
FooBar Cont.
----
<!-- font_size: 2 -->
The next function we'll use is `map`. It takes in a function that has this signature:
```Fn(B) -> F```, or in other words, transforms a type `B` into type `F`. What this means is we can
change the type our iterator outputs!

```rust +exec
# fn main() {
(0..8)
  .map(|num| (num * 7) as f32)
  .for_each(|val| println!("{val:?}"))
# }
```
<!-- end_slide -->
FooBar Cont.
----

```rust +exec
# fn main() {
fn foobar(n: usize) {
  (1..=n)
    .filter(|val| val % 3 == 0 || val % 5 == 0)
    .map(|val| {
      if val % 15 == 0 {
        "FooBar"
      } else if val % 5 == 0 {
        "Bar"
      } else {
        "Foo"
      }
    })
    .for_each(|val| println!("{val}"))
}

foobar(15);
# }
```

<!-- pause -->
<!-- font_size: 2 -->
Because we omitted numbers that are not divisible than 5 or 3, we don't print anything other than Foo or Bar.

<!-- end_slide -->

Concluding FooBar and Iterators
===

<!-- font_size: 2 -->
Iterators in Rust can do a *lot* more than FooBar. I implemented simple `wc` in
5 lines of Rust code using them. They're worth learning, for many, many reasons,
including multithreaded performance (`rayon` library in Rust).

I've implemented things like ternary multiplication and division using only iterators.

GOTO: Dice Example, WC example

<!-- end_slide -->

<!-- font_size: 2 -->
<!-- new_lines: 8 -->
<!-- font_size: 5 -->
Part 2
===

<!-- font_size: 3 -->
<!-- alignment: center -->
**Invariants**

<!-- end_slide -->
Invariants
---

In Rust, we can say that safe Rust is safe because it maintains certain "invariants" at compile time.

An invariant is something that can never change throughout a program, and must always be true.

For example: there must never exist a `&` and `&mut` to the same element.

```rust +exec
# fn main() {
let mut val = 5;
let valref = &val;
let mutvalref = &mut val;
assert_eq!(*valref, 5)
# }
```

^^^ Causes Errors

<!-- end_slide -->

Invariants
---
<!-- font_size: 2 -->

A simple invariant you've probably experienced has to do with types. In safe Rust, you
cannot assign a type an invalid value. The typechecker enforces this at compile time.
*(Type safety can be viewed as a form of memory safety, as violating it can lead to
memory corruption. Types are stored in memory after all)*

However, the most important invariants that safe Rust upholds all have to do with "memory safety".
This means that there are things which can always be said to be true about a safe Rust program.

Invariants EXIST IN OTHER LANGUAGES. They are just enforced by the programmer, not the compiler.
This is why Rust is actually easier than C :)

<!-- end_slide -->

Unsafe
---
<!-- font_size: 2 -->

Rust is actually 2 languages.

One is safe. The other is not.

You can opt into the unsafe version by using the `unsafe` keyword.

```rust +exec
# #![allow(clippy::all)]
# fn main() {
unsafe {
  let ptr: *mut usize = 0x0usize as *mut usize;
  *ptr = 12;
}
# }
```

This is not memory safe and crashes our program.

<!-- end_slide -->

Unsafe
---

We can also violate type safety.

```rust +exec
# #![allow(clippy::all)]
# #![allow(unused)]
# fn main() {
#[repr(u8)]
enum Vehicle {
  Car = 0,
  Plane = 1,
}

let vehicle: Vehicle = unsafe { std::mem::transmute(2u8) };
# }
```

We crash here because this code is running in debug mode, but would not be enforced in a
production environment. We have violated type safety.

Notably, this is allowed *if* you maintain type safety.

<!-- end_slide -->
<!-- new_lines: 4 -->
<!-- font_size: 4 -->
Safe Unsafe
---

<!-- alignment: center -->
GOTO Jimmy's Set Example

<!-- end_slide -->
Unsafe
---

<!-- font_size: 2 -->
Unsafe can exist in a few contexts:

Code Blocks
<!-- font_size: 1 -->
```rust
unsafe {
  std::mem::transmute(3);
}
```

<!-- font_size: 2 -->
Unsafe Functions
<!-- font_size: 1 -->
```rust
unsafe fn myfn() { ... }
```

<!-- font_size: 2 -->
Unsafe traits:
<!-- font_size: 1 -->
```rust
unsafe trait MyTrait { .. }
```

<!-- font_size: 2 -->
The last 2 mark something as unsafe to use, and the first gives you a context where you are allowed to use it.

<!-- end_slide -->

Pointers
---

<!-- font_size: 2 -->
The most important thing that Rust gives access to in `unsafe` is dereferencing raw pointers.
A raw pointer `*mut T`/`*const T` is similar to a reference, except the compiler doesn't uphold any
invariants about them, it simply assumes we do so.

We can create them using this syntax, similarly to using references.
```rust
let val = 17;
let ptr = &raw mut val;
```

Those familar with C will likely already be familar with these.

<!-- end_slide -->

Deref
---
<!-- font_size: 2 -->
In Rust, the deref operator is `*`. It works on references, and can be overriden (safely) using the
`Deref` trait.

<!-- pause -->
It is always safe to dereference a reference (`&`/`&mut`) because it will always be valid.
It is never safe to dereference a pointer because its validity is determined by us, the programmer.

<!-- pause -->
This begs the question: Why use pointers and unsafe?

<!-- pause -->
A: If you're working on high-level applications you likely will never need unsafe. However:
- It allows the programmer to (**<span style="color: red">CAREFULLY</span>**) optimize a program
- It allows access to lower level abstractions necessary for working with hardware.
- It allows access to directly working with other programming languages (FFI). (This is what I was paid to
do at Cloudflare, and why I know so much about unsafe Rust)

Some things are easier with unsafe! Many `std` data structures use it. But we *must* understand safe Rust's rules first.

<!-- end_slide -->
Quick Invariants:
---

<!-- font_size: 2 -->
- A value in rust is always initalized when accessed
- References `&` always point to valid memory
- Slices `&[T]` always point to a finite length of a valid array of type `T`
- A `&` and `&mut` never point to overlapping memory
- A reference is always aligned to `size_of<usize>` bytes and points to aligned memory
- Two `&mut` never point to overlapping memory
- Memory pointed to by a `&` is never mutated
- Unsafe compiler intrinsics are not called
- A caught `panic!` results in a program with all other invariants upheld
- The `!` (never) type is never constructed
- `dyn Trait` pointer metadata is always the vtable for the trait

All of these can be violated using `unsafe`. It is up to *us* to make sure they are not.

*Disclaimer: This list is not comprehensive.

Calvin will go over `unsafe` next week, and you will begin writing `unsafe` code.

<!-- end_slide -->
Useful Tools
===

<!-- font_size: 2 -->
- Cargo
- Miri
- Godbolt.org (compiler explorer)
- Rust Playground
- Rust Book
- Rustonomicon
