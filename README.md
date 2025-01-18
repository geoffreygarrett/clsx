# clsx

> A tiny utility for conditionally constructing space-separated class strings in **Rust**,  
> inspired by [lukeed/clsx](https://github.com/lukeed/clsx).

[![Crates.io](https://img.shields.io/crates/v/clsx?color=orange)](https://crates.io/crates/clsx)
[![docs.rs](https://img.shields.io/docsrs/clsx?color=blue)](https://docs.rs/clsx)
[![License](https://img.shields.io/crates/l/clsx)](#license)

A **single-pass**, flexible, and highly efficient library for assembling “className” strings in Rust.

## Overview

- **Single pass** – Minimizes allocations by appending directly to a single `String`.
- **Flexible** – Supports:
    - Strings, numbers, booleans
    - Arrays/slices, vectors, options
    - Hash maps (`HashMap<String, bool>`) to mimic JS object usage
    - Tuples `(bool, &str)` or `(bool, String)` for conditional insertion
    - Closures returning something that also implements `ClsxArg`
- **Stable Rust** – No nightly features required.

<details>
<summary><strong>Table of Contents</strong></summary>

- [Installation](#installation)
- [Usage](#usage)
- [API](#api)
- [Advanced Usage](#advanced-usage)
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

Then in your code:

```rust
use clsx::clsx;
```

## Usage

```rust
use clsx::clsx;

fn main() {
    // 1) Basic strings
    let classes = clsx!("foo", "bar", "baz");
    assert_eq!(classes, "foo bar baz");

    // 2) Conditional tuples
    let is_active = true;
    let is_disabled = false;
    let classes = clsx!(
        "btn",
        (is_active, "btn-active"),
        (is_disabled, "btn-disabled")
    );
    // => "btn btn-active"
    assert_eq!(classes, "btn btn-active");

    // 3) HashMap usage
    use std::collections::HashMap;
    let mut map = HashMap::new();
    map.insert("flex".to_string(), true);
    map.insert("hidden".to_string(), false);
    let classes = clsx!(map, "base");
    assert_eq!(classes, "flex base");

    // 4) Arrays & flattening
    let classes = clsx!(
        ["hello", "world"],
        (true, "testing"),
        ["nested", "classes"]
    );
    assert_eq!(classes, "hello world testing nested classes");

    // 5) Options
    let maybe_active: Option<&str> = Some("active");
    let none_str: Option<&str> = None;
    let classes = clsx!("btn", maybe_active, none_str, "p-4");
    // => "btn active p-4"
    assert_eq!(classes, "btn active p-4");

    // 6) Numbers
    let i = 10;
    let f = 3.14;
    let classes = clsx!("start", i, f, "end");
    // => "start 10 3.14 end"
    assert_eq!(classes, "start 10 3.14 end");
}
```

## API

### `clsx!(...args) -> String`

**Flatten** and **conditionally** assemble space-separated classes from any combination of arguments:

| Type(s)                   | Behavior                                                                         |
|---------------------------|----------------------------------------------------------------------------------|
| `&str`, `String`          | Appends if non-empty                                                             |
| **booleans**              | Ignored (no classes appended)                                                    |
| **numeric types**         | Appended as string (`10`, `3.14`, etc.)                                          |
| **arrays/slices/vectors** | Flatten each item (recursively)                                                  |
| **hash maps**             | (`HashMap<String, bool>`) – Only the keys whose value is `true` get appended     |
| **tuples**                | `(bool, &str)` or `(bool, String)` => appended only if the bool is `true`        |
| **option**                | `Option<T>` => appended if `Some`, ignored if `None`                             |
| **closures**              | Called, and the result is appended if it’s non-empty or meets the above criteria |

> **Note:** Any empty strings, false booleans, or “falsey” checks automatically discard the class from the output.

## Advanced Usage

- **Inline if Expressions**  
  Instead of `(bool, &str)`, you can inline `if your_bool { "class" } else { "" }`:
  ```rust,ignore
  let show_extra = false;
  let classes = clsx!("core", if show_extra { "extra" } else { "" }, "final");
  // => "core final"
  ```
- **Closures** returning `Option<T>`:
  ```rust,ignore
  let maybe = || -> Option<&'static str> { Some("maybe-yes") };
  let never = || -> Option<&'static str> { None };
  let output = clsx!("start", maybe, never, "end");
  // => "start maybe-yes end"
  ```

## Benchmarks

We don’t provide cross-browser or JS benchmarks, but this crate is quite small and uses:

- A **single pass** over arguments
- **Pre-allocated** `String` capacity (approx. `8 * number_of_arguments` bytes)
- `std::fmt::Write` for numeric conversions to avoid small temporary buffers

For detailed performance measurements, add your own [Criterion](https://crates.io/crates/criterion) or custom
benchmarks.

## Support

All stable releases of Rust (>=1.60) are supported. If you find any compatibility issues, please file
an [issue](https://github.com/yourUsername/clsx-rs/issues).

## Tailwind Support

Use `clsx` with [Tailwind CSS](https://tailwindcss.com/) by simply providing the Tailwind classes. For advanced
autocompletion, your IDE needs to recognize `clsx!(...)` usage. The standard JS/TS IntelliSense extension might not work
in Rust, but you may
try [Tailwind Language Server](https://marketplace.visualstudio.com/items?itemName=tailwindlabs.tailwindcss-intellisense)
or a Rust-specific extension.

## Related

- [rclassnames](https://crates.io/crates/rclassnames) – Another Rust library with a similar approach
- [lukeed/clsx](https://github.com/lukeed/clsx) – The original JavaScript library
- [obj-str](https://github.com/lukeed/obj-str) – Smaller JS utility for object-based class composition

## License

MIT © [RustForWeb](https://github.com/RustForWeb/)
This library is a Rust port inspired by the original [clsx](https://github.com/lukeed/clsx) by Luke Edwards.
