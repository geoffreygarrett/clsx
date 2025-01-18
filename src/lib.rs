/// # clsx
///
/// A tiny, single-pass utility for conditionally constructing a space-separated class string in Rust,
/// inspired by the original JavaScript [`clsx`](https://github.com/lukeed/clsx) library.
///
/// ## Key Features
///
/// - **Single-pass** string concatenation to avoid excessive allocations.
/// - **Supports** strings, numbers, arrays/slices, vectors, options, hashmaps, boolean-string tuples, closures, etc.
/// - **Ergonomic** macro `clsx!` that flattens its arguments and conditionally includes classes.
/// - **Optimized** with a rough capacity guess (~8 chars per argument) to reduce re-allocation overhead.
/// - **Tested** extensively for various types and usage patterns.
///
/// ## Usage Example
///
/// ```rust
/// use clsx::clsx;
///
/// fn main() {
///     let is_active = true;
///     let classes = clsx!("btn", (is_active, "btn-active"), "p-4");
///     assert_eq!(classes, "btn btn-active p-4");
///
///     // Flatten arrays, slices, nested structures:
///     let nested = clsx!(["foo", "bar"], (true, "extra"));
///     assert_eq!(nested, "foo bar extra");
/// }
/// ```
///
/// # Installation
///
/// This library can be included in your Rust project by adding the following to your `Cargo.toml`:
///
/// ```toml
/// [dependencies]
/// clsx = { git = "https://github.com/YOUR_USERNAME/clsx-rs.git", branch = "main" }
/// ```
///
/// *Note: Replace the git URL with the actual repository location.*
///
/// # Parity with JS clsx
///
/// This Rust port offers feature parity with the core ideas of [lukeed/clsx](https://github.com/lukeed/clsx):
/// - **Flattening** of nested arrays and objects/hashmaps.
/// - **Conditional** inclusion via `(bool, &str)` or `(bool, String)`.
/// - **Discarding** of `false` or empty values.
/// - **Support** for closures that return classable values at runtime.
///
/// Differences include type‐safe expansions to Rust data structures like `Option`, numeric types,
/// and seamless addition of custom behavior by implementing `ClsxArg`.
///
/// # Examples
///
/// - Booleans or empty strings are ignored:
///
///   ```rust
///   # use clsx::clsx;
///   let classes = clsx!("btn", false, "", "enabled");
///   assert_eq!(classes, "btn enabled");
///   ```
///
/// - HashMap interpreted like JS objects:
///
///   ```rust
///   # use clsx::clsx;
///   # use std::collections::HashMap;
///   let mut map = HashMap::new();
///   map.insert("visible".to_string(), true);
///   map.insert("hidden".to_string(), false);
///   let classes = clsx!(map, "base");
///   assert_eq!(classes, "visible base");
///   ```
///
/// # License
///
/// Licensed under the MIT license, as per the original [clsx](https://github.com/lukeed/clsx).
///

mod clsx;

pub use clsx::*;