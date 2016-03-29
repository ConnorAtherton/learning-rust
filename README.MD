# Learning Rust

## Installing

For linux or Mac you need to install the install script like so:

`curl -sSf https://static.rust-lang.org/rustup.sh | sh`

This script is responsible for installing the Rust compiler and the Rust community package
manager, Cargo. We'll look into Cargo in more detail further on.

It's a good idea to check these type of install scripts - where you are curling
directly from the internet - just so you understand what they will do to your
system when you run them.

Here are the main things this script will do

- Checks that all the POXIX commands it needs are available on your system, look at `assert_cmds` on
  line **1920** to see the full list Rust requires. In most cases the base operating system
  installation will have these tools available -- mostly from the [Coreutils
  package](http://www.gnu.org/software/coreutils/manual/coreutils.html).

- Creates a directory for rust to install information it needs to place on the file system. Rust
  will add this information to `~/.rustup` by default. You can override this placement by setting
  the `RUSTUP_HOME` environment variable before running the install script. This directory will be
  deleted after the installation is complete unless you explicitly pass the `--save` option.

NOTE: The Rust compiler is called `rustc` and not just `rust`.

When the installation script is finished, check that the binary installed correctly by checking the
version `rustc  --version` and `cargo --version`.

## Code conventions

- Like Ruby, file names should be **snake_case**, so `file_name.rs` instead of `filename.rs`.
- Use the `-o` flag specify where the binary executable should be places when compiling code or it will be
  places in the directory you ran the `rustc` command.

## Using external code

Using the rust executable is only suitable for small projects without external dependencies. Since
most useful software is large enough to become a management burdon, and requires using code that
other people have built, we need a tool to act as a build system. This is analagous to complex
Makefiles from C or C++ land.

Cargo is the tool that acts as the build system for Rust projects. Cargo enforces a project
structure in order to work correctly. Library code must be placed inside a `src`, tests go in
`tests`, and the executable output Cargo builds is placed inside a `target`. You can tell Cargo
to create the base structure by running `cargo new <project_name>`.

Before you can use Cargo to build your project, you need to create a configuration file names
`Cargo.toml`. It's an INI-like file format with sections specific for Rust project. Check the
docs for the full list. The important ones starting out are the `[package]` and `[dependencies]`
sections. The former is the place to descibe information about our code and the latter is used
for specifying what other code we need to make our code function correctly.

```
[package]
name = "hello_world"
version = "0.0.1"
authors = [ "Connor Atherton <user@example.com>" ]

[dependencies]
regex = "0.1.41"
```

### Types of Rust packages

Packages of Rust code -- usually a standalone project -- is referred to as a `crate`.
There are two types of creates:

**library** is code that is intended to be used by other programs.

**Binary** is an executable designed to be run standalone. Binary crates will usually pull in
library crates as dependencies.

## Notes

- Like lots of other mainstream languages, a `main` function is required as an entrypoint to the
  program.
- Rust has macros. They can be distinguished from normal function invocations because macros always
  end with a `!`.
- Vim has terrible support for Rust right now so you need to install the rust.vim plugin to get
  thing like syntax highlighting and autoformatting.
- The indentation conventions appears to be 4 spaces in the docs.
  [Servo](https://github.com/servo/servo/blob/master/components/compositing/compositor.rs) also uses
  4 spaces. Use 4 spaces.
- Bindings are required to be initialized with a value before you're allowed to use them.
- Has block scope within braces `{}`
- You must declare the types of all function arguments.
- All primitive types implement the `Copy` trait.