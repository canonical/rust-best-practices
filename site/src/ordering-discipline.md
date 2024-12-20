# Ordering discipline

## General definition ordering

When read from top to bottom, a file should feel like a tour of the APIs it defines.
The most important items should be defined further up the file, with their helpers below.
No `impl` block should come before the type or trait to which it relates.
In this way, lower-level implementation details are hidden from the reader until they wish to know more, at which point, having gained a good knowledge of the overall form of the code, they can read on to understand how it functions.

✅ Do this:

```rust
{{#include snippet_helpers/ordering_discipline.rs}}
# type Foo = Arbitrary;
impl Foo {
    pub fn some_func(&self) {
        self.some_helper_func();
    }

    fn some_helper_func(&self) {
        // ...
    }
}
```

⚠️ Avoid this:

```rust
{{#include snippet_helpers/ordering_discipline.rs}}
# type Foo = Arbitrary;
impl Foo {
    fn some_helper_func(&self) {
        // ...
    }

    pub fn some_func(&self) {
        self.some_helper_func();
    }
}
```

## Impl block placement

By default, `impl SomeType` blocks for a given type should be in the same file, immediately below where that type is defined.
Trait implementations `impl SomeTrait for SomeType` should go in the file where `SomeTrait` or `SomeType` is defined.
This is effectively the orphan rule but applied within crates.

## Impl block ordering

The `impl` blocks in the same file for `MyType` should be ordered as follows:

- `impl MyType`
- `unsafe impl StandardTrait for MyType`
- `unsafe impl MyTrait for MyType`
- `unsafe impl ThirdPartyTrait for MyType`
- `impl StandardTrait for MyType`
- `impl MyTrait for MyType`
- `impl ThirdPartyTrait for MyType`

The `impl` blocks in the same file for `MyTrait` should be ordered as follows:

- `impl MyTrait for StandardType`
- `impl MyTrait for MyType`
- `impl MyTrait for ThirdPartyType`

## Derive ordering

Put all derive items in a single `#[derive(...)]` (the formatter will preserve readability by introducing line-breaks as it deems fit).
Derived items should be ordered as follows:

- `Copy`
- `Clone`
- `Debug`
- `PartialEq`
- `Eq`
- `PartialOrd`
- `Ord`
- Other standard traits, ordered lexicographically
- Third party traits, ordered lexicographically

## Declaration ordering

Rust provides several different types of declaration and where these are declared consecutively in the same block, they should be ordered for visual consistency.
This will help draw the reader’s eye to the important parts of each declaration, rather than getting lost in some superfluous ordering.

Declarations should be ordered as follows:

- `const`
- `static`
- `lazy_static!`
- `let`
- `let mut`

## Struct field ordering

The more public a field, the more likely a user will to want to know more about it and understand it.
Therefore, we should put the items they are most likely to care about nearer the top of our code, avoiding them having to skip over parts uninteresting to them.
Specifically, this means that we should place:

- `pub` fields first,
- `pub(crate)` fields next,
- private fields last.

Structs neatly organised in this way make it clear when the reader has entered implementation details and hence when they are less likely to glean useful information.

If the reason for ordering fields in a different way to the above is due to a derivation such as `Ord` or `PartialOrd`, this is not a good reason for deviation from the norm.
Maintaining consistency is of a higher priority than a single derivation, hence the relevant implementations should be written out by hand.
The reader is more likely to be looking at the entire struct rather than just one trait implementation.

✅ Do this:

```rust
# use std::collections::BTreeMap;
# type StackFrame = ();
# type Value<'h> = &'h ();
struct ScriptExecutionContext<'h, T> {
    pub user_data: T,

    pub(crate) global_vars: BTreeMap<String, Value<'h>>,

    stack: Vec<StackFrame>,
    max_steps: Option<u64>,
    steps: u64,
}
```

⚠️ Avoid this:

```rust
# use std::collections::BTreeMap;
# type StackFrame = ();
# type Value<'h> = &'h ();
struct ScriptExecutionContext<'h, T> {
    stack: Vec<StackFrame>,
    pub(crate) global_vars: BTreeMap<String, Value<'h>>,
    pub user_data: T,
    steps: u64,
    max_steps: Option<u64>,
}
```
