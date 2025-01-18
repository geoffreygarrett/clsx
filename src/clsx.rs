use std::collections::HashMap;

/* -------------------------------------------------------------------------------------------------
 * Internal Macro: __clsx_count_args
 * -----------------------------------------------------------------------------------------------*/

/// Count how many comma-separated arguments appear in a macro invocation.
/// For example: `__clsx_count_args!(a, b, c)` → `3`.
#[doc(hidden)]
#[macro_export]
macro_rules! __clsx_count_args {
    () => (0usize);
    ($head:expr) => (1usize);
    ($head:expr, $($tail:expr),*) => (1usize + $crate::__clsx_count_args!($($tail),*));
}

/* -------------------------------------------------------------------------------------------------
 * Trait: ClsxArg
 * -----------------------------------------------------------------------------------------------*/

/// A trait representing any type that can be converted into one or more class names.
///
/// Implementors of this trait should append their class string(s) into the given `out` buffer,
/// optionally adding a leading space if `out` is not empty.
///
/// ## Common Implementors
/// - `&str` / `String` (directly appended)
/// - Booleans (ignored by default)
/// - Numeric types (converted to string)
/// - `Option<T>` (appended if `Some`, ignored if `None`)
/// - Slices/arrays of any `ClsxArg` type
/// - `HashMap<String, bool>` (keys appended if value is `true`)
/// - Tuples like `(bool, &str)` or `(bool, String)` (included only if boolean is `true`)
/// - Closures returning something that implements `ClsxArg`
pub trait ClsxArg {
    /// Appends this argument's class(es) into `out`.
    ///
    /// Implementors should insert a leading space if `out` is non-empty and a valid class
    /// is about to be appended.
    fn append_to(&self, out: &mut String);
}

/* -------------------------------------------------------------------------------------------------
 * Helper: push_with_space_if_needed
 * -----------------------------------------------------------------------------------------------*/

#[inline]
fn push_with_space_if_needed(out: &mut String, val: &str) {
    if !val.is_empty() {
        if !out.is_empty() {
            out.push(' ');
        }
        out.push_str(val);
    }
}

/* -------------------------------------------------------------------------------------------------
 * Implementations for Strings & Str
 * -----------------------------------------------------------------------------------------------*/

impl ClsxArg for &str {
    #[inline]
    fn append_to(&self, out: &mut String) {
        push_with_space_if_needed(out, self);
    }
}

impl ClsxArg for String {
    #[inline]
    fn append_to(&self, out: &mut String) {
        push_with_space_if_needed(out, self);
    }
}

impl ClsxArg for &String {
    #[inline]
    fn append_to(&self, out: &mut String) {
        push_with_space_if_needed(out, self);
    }
}

impl ClsxArg for &&str {
    #[inline]
    fn append_to(&self, out: &mut String) {
        push_with_space_if_needed(out, self);
    }
}

impl ClsxArg for &&&str {
    #[inline]
    fn append_to(&self, out: &mut String) {
        push_with_space_if_needed(out, self);
    }
}

/* -------------------------------------------------------------------------------------------------
 * Booleans
 * -----------------------------------------------------------------------------------------------*/

/// Booleans don't contribute anything by default.
impl ClsxArg for bool {
    #[inline]
    fn append_to(&self, _out: &mut String) {
        // no-op
    }
}

/* -------------------------------------------------------------------------------------------------
 * Numeric Types
 * -----------------------------------------------------------------------------------------------*/

macro_rules! impl_number {
    ($($t:ty),+) => {
        $(
            impl ClsxArg for $t {
                #[inline]
                fn append_to(&self, out: &mut String) {
                    if !out.is_empty() {
                        out.push(' ');
                    }
                    use std::fmt::Write;
                    let _ = write!(out, "{}", self);
                }
            }
        )+
    }
}

impl_number!(i8, i16, i32, i64, isize, u8, u16, u32, u64, usize, f32, f64);

/* -------------------------------------------------------------------------------------------------
 * Collections
 * -----------------------------------------------------------------------------------------------*/

impl<T: ClsxArg> ClsxArg for Option<T> {
    #[inline]
    fn append_to(&self, out: &mut String) {
        if let Some(val) = self {
            val.append_to(out);
        }
    }
}

impl<T: ClsxArg> ClsxArg for Vec<T> {
    #[inline]
    fn append_to(&self, out: &mut String) {
        for item in self {
            item.append_to(out);
        }
    }
}

impl<T: ClsxArg> ClsxArg for &[T] {
    #[inline]
    fn append_to(&self, out: &mut String) {
        for item in *self {
            item.append_to(out);
        }
    }
}

impl<T: ClsxArg, const N: usize> ClsxArg for [T; N] {
    #[inline]
    fn append_to(&self, out: &mut String) {
        for item in self {
            item.append_to(out);
        }
    }
}

/* -------------------------------------------------------------------------------------------------
 * HashMap of (String -> bool)
 * -----------------------------------------------------------------------------------------------*/

impl ClsxArg for HashMap<String, bool> {
    #[inline]
    fn append_to(&self, out: &mut String) {
        for (class_name, flag) in self.iter() {
            if *flag && !class_name.is_empty() {
                push_with_space_if_needed(out, class_name);
            }
        }
    }
}

/* -------------------------------------------------------------------------------------------------
 * Tuples: (bool, &str) and (bool, String)
 * -----------------------------------------------------------------------------------------------*/

