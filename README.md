# clsx

> A tiny utility for conditionally constructing space-separated class strings in **Rust**,  
> inspired by [lukeed/clsx](https://github.com/lukeed/clsx).

[![Crates.io](https://img.shields.io/crates/v/clsx?color=orange)](https://crates.io/crates/clsx)
[![docs.rs](https://img.shields.io/docsrs/clsx?color=blue)](https://docs.rs/clsx)
[![License](https://img.shields.io/crates/l/clsx)](#license)

A single-pass, flexible, and highly efficient library for assembling “className” strings in Rust.

## Overview

- **Single pass** – Minimizes allocations by appending directly to a single `String`.
- **Flexible** – Supports:
    - Strings, numbers, booleans
    - Arrays/slices, vectors, options
    - Hash maps (`HashMap<String, bool>`) to mimic JS object style
    - Tuples `(bool, &str)` or `(bool, String)` for conditional insertion
    - Closures that return classable arguments
- **Stable Rust** – No nightly features required.

<details>
<summary><strong>Table of Contents</strong></summary>

- [Installation](#installation)
- [Usage](#usage)
- [API](#api)
- [Modes](#modes)
- [Benchmarks](#benchmarks)
- [Support](#support)
- [Tailwind Support](#tailwind-support)
- [Related](#related)
- [License](#license)

</details>

## Installation

Add `clsx` to your `Cargo.toml`:

```toml
[dependencies]
clsx = "0.1.0"
```

Then, inside your Rust code:

```rust
use clsx::clsx;
```

## Usage

```rust
use clsx::clsx;

fn main() {
    // Strings (variadic)
    let classes = clsx!("foo", true && "bar", "baz");
    assert_eq!(classes, "foo bar baz");

    // Conditional tuples
    let is_active = true;
    let is_disabled = false;
    let classes = clsx!("btn", (is_active, "btn-active"), (is_disabled, "btn-disabled"));
    assert_eq!(classes, "btn btn-active");

    // HashMap
    use std::collections::HashMap;
    let mut map = HashMap::new();
    map.insert("flex".to_string(), true);
    map.insert("hidden".to_string(), false);
    let classes = clsx!(map, "base");
    assert_eq!(classes, "flex base");

    // Arrays and nesting
    let classes = clsx!(["hello", "world"], (true, "test"), ["nested", "ok"]);
    assert_eq!(classes, "hello world test nested ok");
}
```

## API

### `clsx!( ...args ) -> String`

The macro can accept any number of arguments. Each argument may be one of:

- A **String** or `&str` (appended if non-empty).
- A **boolean** (ignored when standalone).
- A **numeric** type (converted to string).
- An **array**, **slice**, or **vector** of other `ClsxArg` implementors (flattened).
- A **hash map** (`HashMap<String, bool>`) – Only keys whose value is `true` are appended.
- A **tuple** of `(bool, &str)` or `(bool, String)` – appended only if the bool is `true`.
- A **closure** returning an implementor of `ClsxArg` – evaluated at runtime, appended if non-empty.

> _Any_ falsey or empty values are automatically discarded.

```rust
clsx!(true, false, "", None, Some("valid"));
//=> "valid"
```

## Modes

Unlike the JavaScript version, we currently do **not** ship separate “lite” or “UMD” builds for Rust.  
The single `clsx` crate includes all functionality, and you can simply avoid passing non-string values if you want
“lite-like” usage.

## Benchmarks

We do not bundle cross-browser JavaScript benchmarks, but this Rust library is extremely lightweight and uses:

- A **single pass** over all arguments
- **Pre-allocated** string capacity (roughly 8 bytes * number_of_arguments)
- **`std::fmt::Write`** for numeric conversions to avoid small temporary allocations

If you’d like to measure performance for your specific case, consider adding a `#[bench]` or Criterion-based benchmark
to your own codebase.

## Support

All stable releases of Rust (1.60+) are supported. This library does not require any feature flags or nightly
compiler.  
If you encounter any issues in earlier compilers, please file an issue.

## Tailwind Support

Using `clsx` with [Tailwind CSS](https://tailwindcss.com/) is straightforward. Pass your Tailwind classes into the macro
like any other strings:

```rust
// tailwind usage
let is_primary = true;
let classes = clsx!("text-base", (is_primary, "text-primary"), "bg-gray-50");
```

For advanced autocompletion or specialized tooling, you may need an editor extension that recognizes `clsx!(...)` usage.
Some solutions like the JS/TS Tailwind IntelliSense do not apply directly to Rust, though you might
find [Tailwind Language Server](https://marketplace.visualstudio.com/items?itemName=tailwindlabs.tailwindcss-intellisense)
helpful in certain contexts.

## Related

- [rclassnames](https://crates.io/crates/rclassnames) – Another Rust library with a similar approach
- [lukeed/clsx](https://github.com/lukeed/clsx) – The original JavaScript library
- [obj-str](https://github.com/lukeed/obj-str) – Smaller JS utility for object-based class composition

## License

MIT © (your name or org)

This library is a Rust port inspired by the original [clsx](https://github.com/lukeed/clsx) by Luke Edwards.
