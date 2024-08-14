# Rust best practices
This is a list of best-practices which originate from discussions with both our CTO and the company’s other tech leads. All points listed here must be strongly considered before merging code at Canonical. Individually, the things written here may seem unimportant or even trivial, but each of these is a crucial building block for writing good, clean code. You wouldn’t want to hire a builder who has a reputation for installing rotten beams.

Ideally, this document would be a spec for a new AI linter but at present we must rely on the diligence of humans. Therefore, this document should be considered as a style guide for ‘Canonical Rust,’ and is intended to complement Rust for Rustaceans and well thought-out, consistent API design.

If you spot any of the problems detailed here in our existing code-bases or patches, don’t be afraid to fix them—a good codebase is inherently more maintainable and will cause fewer headaches and annoyances later. Remember that all code should aim to be locally-consistent—new code shouldn’t stick out like a sore thumb. Note also that the perfect is the enemy of the good—sole focus on fine-tuning code at the cost of all forward progress doesn’t keep the lights on.

_Disclaimer: this is not a complete list, more items can and likely will be added in future._

# Table of Contents
<!-- TOC HERE -->

# Preconditions
All new code should abide by cargo fmt, cargo clippy, and cargo clippy --tests. If your crate uses features, be careful to ensure that clippy is definitely being run on your code.

# Cosmetic discipline

## Spacing

Use blank lines semantically, rather than aesthetically. They should be used consistently, regardless of the size of a section of code, to delimit sections of strongly-associated code. There are no hard and fast rules for this strong association, but the following heuristics are quite effective.

- If a variable is declared and only used in the block of code which follows it, that declaration and block are strongly associated. Do not put a blank line here.
- If a variable is used in multiple blocks of code, not just the one which follows it, that declaration is not strongly associated with the block immediately after it. Put a newline here.
- If a variable is declared and then checked, the declaration and check are strongly associated and must not be separated by a blank line. If the check contains more than three lines, the declaration and check start to form their own strongly associated block so require a blank line after.

✅ Do this:
```rust
let x = foo();
if !x.is_valid() {
    return Err(Error::Invalid);
}
println!(“{x}”);

let y = baz();
if !y.is_valid() {
    return Err(Error::Invalid);
}
return Ok(y);
```

⚠️ Avoid this:
```rust
let x = foo();

if !x.is_valid() {
    return Err(Error::Invalid);
}

println!(“{x}”);
let y = baz();
if !y.is_valid() {
    return Err(Error::Invalid);
}

return Ok(y);
```

## Grouping

Don’t interleave unrelated code. Remember, to a new reader, this will look deliberate and they will be confused about how variables relate. Keep it clean and group together strongly intradependent sections of code.

This is particularly significant where closures are used—if a closure is defined half-way through a function, does not capture anything and then is only used at the end, the reader will have to keep many things in mind for no good reason. If values are captured, declare closures close to where they’re needed. If no captures are required, consider defining them at the top of the highest possible scope to make it obvious that no closures are needed. Also, consider whether a closure is required at all—code may feel cleaner with a simpler, more top-to-bottom flow control pattern. Logic should feel clean and be easy to follow.

_The following snippets assume that functions `foo`, `bar` and `baz` are free of side-effects._

✅ Do this:
```rust
let x = foo();
let b = baz();;
if !b.is_valid() {
    return Err(Error::Invalid)
}
let z = x + b;

let y = bar();
if !y.is_valid() {
    return Err(Error::Invalid)
}
```

⚠️ Avoid this:
```rust
let x = foo()
let check = |x| {
    if !x.valid() {
        return Err(Error::Invalid)
    }
    Ok(x)
};
let y = bar();
let z = x + check(baz())?;
check(y)?;
```

# Naming discipline

## Name content

