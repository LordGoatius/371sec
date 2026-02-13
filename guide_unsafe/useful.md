---
title: How to use Unsafe
author: Jimmy Ostler
options:
  implicit_slide_ends: true
theme:
  name: dark
---

Unsafe Rust
===

<!-- font_size: 2 -->
Hands-on day today, I'll be looking at real code I've written using unsafe.

Intro
---
<!-- font_size: 2 -->
Most of this comes from me looking through my practice OS I wrote before this class and aggregating the most used and useful unsafe functions

List
---
<!-- font_size: 2 -->
Pointers:
- read/write/(_volatile/_unaligned)
- add
- cast

Slices:
- to_raw_parts
- from_raw_parts

Functions:
- no_mangle
- link_section
- `extern "C"`

*Just these* cover the vast majority of my uses of `unsafe`

Instead of writing a long presentation, because that's very time consuming, I'm going to show an example for each of these.
