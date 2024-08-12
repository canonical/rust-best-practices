# Rust best practices
This is a list of best-practices which originate from discussions with both our CTO and the company’s other tech leads. All points listed here must be strongly considered before merging code at Canonical. Individually, the things written here may seem unimportant or even trivial, but each of these is a crucial building block for writing good, clean code. You wouldn’t want to hire a builder who has a reputation for installing rotten beams.

Ideally, this document would be a spec for a new AI linter but at present we must rely on the diligence of humans. Therefore, this document should be considered as a style guide for ‘Canonical Rust,’ and is intended to complement Rust for Rustaceans and well thought-out, consistent API design.

If you spot any of the problems detailed here in our existing code-bases or patches, don’t be afraid to fix them—a good codebase is inherently more maintainable and will cause fewer headaches and annoyances later. Remember that all code should aim to be locally-consistent—new code shouldn’t stick out like a sore thumb. Note also that the perfect is the enemy of the good—sole focus on fine-tuning code at the cost of all forward progress doesn’t keep the lights on.

_Disclaimer: this is not a complete list, more items can and likely will be added in future._

This document covers:
<!-- TOC HERE -->

# Preconditions
All new code should abide by cargo fmt, cargo clippy, and cargo clippy --tests. If your crate uses features, be careful to ensure that clippy is definitely being run on your code.

# Naming discipline

## Pattern match variable naming

To reduce cognitive load, pattern-matched variables should be named consistently, that is, the new variable must be either:
- The same as the variable/field it comes from
- The first letter of the variable/field it comes from

When matching structs and struct-like enum variants, try to use the original field names.

✅ Do this:
```rust
if let Some(response) = response { ... }
if let Some(response) = event.response { ... }
if let Some(event_response) = event.response { ... } // E.g. to avoid shadowing.

let Self { name, path } = self;

enum State {
    Reading(fs::File),
    Evaluating {
        workload: Workload,
        ...
    }
}
match state {
    State::Reading(file) => {...}
    State::Evaluating{ workload, .. } => {...}
}
```

⚠️ Avoid this:
```rust
if let Some(r) = response { ... }
if let Some(r) = event.response { ... }
if let Some(er) = event.response { ... }

let Self { name: some_name, path: name } = self;

match state {
    State::Reading(data_source) => {...}
    State::Evaluating{ workload: to_eval, .. } => {...}
}
```

## Generic type parameter naming

To avoid them being mistaken for concrete types, generic type parameters should have single-letter names.

## Lifetime parameter naming

Lifetime parameters give a unique opportunity to link together different parts of an API in the mind of a user.
If the same data is used in multiple places in an API, make sure the lifetime names match.

Lifetime names should be derived from the reference they represent, not from the type they are passed to.
Further, they should consist of either a single letter, short word or extremely well understood acronym.
Numbers should not be used in lifetime names.

Single-letter lifetime names are acceptable if a structure is expected to be used very many times (e.g. a script interface may make heavy use of some `Value<’h>` which contains a reference to a heap upon which it is allocated).
NB: the compiler will occasionally recommend the use of a named lifetime `’a` as it lacks wider context information; `’a` is almost always a bad name.

Lifetime parameters should aim to be concise without losing meaning.
Given the difficulty new users face when understanding lifetimes in an interface, try to give them a hand by being explicit.
Single-letter lifetime names should generally be avoided.

✅ Do this:

```rust
struct QueryMatch<'cursor, 'tree> { .. }

struct Value<'h> { .. }
```

⚠️ Avoid this:

```rust
struct QueryMatch<'a, 'b> { .. }

struct Value<'value> { .. }
```

# Exhaustively match to draw attention

Pattern matching is an excellent way to ensure that all items of data in internal structures have been considered, not only by the author of the current change, but also by the authors of any future changes. When using internal interfaces, always consider using pattern-matching to deliberately create compiler errors and thus draw attention.

