---
title: Configuring Tools on Linux
author: Jimmy Ostler
options:
  implicit_slide_ends: true
theme:
  name: dark
---


Configs
===

Configs
---

<!-- font_size: 2 -->
On Linux, most applications come with very barebones behavior.

<!-- pause -->
This does not mean they do not *support* advanced behaviors. They simply need to be configured.

<!-- pause -->
Vim is a good example. Basic vim doesn't look great, is hard to understand, and feels unpolished.

<!-- pause -->
However, this isn't a requirement. Vim (and neovim, helix, etc) are all designed to be configurable.

Config Files
---

<!-- font_size: 2 -->
On your linux system (or subsystem [or unix-like system]) there will be a directory (folder) where
confiuration files are meant to go, usually `~/.config` (`~` represents the HOME or USER directory).

Applications will have a folder here where you're expected to put any custom configuration.


For example: my helix (text editor) config lives in `~/.config/helix/config.toml`

My Helix Config
---

```bash +exec_replace
cat /home/lordgoatius/.config/helix/config.toml
```

Configuration Difficulty
---

<!-- font_size: 2 -->
Some programs are easier to config that others. Helix is in general considered easy,
neovim is considered easy but complicated, and vim is considered hard.

<!-- pause -->
This is primarily through the language these programs chose to represent configuration.

<!-- pause -->
- Helix chose TOML, which is what `cargo` uses as well (in the `Cargo.toml` file in every rust crate).
- Neovim uses Lua, a programming language kinda like python but everything is a dictionary.
- Vim uses vimscript, which is nearly universally reviled and used nowhere else.

<!-- pause -->
This is essentially the only reason neovim is more popular than vim.

Configuring Neovim
---

<!-- font_size: 2 -->
<!-- pause -->
Because configuring vim is awful I will not be showing how to do that.

<!-- pause -->
Instead, I'll be showing how to configure `neovim`, which I recommend everyone who
wishes to have LSP (autocomplete + inline error messages) support do as well.

<!-- pause -->
Alternatively you may use `helix`, which is the easiest to configure, with directions and a guide
available online: [https://docs.helix-editor.com/configuration.html]

Neovim Cont.
---

<!-- font_size: 2 -->
Before we configure neovim, I want to say there are many, *many* guides online about this.

<!-- pause -->
I learned from them, and it's likely many of them will know better than me.

My Neovim Configuration Loop:

1. Install Package manager
2. Lookup behavior I want, find package
3. Add package, reading the readme.md to configure the package however I'd like
4. GOTO 2 until satisfied

I used the Lazy package manager since it's widespread and was the first one I used that worked.

Lazy Config
---

<!-- font_size: 2 -->
The link: https://lazy.folke.io/installation

This will provide you with instructions on how to enable the lazy package manager, as well as guides
on how to use it. I believe reading documentation and learning from it is an essential skill, and therefore
once I go over helix, I'll start a jlab (jimmy lab) and allow everyone to begin configuring their setups.

The end goal is to be able to see errors inline while editing Rust without needing to
`cargo check` or `cargo build`. (show my example in helix)

Helix Config
---

<!-- font_size: 2 -->
Helix configuring is different from neovim since neovim uses a turing complete programming language and runs the script
every time you open the program.

Helix has a set amount of options that can be changed from the default, which are read
and modify the behavior of the program, but nothing turing complete, and (currently, in progress) no plugin system

The helix documentation is a nearly fully featured guide to using and configuring helix.

Lab Time!
===

<!-- font_size: 2 -->
I will be here to answer questions, though since this lab is also an exercise in reading and understanding
documentation, my assistance will first be to find the relevant documentation, and then in understanding it.