Naming is one of the three hardest problems in programming (along with off-by-one errors). Every variable, every function, every type and every concept requires a good name which fits into a good naming scheme. There is no one optimal way to come up with a good name, however when attempting to do so, the first place to look is for similar names in your project and try to mimic these. This should result in a name which intuitively feels like it belongs among the rest of the code. Even doing this has its pitfalls, so ideally your name should:
- **say what it means**—make the name fit conceptually into the surrounding context. If a reader sees `fn is_in(a: &str, b: &str)`, the order is not as obvious as if they were to see `fn is_in(haystach: &str, needle: &str)`.
- **have a consistent word order**—inconsistency makes an API look dishevelled, unplanned and hence unprofessional. If the rest of the API uses `verb_noun` then unless there is a very good reason not to, the next function should be of the form `verb_noun`.
- **be concise**—a long name can almost always be shortened. More characters implies a need to disambiguate, so if no such need exists, reduce the cognitive load on the next reader by reducing the amount they must read. Of course, don’t take this too far—the next reader must not be expected to look elsewhere to understand the full meaning of a name, as may occur if nonstandard acronyms or abbreviations are used.
- **comprise simple words**—a long word can often be replaced by a shorter one. A concise name will comprise the smallest list of the smallest words which do not lose the subject’s meaning. Remember: [thesaurus.com][thesaurus] and [dictionary.com][dictionary] are your friends!
- **comprise correct words**—if there is any disagreement over the implications of chosen words, then there will be some reader who gets the wrong idea. It’s better to spend more time discussing internally than to confuse a user. (Example: in Starlark, a 20-minute discussion was needed on the choice between `NOT_SAFE` vs `UNSAFE` as an empty value of a set of safety flags, where each flag had a name like `FOO_SAFE`.)
- **be unified**—there should be one and only one name for concepts used. If more are used haphazardly, it implies a difference where there is none and thus muddies the water.
- **avoid including types**—type names should be omitted unless required to discriminate between two variables of different types which roughly hold the same value. Some examples: in a finder function `needleStr` and `haystackStr` can be more concisely expressed as `needle` and `haystack`.

Canonical policy dictates that names should use UK spelling and not US or other spelling.

Good names with the help of concise doc comments do a good job of explaining a good API. However, if after much consideration, there don’t seem to be any good names, this is likely caused by the API not being good. If an API cannot be easily and intuitively explained, it is not a good API and it’s time for a refactor.

A good, semantically and behaviorally consistent API hidden behind a layer of bad naming is hard to distinguish from a bad API. Time spent getting the right naming will pay off.

Great care should be taken over all names, but extreme care should be taken over publicly exposed ones. These names do not have the luxury of being able to be tweaked without consequence later—any appreciation for a slightly better name an external user may have will be completely overshadowed by their irritation of having to deal with a breaking change.

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

## When to use `Self`

Use `Self` wherever possible (i.e. it doesn’t conflict with the next section).
This keyword helps reduce the number of types the reader must keep in their head and hence the difficulty of reading code which relates to the type in of the current `impl` block.
By maximising the use of `Self` it also highlights where it _isn’t_ possible to use it, for example when the return type of a function has a slightly different set of generic parameters.

✅ Do this:

```rust
impl Node {
    pub fn new(parent: &Self) -> Self {
        Self(..)
    }
}

impl PartialOrd for Node {
    fn cmp(&self, other: &Self) -> Ordering;
}
```

⚠️ Avoid this:

```rust
impl Node {
    pub fn new(parent: &Node) -> Node {
        Foo(..)
    }
}

impl PartialOrd<Rhs=Node> for Node { // NB: Rhs=Self is also the default for PartialOrd.
    fn cmp(&self, other: &Node) -> Ordering;
}
```

## When not to use `Self`

Do not use `Self` when constructing associated types.

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

# Comment and Doc discipline

## First-sentence form

When wrapped, the first sentence of a doc comment should be at most two lines.
It should clearly and concisely explain the whole of the golden path of a function.
After reading this first sentence, it should be clear _when_ to use the given function/type—don’t fall into the trap of just explaining _what_ the given item does!

## Definite vs. Indefinite articles