✅ Do this:

```rust
impl Ord for MyStruct {
    fn cmp(&self, other: &Self) -> Ordering {
        let Self {
            my,
            thing,
            with,
            some,
            unused: _,
            fields: _,
        } = self;
        (my, thing, with, some)
            .cmp(&(other.my, other.thing, other.with, other.some))
    }
}
```

⚠️ Avoid this:

```rust
impl Ord for MyStruct {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.my, self.thing, self.with, self.some)
            .cmp(&(other.my, other.type, other.with, other.some))
    }
}
```

## Avoid numeric tuple-indexing
Although sometimes a handy shorthand, indexing tuples with .0, .1 etc. deprives us of the opportunity to insert a good name in the same way that field-access on a struct would. Instead, prefer to use pattern-matching to better document the data currently being handled.

✅ Do this:

```rust
fn line_through(point1: (f64, f64), point2: (f64, f64)) -> Line {
	let (x1, y1) = point1;
	let (x2, y2) = point2;
	let gradient = (y2 - y1) / (x2 - x1);
	let y_intercept = y1 - gradient * x1;
	Line {
		gradient,
		y_intercept,
	}
}
```

⚠️ Avoid this:

```rust
fn line_through(point1: (f64, f64), point2: (f64, f64)) -> Line {
	let gradient = (point2.1 - point1.1) / (point2.0 - point1.0);
	let y_intercept = point1.1 - gradient * point1.0;
	Line {
		gradient,
		y_intercept,
	}
}
```

# Error discipline

# Code discipline

## Don’t import all

In general, do not import `*` from a crate. Consider a source file which does this twice from two different dependencies, making use of items from each. Now, consider what happens when these crates are updated and some of these items are removed—the compiler will complain of undefined symbols, but will have absolutely no idea where these came from. Or worse, as the global namespace is used, updates can now cause name-clashes! By sticking to explicit imports only, we help ourselves and our future maintainers.

A corollary of this is that preludes, regardless of their initial convenience, should not be used by us in our own code. Nevertheless, they remain a handy tool for others to use when prototyping, so we should still consider exposing them where appropriate.

The only exception to these rules is that in the context of a unit test module, inserting use super::* is acceptable as it is a well-established and clear convenience.

The rule around enums is slightly different. Here, it is acceptable to import * to bring all items of an enum into scope. However, this should not be done at the top level, only locally to improve the readability of long match statements. There, they should be placed as close as possible to the relevant match, preferably on the line immediately preceding it.

✅ Do this:

```rust
use some_crate::{SpecificItem1, SpecificItem2};
use some_other_crate::SpecificItem3;

...

fn some_fn(some_enum: SomeEnum) -> {
    ...

    use SomeEnum::*;
        Variant2 => {...},
    }
}
```

⚠️ Avoid this:

```rust
use some_crate::*;
use some_other_crate::prelude::*;
use another_crate::SomeEnum::*;

...

fn some_fn(some_enum: SomeEnum) -> {
    ...

    match {
        Variant1 => {...},
        Variant2 => {...},
    }
}
```

## Import grouping

At the time of writing, stable `rustfmt` does not yet express an opinion on the order of imports, hence for now we must do this ourselves.
To clearly delimit whether an import is from the standard library, from a third party library or our own work, these imports should be split into four blocks as follows:

- `std`, `core`, `alloc`
- Third party crates
- `self`, `super`, `crate`

Note that this order follows the unstabilised `import_group = "StdExternCrate"` option.

✅ Do this:

```rust
use std::path::PathBuf;

use camino::Utf8PathBuf;
use tokio::runtime::Runtime;

use crate::{Error, Result};
```

⚠️ Avoid this:

```rust
use camino::Utf8PathBuf;
use crate::{Error, Result};
use std::path::PathBuf;
use tokio::runtime::Runtime;
```

## Import form