impl ClsxArg for (bool, &str) {
    #[inline]
    fn append_to(&self, out: &mut String) {
        if self.0 && !self.1.is_empty() {
            push_with_space_if_needed(out, self.1);
        }
    }
}

impl ClsxArg for (bool, String) {
    #[inline]
    fn append_to(&self, out: &mut String) {
        if self.0 && !self.1.is_empty() {
            push_with_space_if_needed(out, &self.1);
        }
    }
}

/* -------------------------------------------------------------------------------------------------
 * Closures returning a ClsxArg
 * -----------------------------------------------------------------------------------------------*/

impl<F, R> ClsxArg for F
where
    F: Fn() -> R,
    R: ClsxArg,
{
    #[inline]
    fn append_to(&self, out: &mut String) {
        (self)().append_to(out);
    }
}

/* -------------------------------------------------------------------------------------------------
 * Macro: clsx!(...)
 * -----------------------------------------------------------------------------------------------*/

/// The `clsx!` macro composes class names in a single pass, appending each
/// argument's class(es) into one final space‐separated `String`.
///
/// This macro also pre‐allocates space for the output `String`, using
/// a rough guess of ~8 characters per argument to reduce reallocation overhead.
///
/// # Examples
///
/// ```rust
/// use clsx::clsx;
///
/// let is_active = true;
/// let classes = clsx!("btn", (is_active, "btn-active"), "p-4");
/// assert_eq!(classes, "btn btn-active p-4");
///
/// // Flatten arrays, slices, etc.:
/// let nested = clsx!(["foo", "bar"], (true, "extra"));
/// assert_eq!(nested, "foo bar extra");
/// ```
#[macro_export]
macro_rules! clsx {
    () => {
        String::new()
    };
    ($($arg:expr),+ $(,)?) => {{
        const __COUNT: usize = $crate::__clsx_count_args!($($arg),*);
        let mut out = String::with_capacity(__COUNT * 8);
        $(
            $crate::ClsxArg::append_to(&$arg, &mut out);
        )+
        out
    }};
}

/* -------------------------------------------------------------------------------------------------
 * Tests
 * -----------------------------------------------------------------------------------------------*/

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    #[test]
    fn test_clsx_basic() {
        let result = clsx!("class1", "class2", "class3");
        assert_eq!(result, "class1 class2 class3");
    }

    #[test]
    fn test_clsx_single_argument() {
        let result = clsx!("solo");
        assert_eq!(result, "solo");
    }

    #[test]
    fn test_clsx_no_arguments() {
        let result = clsx!();
        assert_eq!(result, "");
    }

    #[test]
    fn test_clsx_tuple_true() {
        let is_active = true;
        let classes = clsx!("btn", (is_active, "btn-active"));
        assert_eq!(classes, "btn btn-active");
    }

    #[test]
    fn test_clsx_tuple_false() {
        let is_active = false;
        let classes = clsx!("btn", (is_active, "btn-active"));
        assert_eq!(classes, "btn");
    }

    #[test]
    fn test_clsx_with_conditionals() {
        let is_active = true;
        let is_disabled = false;
        let result = clsx!(
            "btn",
            (is_active, "btn-active"),
            (is_disabled, "btn-disabled"),
        );
        assert_eq!(result, "btn btn-active");
    }

    #[test]
    fn test_clsx_all_false_conditions() {
        let is_active = false;
        let is_disabled = false;
        let result = clsx!(
            "btn",
            (is_active, "btn-active"),
            (is_disabled, "btn-disabled"),
        );
        assert_eq!(result, "btn");
    }

    #[test]
    fn test_clsx_with_options() {
        let some_class: Option<&str> = Some("visible");
        let none_class: Option<&str> = None;
        let result = clsx!(some_class, none_class, "base");
        assert_eq!(result, "visible base");
    }

    #[test]
    fn test_clsx_with_hashmap() {
        let mut map = HashMap::new();
        map.insert("flex".to_string(), true);
        map.insert("hidden".to_string(), false);
        let result = clsx!(map, "base");
        assert_eq!(result, "flex base");
    }

    #[test]
    fn test_clsx_with_closures() {
        let result = clsx!(
            "base",
            || "dynamic".to_string(),
            || if true { "active".to_string() } else { "".to_string() },
        );
        assert_eq!(result, "base dynamic active");
    }

    #[test]
    fn test_clsx_with_closures_false() {
        let flag = false;
        let result = clsx!(
            "something",
            || if flag { "hidden" } else { "" },
        );
        assert_eq!(result, "something");
    }

    #[test]
    fn test_clsx_nested_structures() {
        let array = ["nested1", "nested2"];
        let result = clsx!(
            array,
            (true, "conditional"),
            ["deeply", "nested", "class"]
        );
        assert_eq!(result, "nested1 nested2 conditional deeply nested class");
    }

    #[test]
    fn test_clsx_large_array() {
        let arr = ["one", "two", "three", "four", "five"];
        let result = clsx!(arr, "six");
        assert_eq!(result, "one two three four five six");
    }

    #[test]
    fn test_clsx_with_numerics() {
        let i = 10;
        let f = 3.14;
        let negative = -5i32;
        let result = clsx!("start", i, f, negative, "end");
        assert_eq!(result, "start 10 3.14 -5 end");
    }

    #[test]
    fn test_clsx_with_bools() {
        let result = clsx!("hello", true, false, "world");
        assert_eq!(result, "hello world");
    }
}