When referring to parameters, be concrete and specific.
Where possible, refer parameters by their name and if an article must be used (i.e. ‘a’/’an’ and ‘the’), always prefer the definite article, ‘the.’
Leave no room for ambiguity and hence misunderstanding.

<!-- ## Further reading -->
<!-- TODO(kcza): flesh this out -->
<!-- Check out  Effective Go and  Google’s style guidelines. If you find yourself in a situation which isn’t covered by this document, refer to these. If you feel this situation is common-enough, message Ed Jones to discuss adding it to this document. -->

# (Admin, to be removed once stable)

| who’s | done      | what |
|-------|-----------|------|
| Ed    | yes       | all function calls which return () should end with ; |
| Ed    | yes       | all explicit return statements must end with ; |
| Ed    | yes       | if let Some(x) = x (pattern matched must be the same as the value being matched, or share the first letter, same true for match) |
| Ed    | yes       | impl ordering (unsafe then impl StandardTrait for T, impl NonStandardTrait for T, more general first |
| Ed    | yes       | derive ordering: Clone then Copy then standard (what order?) then non-standard in alphabetical order (or dependency order?) |
| Ed    | yes       | Semantic newlines |
| Ed    | yes       | Code grouping |
| Ed    | yes       | Forbid struct construction with Self::XXX {...}, for associated types, use the type name explicitly. NB: this is not enums! |
| Ed    | yes       | Forbid use XXX::* unless XXX is an enum, and even then, this must be local (definitely do this if rustfmt would start awkwardly wrapping?) |
| Ed    | yes       | Forbid use of use super::... except for use super::* in unit tests |
|       |           | Struct population must not mix computed and assigned/renamed fields |
|       |           | Use a consistent order for naming |
|       |           | thiserror message forms |
|       |           | panic!() message forms |
|       |           | .expect() message forms |
|       |           | One-block declarations: consts then statics then lazy_statics (?) then lets. |
|       |           | Unsafe functions / functions containing unsafe must list preconditions (to what extent does clippy help with this?) |
| Ed    | yes       | Generic type parameters must be one letter |
| Ed    | yes       | Generic lifetime names must not contain underscores |
| Ed    | yes       | Generic lifetime names must have a meaningful name if one exists (if long-lived?) |
| Ed    | yes       | Function doc’s first sentence must be no more than 160 chars (wrapped) |
| Ed    | yes       | Definite vs indefinite articles in docs |
| Ed    | REJECTED  | Comments must be no wider than 80 characters from their leftmost point |
| Ed    | yes       | impl LocalTrait for ForeignType order: std first, then non-std but popular, then others in a reasonable order |
| Ed    | yes       | Use Self as reasonably possible (in function return types, value construction Self{}/Self()/Self) |
|       |           | No use of declarations in format-like macros (the key=value part at the end), prefer local lets |
|       |           | Use `.map(\|_\| ())` rather than `.map(drop)` |
|       |           | If a where clause exists, all type bounds must be within it (no mixing) |
|       |           | Clippy lints to turn on: no broken links, |
|       |           | Prefer scoping to drop (avoid all drop calls?) |
|       |           | Always use clap-derive not the old API |
|       |           | Prefer iterator combinators if infallible, consider avoiding if fallible |
|       |           | No method calls on struct literals |
|       |           | Scoped mutability—names must match: let x = { let mut x = …; …; x} |
|       |           | Prefer Foo::new() or equivalent to Foo{} |
|       |           | All imports must be in one block (until rustfmt stabilises its many sorted blocks) |
| Ed    | yes       | Imports must be nested (no Java-style repetition) |
|       |           | Error type must be defined in lib.rs or error.rs in crate root |
|       |           | Always declare Result<T>, this must be done immediately below the error type |
|       |           | Don’t use mod.rs, use foo.rs next to foo/ to help code editors! |
|       |           | Constructing boxed errors? |
|       |           | When using builders, provide a MyType::builder() -> MyTypeBuilder? |
|       |           | MyTypeBuilder must have a .build() -> MyType (duh) |
|       |           | Prefer builders by value? (avoid awkwardly relying on clone() calls getting optimised away) |
|       |           | Use explicit Ok(()) explicitly rather than Ok(returns_unit()) (split into two lines) |
|       |           | No method calls on (curly brace?) macro results? |
|       |           | Prefer explicit matches (avoid catch-all _ => () cases) |
|       |           | Start by defining the error states (the error path must be first-class, even if not optimised) |
|       |           | Leverage the type system to uphold invariants and prevent misuse |
|       |           | Minimise use of unsafe and scope as closely as possible to where you actually need unsafe |
|       |           | Use rustfmt, but use THESE SPECIFIC (insert here) settings, do not use #[rustfmt::skip]! |
|       |           | Prefer let xxx: Foo to turbofish to add type annotations on the last thing in a chain (unless really annoying!) i.e. prefer to signpost what the goal of a large chain is, also can be slightly more reliable in some cases |
|       |           | For locally-written Serde, use an explicit return and put the helper types at the end of the function, don’t write macro helpers for the helper types! (Type names don’t need to be crate-level unique, just distinct enough but within reason e.g. no type shadowing)
|       |           | Shadowing: two levels of variable shadowing okay, type shadowing is never okay! |
|       |           | In mod.rs equivalent, put all mods first, then all pub uses then pub(crate) uses all uses in separate blocks (grouped nicely) then #[cfg(..)] after all |
|       |           | All use statements must come before all code |
|       |           | When read from top-to-bottom, try to make this feel like a tour of the API (guide the reader) |
|       |           | Redundant type annotations—does clippy prevent let foo: Foo = (...).collect::<Foo>();? |
|       |           | No .map(Self) at the end of a chain (e.g. to construct Result<Self> |
|       |           | No let x = &expr unless indexing/slicing |
|       |           | Use `vec![]` rather than Vec::new(), use Foo::new() rather than Foo::with_capacity(0) (for which Foo? yes: Vec, HashMap, BTreeMap) |
|       |           | Don’t pattern-match on pointer parameters i.e. don’t `.map(\|&x\| x)`, use `.map(\|x\| *x)` |
|       |           | Don’t populate tuples with lots of computation. If the line breaks, put things in variables |
|       |           | Don’t pattern-match in fn functions (like one would expect in closures) |
|       |           | Put pub fields first in mixed structs. If this breaks Ord, write it manually |
|       |           | Prefer pattern matching to field-access where all fields should be considered |
|       |           | Don’t assign or compare &lit (e.g. &0, &""), put the & as close to where needed as possible and deref instead |
|       |           | In tests, #[test] (or equivalent) should be the final attribute |
|       |           | If hex is used, make it lowercase |
|       |           | All public top-level items must be documented |
|       |           | Put raw API-data unpacking structs at the end of the function they’re needed in after // serde structs |
|       |           | Put test helper structs at the end of the function they’re used in, after // test structs |
|       |           | Prefer to call API response structs Response (no shadowing due to independent scopes) |
|       |           | Unpack to ensure all fields are considered |
|       |           | advice use of `.ok().unwrap_or_else(\|\|...)` if `.unwrap_or_else(\|_\| …)` is seen |
|       |           | which files #![...] annotations are placed in |
|       |           | unused trait method parameters should use let _ = param rather than named _foo or annotations |
|       |           | format! parameters which consist of a single identifier (no pathing) should be folded into the format string |
|       |           | Builder creation: prefer Foo::builder() to FooBuilder::new() (also have no public ::new() on the builder) |
|       |           | Local helper types should be put behind a comment at the end, after an explicit return? |
|       |           | Prefer `.collect()` to `Foo::from_iter` |
|       |           | Functions upon which functions depend should come after |
<!-- TODO(kcza): function dependencies AFTER -->

[dictionary]: https://www.dictionary.com/
[thesaurus]: https://www.thesaurus.com/