Excessive repetition harms readability.
For this reason, avoid Java-style imports where every single imported item gets its own line with a complete and exhaustive path and instead use nesting.
If two `use` lines share the same prefix, they must be merged.

✅ Do this:

```rust
use allocative::Allocative;
use derive_more::Display;
use starlark::{
    environment::{FrozenModule, Module},
    eval::Evaluator,
    values::{AllocValue, Freeze, ProvidesStaticType, StarlarkValue, ValueLike},
};
use starlark_derive::{starlark_value, NoSerialize, Trace};
```

⚠️ Avoid this:

```rust
use allocative::Allocative;
use derive_more::Display;
use starlark::environment::FrozenModule;
use starlark::environment::Module;
use starlark::eval::Evaluator;
use starlark::values::AllocValue;
use starlark::values::Freeze;
use starlark::values::ProvidesStaticType;
use starlark::values::StarlarkValue;
use starlark::values::ValueLike;
use starlark_derive::starlark_value;
use starlark_derive::NoSerialize;
use starlark_derive::Trace;
```

## Import self explicitly

When importing from a child module `foo`, always `use self::foo` as this avoids future name-clashes with a dependency called `foo`.
The same should be used when using a child declaration without `use`, i.e. `let _ = self::child::some_function()`;

✅ Do this:

```rust
mod foo;
mod bar;

pub use self::foo::Foo;
pub use self::bar::Bar;
```

⚠️ Avoid this:

```rust
mod foo;
mod bar;

pub use foo::Foo;
pub use bar::Bar;
```

## Associated type value construction

Traits often include type fields as part of the interface they describe. These types may be referred to with Self::AssociatedType.  Do not use these to construct values as it prevents the next reader from understanding which type is in use and which fields and methods are available. Use these Self::* types to define interfaces, but use concrete types to define implementations.

The only exception is for trait items which return a Result<_, Self::Err>, where Err is set to the crate’s Error type. In this case, it is okay to use the crate’s Result type alias instead.

✅ Do this:

```rust
impl Responder for MyType {
    type Response = SomeStruct;
    type Err = Error;

    fn respond(&self, _input: Input) -> Result<Self::Response> {
        Ok(SomeStruct{
            some: ...,
            fields: ...,
        })
    }
}
```

⚠️ Avoid this:

```rust
impl Responder for MyType {
    type Response = SomeStruct;
    type Err = Error;

    fn respond(&self, _input: I) -> Result<SomeStruct, Error> {
        Ok(Self::Response {
            some: ...,
            fields: ...,
        })
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

Put all derive items in a single `#[derive]`, the formatter will introduce line-breaks as it deems fit.
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

# Function discipline

## No-information returns

Any expression or statement which returns nothing (i.e. `()` such as `println!`) or never return (i.e. `!` such as `std::process::exit`) should end with a semicolon.

In the case of `()`, any block which ends with a function call which returns `()` relies on the return type of that function to never be changed to return something more useful.
This is a strange dependency which may cause needless compiler errors in future, hence is best avoided.
Using an explicit `;` reinforces the fact that we expect to obtain no information from a particular call.

As the never type, `!` also denotes that no information is returned, we should also use an explicit semicolon.

If a `match` statement is expected to return `()`, then it is being used as a control-flow structure.
Therefore, do-nothing `match` branches should be written as `{}` rather than `()`.

✅ Do this:

```rust
fn setup_foo(&self) {
    match self.foo_type {
        FooType::A => ()
        ...
    }
    self.setup_bar();
}
```

⚠️ Avoid this:

```rust
fn setup_foo(&self) {
    match self.foo_type {
        FooType::A => {}
        ...
    }
    self.setup_bar()
}
```

## Comment discipline

<!-- ## Further reading -->
<!-- Check out  Effective Go and  Google’s style guidelines. If you find yourself in a situation which isn’t covered by this document, refer to these. If you feel this situation is common-enough, message Ed Jones to discuss adding it to this document. -->

# (Admin, to be removed once stable)

| who’s | done | what |
|-------|------|------|
| Ed    | yes  | all function calls which return () should end with ; |
| Ed    | yes  | all explicit return statements must end with ; |
| Ed    | yes  | if let Some(x) = x (pattern matched must be the same as the value being matched, or share the first letter, same true for match) |
| Ed    | yes  | impl ordering (unsafe then impl StandardTrait for T, impl NonStandardTrait for T, more general first |
| Ed    | yes  | derive ordering: Clone then Copy then standard (what order?) then non-standard in alphabetical order (or dependency order?) |
| Ed    | no   | Semantic newlines |
| Ed    | yes  | Forbid struct construction with Self::XXX {...}, for associated types, use the type name explicitly. NB: this is not enums! |
| Ed    | yes  | Forbid use XXX::* unless XXX is an enum, and even then, this must be local (definitely do this if rustfmt would start awkwardly wrapping?) |
| Ed    | yes  | Forbid use of use super::... except for use super::* in unit tests |
| Ed    | no   | Struct population must not mix computed and assigned/renamed fields |
| Ed    | no   | Use a consistent order for naming |
| Ed    | no   | thiserror message forms |
| Ed    | no   | panic!() message forms |
| Ed    | no   | .expect() message forms |
| Ed    | no   | One-block declarations: consts then statics then lazy_statics (?) then lets. |
| Ed    | no   | Unsafe functions / functions containing unsafe must list preconditions (to what extent does clippy help with this?) |
| Ed    | yes  | Generic type parameters must be one letter |
| Ed    | yes  | Generic lifetime names must not contain underscores |
| Ed    | yes  | Generic lifetime names must have a meaningful name if one exists (if long-lived?) |
| Ed    | no   | Function doc’s first sentence must be no more than 160 chars (wrapped) |
| Ed    | no   | Comments must be no wider than 80 characters from their leftmost point |
| Ed    | yes  | impl LocalTrait for ForeignType order: std first, then non-std but popular, then others in a reasonable order |
| Ed    | no   | Use Self as reasonably possible (in function return types, value construction Self{}/Self()/Self) |
| Ed    | no   | No use of declarations in format-like macros (the key=value part at the end), prefer local lets |
| Ed    | no   | Use `.map(\|_\| ())` rather than `.map(drop)` |
| Ed    | no   | If a where clause exists, all type bounds must be within it (no mixing) |
| Ed    | no   | Clippy lints to turn on: no broken links, |
| Ed    | no   | Prefer scoping to drop (avoid all drop calls?) |
| Ed    | no   | Always use clap-derive not the old API |
| Ed    | no   | Prefer iterator combinators if infallible, consider avoiding if fallible |
| Ed    | no   | No method calls on struct literals |
| Ed    | no   | Scoped mutability—names must match: let x = { let mut x = …; …; x} |
| Ed    | no   | Prefer Foo::new() or equivalent to Foo{} |
| Ed    | no   | All imports must be in one block (until rustfmt stabilises its many sorted blocks) |
| Ed    | yes   | Imports must be nested (no Java-style repetition) |
| Ed    | no   | Error type must be defined in lib.rs or error.rs in crate root |
| Ed    | no   | Always declare Result<T>, this must be done immediately below the error type |
| Ed    | no   | Don’t use mod.rs, use foo.rs next to foo/ to help code editors! |
| Ed    | no   | Constructing boxed errors? |
| Ed    | no   | When using builders, provide a MyType::builder() -> MyTypeBuilder? |
| Ed    | no   | MyTypeBuilder must have a .build() -> MyType (duh) |
| Ed    | no   | Prefer builders by value? (avoid awkwardly relying on clone() calls getting optimised away) |
| Ed    | no   | Use explicit Ok(()) explicitly rather than Ok(returns_unit()) (split into two lines) |
| Ed    | no   | No method calls on (curly brace?) macro results? |
| Ed    | no   | Prefer explicit matches (avoid catch-all _ => () cases) |
| Ed    | no   | Start by defining the error states (the error path must be first-class, even if not optimised) |
| Ed    | no   | Leverage the type system to uphold invariants and prevent misuse |
| Ed    | no   | Minimise use of unsafe and scope as closely as possible to where you actually need unsafe |
| Ed    | no   | Use rustfmt, but use THESE SPECIFIC (insert here) settings, do not use #[rustfmt::skip]! |
| Ed    | no   | Prefer let xxx: Foo to turbofish to add type annotations on the last thing in a chain (unless really annoying!) i.e. prefer to signpost what the goal of a large chain is, also can be slightly more reliable in some cases |
| Ed    | no   | For locally-written Serde, use an explicit return and put the helper types at the end of the function, don’t write macro helpers for the helper types! (Type names don’t need to be crate-level unique, just distinct enough but within reason e.g. no type shadowing)
| Ed    | no   | Shadowing: two levels of variable shadowing okay, type shadowing is never okay! |
| Ed    | no   | In mod.rs equivalent, put all mods first, then all pub uses then pub(crate) uses all uses in separate blocks (grouped nicely) then #[cfg(..)] after all |
| Ed    | no   | All use statements must come before all code |
| Ed    | no   | When read from top-to-bottom, try to make this feel like a tour of the API (guide the reader) |
| Ed    | no   | Redundant type annotations—does clippy prevent let foo: Foo = (...).collect::<Foo>();? |
| Ed    | no   | No .map(Self) at the end of a chain (e.g. to construct Result<Self> |
| Ed    | no   | No let x = &expr unless indexing/slicing |
| Ed    | no   | Use `vec![]` rather than Vec::new(), use Foo::new() rather than Foo::with_capacity(0) (for which Foo? yes: Vec, HashMap, BTreeMap) |
| Ed    | no   | Don’t pattern-match on pointer parameters i.e. don’t `.map(\|&x\| x)`, use `.map(\|x\| *x)` |
| Ed    | no   | Don’t populate tuples with lots of computation. If the line breaks, put things in variables |
| Ed    | no   | Don’t pattern-match in fn functions (like one would expect in closures) |
| Ed    | no   | Put pub fields first in mixed structs. If this breaks Ord, write it manually |
| Ed    | no   | Prefer pattern matching to field-access where all fields should be considered |
| Ed    | no   | Don’t assign or compare &lit (e.g. &0, &""), put the & as close to where needed as possible and deref instead |
| Ed    | no   | In tests, #[test] (or equivalent) should be the final attribute |
| Ed    | no   | If hex is used, make it lowercase |
| Ed    | no   | All public top-level items must be documented |
| Ed    | no   | Put raw API-data unpacking structs at the end of the function they’re needed in after // serde structs |
| Ed    | no   | Put test helper structs at the end of the function they’re used in, after // test structs |
| Ed    | no   | Prefer to call API response structs Response (no shadowing due to independent scopes) |
| Ed    | no   | Unpack to ensure all fields are considered |
| Ed    | no   | advice use of `.ok().unwrap_or_else(\|\|...)` if `.unwrap_or_else(\|_\| …)` is seen |
| Ed    | no   | which files #![...] annotations are placed in |
| Ed    | no   | unused trait method parameters should use let _ = param rather than named _foo or annotations |
| Ed    | no   | format! parameters which consist of a single identifier (no pathing) should be folded into the format string |
| Ed    | no   | Builder creation: prefer Foo::builder() to FooBuilder::new() (also have no public ::new() on the builder) |
| Ed    | no   | Local helper types should be put behind a comment at the end, after an explicit return? |
| Ed    | no   | Prefer `.collect()` to `Foo::from_iter` |
<!-- TODO(kcza): function dependencies AFTER -->
