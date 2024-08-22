# Rust best practices

This is a list of best-practices which originate from discussions with both our CTO and the company’s other tech leads. All points listed here must be strongly considered before merging code at Canonical. Individually, the things written here may seem unimportant or even trivial, but each of these is a crucial building block for writing good, clean code. You wouldn’t want to hire a builder who has a reputation for installing rotten beams.

Ideally, this document would be a spec for a new AI linter but at present we must rely on the diligence of humans. Therefore, this document should be considered as a style guide for ‘Canonical Rust,’ and is intended to complement Rust for Rustaceans and well thought-out, consistent API design.

If you spot any of the problems detailed here in our existing code-bases or patches, don’t be afraid to fix them—a good codebase is inherently more maintainable and will cause fewer headaches and annoyances later. Remember that all code should aim to be locally-consistent—new code shouldn’t stick out like a sore thumb. Note also that the perfect is the enemy of the good—sole focus on fine-tuning code at the cost of all forward progress doesn’t keep the lights on.

_Disclaimer: this is not a complete list, more items can and likely will be added in future._
If you find an item which you believe should be in this document, please open an issue.

# Table of Contents

- [Preconditions](#preconditions)
- [Cosmetic discipline](#cosmetic-discipline)
  - [Spacing](#spacing)
  - [Grouping](#grouping)
- [Naming discipline](#naming-discipline)
  - [Name content](#name-content)
  - [Pattern match variable naming](#pattern-match-variable-naming)
  - [Generic type parameter naming](#generic-type-parameter-naming)
  - [Lifetime parameter naming](#lifetime-parameter-naming)
  - [Builder naming](#builder-naming)
  - [Exhaustively match to draw attention](#exhaustively-match-to-draw-attention)
  - [Pattern matching in parameters](#pattern-matching-in-parameters)
  - [Don’t pattern-match pointers](#don’t-pattern-match-pointers)
  - [Avoid numeric tuple-indexing](#avoid-numeric-tuple-indexing)
- [Error discipline](#error-discipline)
- [Code discipline](#code-discipline)
  - [Don’t import all](#don’t-import-all)
  - [Import grouping](#import-grouping)
  - [Import form](#import-form)
  - [Import self explicitly](#import-self-explicitly)
  - [When to use `Self`](#when-to-use-`self`)
  - [When not to use `Self`](#when-not-to-use-`self`)
  - [Impl block placement](#impl-block-placement)
  - [Impl block ordering](#impl-block-ordering)
  - [Derive ordering](#derive-ordering)
  - [Declaration ordering](#declaration-ordering)
  - [Struct population](#struct-population)
  - [Tuple population](#tuple-population)
  - [Avoid loosely-scoped `let mut`](#avoid-loosely-scoped-`let-mut`)
  - [Avoid unassigned `let` declarations](#avoid-unassigned-`let`-declarations)
  - [Format arg inlining](#format-arg-inlining)
  - [Avoid explicit `drop` calls](#avoid-explicit-`drop`-calls)
  - [Generic type parameter constraints](#generic-type-parameter-constraints)
  - [Prefer constructors](#prefer-constructors)
  - [Method calls on closing curly braces](#method-calls-on-closing-curly-braces)
  - [Type annotations](#type-annotations)
  - [API-specific Serde implementations](#api-specific-serde-implementations)
  - [Shadowing](#shadowing)
  - [How to structure `mod.rs`](#how-to-structure-`mod.rs`)
  - [Reference scope](#reference-scope)
  - [Struct field ordering](#struct-field-ordering)
  - [Hex values](#hex-values)
  - [Prefer `collect` when interacting with `FromIterator`](#prefer-`collect`-when-interacting-with-`fromiterator`)
  - [Empty `Vec` construction](#empty-`vec`-construction)
  - [Item ordering](#item-ordering)
- [Function discipline](#function-discipline)
  - [No-information returns](#no-information-returns)
  - [Hide generic type parameters](#hide-generic-type-parameters)
  - [Pattern-matched parameters](#pattern-matched-parameters)
  - [Unused parameters in default trait function](#unused-parameters-in-default-trait-function)
  - [Builder ownership](#builder-ownership)
- [Error discipline](#error-discipline)
  - [Error messages](#error-messages)
  - [Error types](#error-types)
  - [Panic calmly](#panic-calmly)
- [Unsafe discipline](#unsafe-discipline)
  - [Minimise unsafe](#minimise-unsafe)
  - [Document preconditions](#document-preconditions)
- [Project structure discipline](#project-structure-discipline)
  - [Use `mod.rs` to declare a module-root](#use-`mod.rs`-to-declare-a-module-root)
  - [Define `Error` and `Result` in standard locations](#define-`error`-and-`result`-in-standard-locations)
- [Comment and Doc discipline](#comment-and-doc-discipline)
  - [First-sentence form](#first-sentence-form)
  - [Definite vs. Indefinite articles](#definite-vs.-indefinite-articles)
- [Further reading](#further-reading)
- [(Admin, to be removed once stable)](<#(admin,-to-be-removed-once-stable)>)

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
- **comprise correct words**—if there is any disagreement over the implications of chosen words, then there will be some reader who gets the wrong idea. It’s better to spend more time discussing internally than to confuse a user. (Example: in Starlark, a 20-minute discussion was needed on the choice between `NOT_SAFE` vs `UNSAFE` as an empty value of a set of safety flags, where each flag had a name like `FOO_SAFE`.) **be unified**—there should be one and only one name for concepts used. If more are used haphazardly, it implies a difference where there is none and thus muddies the water.
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

To avoid them being mistaken for concrete types, generic type parameters must have single-letter names.

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
struct ASTQueryMatch<'cursor, 'tree> { .. }

struct Value<'h> { .. }
```

⚠️ Avoid this:

```rust
struct ASTQueryMatch<'a, 'b> { .. }

struct Value<'value> { .. }
```

## Builder naming

If a builder for a type `MyType` is provided, then it should have an associated function `builder()` which returns a `MyTypeBuilder`.
The type `MyTypeBuilder` should not have a public constructor.
In typical usage, users of `MyType` shouldn’t need to import `MyTypeBuilder`, it should be a seamless part of `MyType`.

The builder `MyTypeBuilder` must also have a fallible `.build()` method, which returns a `MyType`.

## Exhaustively match to draw attention

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

## Don’t pattern-match pointers

When taking a pointer as an argument to a closure, it is possible to pattern-match the pointer to obtain the value at the other end.
Although it may be convenient, it ultimately harms readability—it is clearer to explicitly dereference the pointer we are given.

✅ Do this:

```rust
    .map(|x| *x)
```

⚠️ Avoid this:

```rust
    .map(|&x| x)
```

## Avoid numeric tuple-indexing

Although sometimes a handy shorthand, indexing tuples with .0, .1 etc. deprives us of the opportunity to insert a good name in the same way that field-access on a struct would. Instead, prefer to use pattern-matching to give human-friendly names to the data being handled.

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

# Code discipline

## Don’t import all

In general, do not import `*` from a crate. Consider a source file which does this twice from two different dependencies, making use of items from each. Now, consider what happens when these crates are updated and some of these items are removed—the compiler will complain of undefined symbols, but will have absolutely no idea where these came from. Or worse, as the global namespace is used, updates can now cause name-clashes! By sticking to explicit imports only, we help ourselves and our future maintainers.

A corollary of this is that preludes, regardless of their initial convenience, should not be used by us in our own code. Nevertheless, they remain a handy tool for others to use when prototyping, so we should still consider exposing them where appropriate.

The only exception to these rules is that in the context of a unit test module, inserting use `super::*` is acceptable as it is a well-established and clear convenience.

The rule around enums is slightly different. Here, it is acceptable to import `*` to bring all items of an enum into scope. However, this should not be done at the top level, only locally to improve the readability of long match statements. There, they should be placed as close as possible to the relevant match, preferably on the line immediately preceding it.

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

Traits often include type fields as part of the interface they describe. These types may be referred to with `Self::AssociatedType`. Do not use these to construct values as it prevents the next reader from understanding which type is in use and which fields and methods are available. Use these `Self::*` types to define interfaces, but use concrete types to define implementations.

The only exception is for trait items which return a `Result<_, Self::Err>`, where `Err` is set to the crate’s `Error` type. In this case, it is okay to use the crate’s `Result` type alias instead.

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

## Declaration ordering

Rust provides several different types of declaration and where these are declared consecutively in the same block, they should be ordered for visual consistency.
As this will help highlight the important information rather than appearing as alphabet soup.

Declarations should be ordered as follows:

- `const`
- `static`
- `lazy_static!`
- `let`
- `let mut`

## Struct population

Structs, like tuples, provide an excellent way to group together related information of different types.
Unlike tuples however, they force values to be named avoiding any forgettable-ordering problems.
When populating each field, there are three forms:

- The field has its value moved in from a variable with the same name,
- The field has its value moved in from a variable with a different name,
- The field takes its value from the result of some computation.

The first form is the cleanest for struct population as not only does it use the least characters, it also implies that the concepts being handled are very compatible, making the code easier to read.
Wherever reasonable, we should aim for this.

The second form is acceptible if the name of the field and the name of the value are similar, for example one being contained in the other.
If it is not possible to tune naming to make this true, this is a sign of messy concepts at play.

The third form can be the most problematic as large computations often draw too much attention, effectively hiding both smaller computations and the other two field-forms above.
Drawing attention in this way indicates that the code awkwardly interleaves separate actions—the same section of code is both populating and doing detailed computation of the contents of a struct.
To avoid some fields sticking out like a sore thumb, either all fields of a struct should be computed or none should be.
In case of a mix, refactor computations into a new `let` declarations and use the first form above.
This avoids the reader having to wade through alphabet soup.

The order in which fields are declared must be the same as the type declaration.

Each line which populates a field in a given struct must be independent.
If there is a dependency between field declarations, for example if some shared state is mutated during their construction, `let` declarations and the first form must be used instead.

✅ Do this:

```rust
struct Entry<K, V> {
    id: u64,
    key: K,
    value: V,
    pretty_date_modified: String,
}

fn get_entry(&self, key: K) -> Result<Entry<K, V>> {
    let id = self.id_of(key)?;
    let value = self.get(key)?;
    let pretty_date_modified = self.date_modified(key)?
        .format_as("yyyy-MM-dd@hh:mm:ss");
    Entry {
        id,
        key,
        value,
        pretty_date_modified,
    }
}
```

⚠️ Avoid this:

```rust
struct Entry<K, V> {
    id: u64,
    key: K,
    value: V,
    pretty_date_modified: String,
}

fn get_entry(&self, key: K) -> Result<Entry<K, V>> {
    let value_stored_at_key = self.get(key)?;
    Entry {
        id: self.id_of(key)?,
        pretty_date_modified: self.date_modified(key)?
            .format_as("yyyy-MM-dd@hh:mm:ss"),
        key,
        value: value_stored_at_key,
    }
}
```

## Tuple population

Tuples are most easily read when they are short as once line-breaks occur, the structure being created starts to get harder to see.
Keep things visually simple—if the formatter chooses to break tuple population into multiple lines, instead introduce new `let` declarations to move computation away from the tuple population.

✅ Do this:

```rust
let key = some_long_computation()?
    .something_else()
    .another_thing();
let value = some_other_long_computation()
    .chained_with_something_else()?;
(key, value)
```

⚠️ Avoid this:

```rust
(
    some_long_computation()?
        .something_else()
        .another_thing(),
    some_other_long_computation()
        .chained_with_something_else()?,
)
```

## Avoid loosely-scoped `let mut`

In many cases, mutability is used to create a given structure which is then used immutably for the remainder of its lifetime.
Whenever this happens, scope the mutable declarations to just where they are needed, thus forcing a compiler error if this condition is broken in future.
Doing this also makes code simpler to read as there are fewer things which can change at any one point.

```rust
let my_structure = {
    let mut my_structure = MyStructure{}
    // Mutate `my_structure` as required.
    my_structure
};
```

For greatest clarity, make sure the name of the outer (immutable) and inner (mutable) declarations have the same name, here `my_structure`.

If mutability is to retain some state whilst iterating through a structure, consider using a functional style instead.
As a simple example, if presented with the following imperative code to count the number of spaces in a string

```rust
let mut num_spaces = 0;
for c in my_string.chars() {
    if c == ‘ ‘ {
        num_speces += 1;
    }
}
```

Consider instead, using the functional style to avoid the mutation—

```rust
let num_spaces = my_string.chars()
    .filter(|c| c == ‘ ‘)
    .count();
```

## Avoid unassigned `let` declarations

Let-declarations without values indicate an odd flow of data through a function.
Instead, prefer to return the required value from the block which computes it.

✅ Do this:

```rust
let message = if result.is_ok() {
    "success!"
} else {
    "failed!"
}

let mut retries = 0;
let message = loop {
    let resp = exec_web_request();
    match resp {
        Ok(m) => break m.text(),
        Err(Error::NetworkUnavailable) => {},
        Err(e) => return Err(e),
    }

    retries += 1;
    if retries > 5 {
        return Err(Error::Unavailable);
    }
}
```

⚠️ Avoid this:

```rust
let message;
if result.is_ok() {
    message = "success!";
} else {
    message = "failed!";
}

let mut retries = 0;
let message;
loop {
    let resp = exec_web_request();
    match resp {
        Ok(m) => {
            message = m.text();
            break;
        },
        Err(Error::NetworkUnavailable) => {},
        Err(e) => return Err(e),
    }

    retries += 1;
    if retries > 5 {
        return Err(Error::Unavailable);
    }
}
```

## Format arg inlining

Arguments to `format!`-like macros should aim to be as similar as possible to the string they are intended to produce.
Whenever a single variable is used in a format argument, it should be inlined to avoid the reader needing to dart back and forth between the format string and its arguments (both of which may stretch over multiple lines).

_NB: older Rust versions may not support this syntax._

✅ Do this:

```rust
format!("{path}/{file}")
```

⚠️ Avoid this:

```rust
format!("{}/{}", path, file)
```

## Avoid explicit `drop` calls

If a value must be dropped early, use curly-braces rather than an explicit `drop` call.
This will highlight the non-standard lifetimes of the values in scope whilst also avoiding the reader missing the `drop` call, which may be nestled among busy-looking lines of code.

Similarly, `drop` should not be used to discard values in call chains.
If a single value is to be ignored, use the ‘toilet operator,’ `|_| ()`, i.e. a closure which takes one argument, ignores it and returns a unit.
This form remains consistent ignoring some parts but not all of an incoming value, such as when extracting keys from key-value pairs—

```rust
    .map(|(key, _)| key)
```

There are two exceptions to this.

If `|_| ...` is used and the ignored parameter is an `Error`, we should more highlight that an error is intentionally being ignored, for example by using `.ok()` on a `Result` being handled.

If converting from some `Result<T>` to a `Result<()>` at the end of the last expression of a function, instead of the ignore marker, use the `?` operator and an explicit `Ok(())`.
This highlights that we care only about side-effects, and that no information is returned in the successful case.

✅ Do this:

```rust
async fn log(&self, message: String) -> Result<()> {
    {
        let mut file = OpenOptions::new()
            .append(true)
            .open(&self.log_file_path)?;
        file.write_all(message.as_bytes())?;
    }
    self.transmit_log(message).await?;
    Ok(())
}
```

⚠️ Avoid this:

```rust
async fn log(&self, message: String) -> Result<()> {
    let mut file = OpenOptions::new()
        .append(true)
        .open(&self.log_file_path)?;
    file.write_all(message.as_bytes())?;
    drop(file);
    Ok(self.transmit_log(message)
        .await
        .map(drop))
}
```

## Generic type parameter constraints

Generic type parameter constraints should be grouped in the same place.
If some constraints are complex enough to be placed into a `where` clause, then all constraints should be moved from angle-bracket declarations into the `where` clause also.

If constraints are declared in angle brackets and that declaration is longer than 30 characters, move all constraints into a `where` clause.

These rules ensure that the reader cannot miss any constraints.

✅ Do this:

```rust
impl<'a, ‘b, I, T> SomeStruct<'a, 'b, I, T>
    where
        ‘b: ‘a,
        I: IntoIterator<T> + ‘a,
        T: ‘b,
{ ... }
```

⚠️ Avoid this:

```rust
impl<'a, 'b: 'a, I: IntoIterator<T> + 'a, T: 'b> SomeStruct { ... }
```

## Prefer constructors

When exposing structs, prefer to expose constructor functions or builders rather than exposing public fields.
There are several benefits here:

- They format more nicely in call chains
- They allow conversions to occur implicitly
- They allow some fields to be computed in terms of others using implementation-specific details

Structs with all-public fields cannot benefit from any of the above and moreover, if it is later decided that any of these properties is beneficial, we face either a breaking change to fix it or extra complication to work around it.

The exception here is for ‘options’ structs, which are passed to single function and which configure its behaviour.

## Method calls on closing curly braces

Control structures and struct literals should not have methods called upon them as they are often used as final expressions in blocks and once formatted these calls often get moved onto the line below.
This adds an unwelcome surprise as the scope of what the reader is currently looking at will appear to increase, adding cognitive load.
Instead, use a binding (`let some_var = ...; some_var.foo()`).

When designing APIs, if a public struct will be filled by consumers for the purpose of calling a single method on it, consider instead reversing the dependency by using a free function which takes the struct as its first parameter.

✅ Do this:

```rust
do_thing(Foo {
    bar: "asdf",
    baz: "fdsa",
})?;

let value = if some_condition {
    value_a
} else {
    value b
};
value.to_string()
```

⚠️ Avoid this:

```rust
Foo {
    bar: "asdf",
    baz: "fdsa",
}
.do_thing()?;

if some_condition {
    value_a
} else {
    value_b
}
.to_string()
```

## Type annotations

The compiler’s type inference is usually very good, but sometimes, we must give it a helping hand.
However, we must careful of how much information we provide and where the necessary annotations are done.

Provide only the minimum amount of information required to help the compiler.
If the compiler does not complain when a type annotation is removed, then it should be removed.
In particular, always make note of the return type of the function currently being written as often, this provides enough information to make all type annotations obselete.

Let’s briefly consider collecting an iterator of `SomeLongType<...>` into a `Vec`.
The compiler will complain that it does not know which collection type to collect into and although explicitly annotating `Vec<SomeLongType<...>>` works, the information the compiler needs is that a `Vec` is being handled, so it is better to use just `Vec<_>`.
This has two effects:

1. The code is cleaner as unnecessary information does not draw the eye
2. The code is more maintainable as the types we handle are allowed to change (the necessary properties are still implicitly enforced at function boundaries)
3. The coder’s wrists are less stressed

In an function implementation, Rust provides three main ways to insert annotations:

1. On `let` declarations, using `let foo: Type`
2. On functions using the turbofish, `func::<T>()`
3. On trait items using the fully-qualified syntax `<Value as Trait>::trait_item`

This is also the order of preference in which these should be used.

The best place to put annotations is on `let` declarations, as this not only makes it clearer what the values being handled are, but also if a `let` takes the results of a long chain of calls spanning many lines, it also makes it more obvious what’s being worked towards.
This pattern can also provide even more information than the other two in some cases.

Next, if type information must be given in the middle of the expression, the turbofish may be used.
One place to be careful though is if `collect` being used in the middle of an expression is what requires annotation—there is often a cleaner and more efficient way to achieve the same thing.

The tool of last-resort is the fully-qualified syntax which disambiguates between same-name items on the same type which come from different namespaces in scope.
If an API you are making can cause an abundance of name-clashes, your consumers will be unhappy.
Refactor it as best you can.

✅ Do this:

```rust
let x: Vec<_> = foo.iter()
    .filter(...)
    .map(...)
    ...
    .flat_map(...)
    .collect();
```

⚠️ Avoid this:

```rust
let x = foo.iter()
    .filter(...)
    .map(...)
    ...
    .flat_map(...)
    .collect::<Vec<SomeExtremelyLongType<With, Generics, And<'lifetimes>>();
```

## API-specific Serde implementations

Data-formats returned from remote APIs should not govern internal representation of data without good reason as we may end up with remote API changes causing significant changes across our codebases.
To avoid this, it is good practice to declare (de)serialisation types which closely match the remote API and then explicitly map these into our own types.
If you see `Serialize`/`Deserialize` being implemented on data which is received from or will be sent to a remote, this is a sign of a remote API bleeding into our code.

Instead, define (de)serialisation types in the functions which implement the necessary API calls.
Let’s say we have a function called `get_image_info`, which makes a web-request to get information associated with given container image name (e.g. author, description, latest version).
To nicely transfer data from some remote format into one we govern, say `ImageInfo`, add an explicit `return` at the end and _below_ this, create a new type called `Response`, which implements `Deserialize`.
Add a comment which says `// serde structs.` to let the reader know that everything beyond this point only relates to modelling the remote API.
If the remote responds with a nested structure, add more types, always trying to maintain a 1:1 relationship between Rust types and the remote format—

```rust
async fn get_image_info(&self, name: &str) -> Result<ImageInfo> {
    let response = self.client.get(...).await?;
    if !response.status_code().is_success() {
        return Err(Error::OhNo { ... })
    }

    let parsed_response: Response = serde_json::from_str(response.text());
    let info = ImageInfo {
        name,
        version: response.metadata.version,
        authors: response.metadata.authors,
        latest_release: response.releases.last()
            .map(|release| ...),
    }
    return Ok(info);

    // serde structs.
    #[derive(Deserialize)]
    struct Response {
        metadata: Metadata,
        releases: Vec<Release>,
    }

    #[derive(Deserialize)]
    struct Metadata {
        version: String,
        authors: Vec<String>,
    }

    #[derive(Deserialize)]
    struct Release {
        ...
    }
}
```

For consistency, we try to always call incoming data `Response` and outgoing data `Message`.
Name shadowing is okay here as the scope is small and well-defined.

Scoping the deserialisation in this way is extremely good practice for several reasons:

Firstly, it minimises the blast radius of incoming remote API changes. If a remote API changes, we need only update the deserialisation structs and their unpacking into our internal ones—the core of our program/library remains untouched.

Secondly, it minimises the amount of code which must be read—if the interaction with the remote API is functioning correctly but someone wishes to know how this function works, they know that they can stop reading past the `// serde structs.` marker.
Conversely, if the API interaction is broken due to a data format ‘surprise,’ that same comment draws the maintainer’s eye to the place they need.

Thirdly, it is often simpler to implement the Serde traits on these types! As we model the remote structure before unpacking into internal ones, less `#[serde(...)]` wrangling is required.

Finally, it reduces the amount of clutter in file-level scopes.
As the `Response` types are locally-scoped, the reader knows exactly where they are used and hence does not need to keep them in mind alongside the rest of the codebase.

## Shadowing

Shadowing provides an excellent way to manipulate data or change its type whilst still highlighting that the ‘same’ data is being processed, however, too many levels of shadowing make things confusing.
In general, if shadowing with scope (i.e. within `{}`), use at most one levels of shadowing.

```rust
if let Some(name_override) = name_override {
    // `name_override` is shadowed, don’t shadow it again.
}
```

Shadowing and changing type in the same scope can be performed at most once per name (e.g. for `into` conversions).

```rust
fn init(self) -> InitedSelf {
    let Self {
        some_field,
        ...
    } = self,
    // ...
    let some_field = some_field.into();
    // ...
    InitedSelf {
        some_field,
        ...
    }
}
```

Shadowing and not changing in the same scope can be done as many times as needed, however if this is being used to mutate a value during its construction, instead consider the scoped mutability pattern—

```rust
let thing = {
    let mut my_thing = ...;
    // Mutate `my_thing` to construct it...
    my_thing
};
```

## How to structure `mod.rs`

Files named `mod.rs` must only be used to specify the structure of a project, if definitions are added, it can quickly become very messy and detract from its core purpose of declaring sub-modules and the current module’s interface.

The `mod.rs` file must be separated into distinct blocks in the following order, keeping the most public items first:

1. `pub mod ...` declarations
2. `pub(crate) mod ...` declarations
3. `mod ...` declarations
4. `pub use ...` declarations
5. `pub(crate) use ...` declarations
6. `pub use ...` declarations

Any items gated behind a `#[cfg(...)]` must be placed at the end of the file, in the same order as the above.
Like-gated items should be wrapped in a block, i.e. `#[cfg(...)] { /* items here */ }`.

No other items should be present.

Note that these guidelines also hold for `lib.rs`, with the exception that a crate’s `Result` and `Error` types are permitted in `lib.rs`, given their central importance.

## Reference scope

In many cases, the compiler is smart enough to create temporary storage locations to store variables which are given the value `&expr`, however, when these are passed to functions, it becomes slightly harder to follow which type is being used, especially when handling `&T` where `T` is `!Copy`.
In this case, it is only a single character in the variable declaration, possibly many lines away which shows that the value `T` is not being moved, only its reference.

Instead of relying on temporary storage locations, store the value explicitly and take a reference where needed.
This way, the transfer of ownership is much more explicit.
As a rule of thumb, only use `&` at the start of the value of a `let` declaration when either indexing or slicing.

✅ Do this:

```rust
let foo = from_func();
other_func(&foo);
```

⚠️ Avoid this:

```rust
let foo = &from_func();
other_func(foo);
```

## Struct field ordering

The more public an item, the more likely a user is to want to know more about it and understand it.
Therefore, we should put the items they are most likely to care about nearer the top of our code.
In the case of struct definitions this means that `pub` fields should come first, then `pub(crate)` ones and then private ones.
Structs neatly organised in this way make it clear when they have entered implementation details and hence when they may stop reading.

If a particular field-ordering is required for a derived `Ord`/`PartialOrd` implementation, instead write those out by hand as the reader is less likely to need to look at these.

✅ Do this:

```rust
struct ScriptThread<T> {
    pub user_data: T,

    pub(crate) global_vars: BTreeMap<String, Value>,

    stack: Vec<StackFrame>,
    max_steps: Option<u64>,
    steps: u64,
}
```

⚠️ Avoid this:

```rust
struct ScriptThread<T> {
    stack: Vec<StackFrame>,
    pub(crate) global_vars: BTreeMap<String, Value>,
    pub user_data: T,
    steps: u64,
    max_steps: Option<u64>,
}
```

## Hex values

Unless there is an existing convention, hex values should be lowercase as this avoids creating visually-impenetrable rectangles.
By using lowercase, we provide more ‘handles’ for the eye to use.

✅ Do this:

```rust
const SOME_SPECIFIC_IMPORTANT_VALUE: u64 = 0xab5c4d320974a3bc;
```

⚠️ Avoid this:

```rust
const SOME_SPECIFIC_IMPORTANT_VALUE: u64 = 0XAB5C4D320974A3BC;
```

## Prefer `collect` when interacting with `FromIterator`

The `FromIterator` trait defines a method `from_iter` which is called by `Iterator::collect`.
We therefore have two methods of collecting into an iterator, `Foo::from_iter` and `collect()` with appropriate type bounds.
Prefer the latter as this makes the order of operations read from top to bottom.

✅ Do this:

```rust
let my_vec: Vec<_> = collection.into_iter()
                .filter(...)
                .collect();
```

⚠️ Avoid this:

```rust
let my_vec = Vec::from_iter(collection.into_iter().filter(...))
```

## Empty `Vec` construction

To construct an empty `Vec`, we have three options: `vec![]`, `Vec::new()` and `Vec::with_capacity(_)`.
If the size of the `Vec` resulting from an operation can be reasonably estimated, prefer `Vec::with_capacity` as this will reduce reallocations.
Otherwise if a zero-sized `Vec` is required, use `Vec::new()` as this indicates most clearly that no operation is being performed—the function `Vec::new` makes no allocations, but no such guarantee is given for `vec!`.
Consider also that `vec![expr; 0]` will still evaluate (and then immediately drop) `expr`.

## Item ordering

When read from top to bottom, a file should feel like a tour of the APIs it defines.
The most important items should be defined further up the file, with their helpers below.
No `impl` block should come before the type or trait to which it relates.
In this way, lower-level implementation details are hidden from the user until they wish to know more, at which point, the reader having gained a good knowledge of the overall form of the code, they can read on to understand how it functions.

✅ Do this:

```rust
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
impl Foo {
    fn some_helper_func(&self) {
        // ...
    }

    pub fn some_func(&self) {
        self.some_helper_func();
    }
}
```

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

## Hide generic type parameters

Generic type parameters add complication to understanding an API.
On functions, only introduce them explicitly if they are required in more than one place in the current signature, otherwise, avoid them by using the unnamed lifetime `'_` or `impl Trait`.
Rust’s elision rules allow us to hide ‘obvious’ lifetimes, by using `impl Trait`, a similar effect can be achieved.

Note that although it is possible to omit specifying even the unnamed lifetime (i.e. it may be possible to write `serde::Deserialize<’de>` as `serde::Deserialise`), this should never be done.
A type without lifetime parameters looks completely self-contained and hence can be freely passed around, but this is not the case!
If a lifetime is present, always communicate that fact (i.e. prefer `serde::Deserialize<’_>` to `serde::Deserialize`).

✅ Do this:

```rust
fn transmit(tx: impl Transmitter<'_>, message: &[u8]) -> Result<()> { ... }
```

⚠️ Avoid this:

```rust
fn transmit<'a, T: Transmitter<’a>>(tx: T, message: &[u8]) -> Result<()> { ... }
```

## Pattern-matched parameters

Using pattern matching in `fn` parameters obfuscates the purpose of values being handled, ultimately harming readability as the fact that this kind of unpacking is done is an implementation detail.
If parameters are to be unpacked, instead do this at the first line of a particular function.
Note that this guidance does not apply to closures, which are commonly used as small, locally-scoped helper functions.

✅ Do this:

```rust
impl Server {
    fn new(config: ServerConfig) -> Result<Self> {
        let Config { db_path, working_path } = config;
        // ...
    }
}
```

⚠️ Avoid this:

```rust
impl Server {
    fn new(ServerConfig { db_path, working_path }: ServerConfig) -> Result<Self> {
        // ...
    }
}
```

## Unused parameters in default trait function

Occasionally, a default trait function implementation is provided, but not all of its parameters are used, causing a compiler warning.
In this case, explicitly add `let _ = unused_param;` lines until all these warnings are removed.

In particular, we do not wish to rename the parameter to something like `_unused_param` as this appear in docs.
Similarly, we don’t want to `#[allow(unused_variables)]` as these will suppress unused-parameter warnings for parameters which are expected to be used in the default implementation.

✅ Do this:

```rust
trait CustomScriptValue<’v> {
    fn at(&self, index: Value<’v>) -> Result<Value<’v>> {
        let _ = index;
        Err(Error::Unsupported { .. })
    }
}
```

⚠️ Avoid this:

```rust
trait CustomScriptValue<’v> {
    fn at(&self, _index: Value<’v>) -> Result<Value<’v>> {
        Err(Error::Unsupported { .. })
    }

    // OR

    #[allow(unused_parameters)]
    fn at(&self, index: Value<’v>) -> Result<Value<’v>> {
        Err(Error::Unsupported { .. })
    }
}
```

## Builder ownership

In Rust, there are two forms of the builder pattern, depending on the receiver type used.
Consider the following builder—

```rust
let frobnicator = Frobnicator::builder()
    .foo("foo")
    .bar("bar")
    .build()?;
```

The methods `foo`, `bar` and `build` can either take ownership of `self` or take `&mut self` by value.
It `self` is used, it encourages a simple flow of data, with values being neatly moved from one place to another.
If `&mut self` is used, it makes conditional use of each of the builder methods simpler, however either the builder must be bound to a variable or all contained data must be cloned during `.build()`.
Although the optimiser may remove some unnecessary cloning, this should not be relied upon.
Besides, if the `.build` method takes a reference to the builder—allowing it to be called multiple times—this feels more like a _factory_ than a _builder._

# Error discipline

## Error messages

In Rust, it is a good idea to start writing a project by first making its `Error` type, this no only encourages message consistency, but also highlights the necessary information for these good error messages.
This in turn, makes it clearer which information should be plumbed where, avoiding the awkward and altogether too-common situation where an error condition is identified, but much work must be done to get the right information there.

Error messages should be concise. Every second longer a user spends reading an error message and not understanding what went wrong is a second longer of their frustration. The form of the phrases used in error messages should be consistent. If, likely from previous messages, someone is expecting—

```
cannot foo the bar
```

but instead reads—

```
barring failed for foo
```

they will experience unnecessary discomfort. Keep it nice and clean.

A majority of the time, error messages should start with a verb and in most cases that verb should be prefixed with ‘cannot.’

Capitalisation of error messages should be consistent. If it is possible that an error will not be a top-level error (e.g. it may be wrapped), then the first letter should be lowercase. Unexpected capital letters in the middle of a line of logging look dishevelled and imply that little thought has gone into the overall design of the implementation. This couldn’t be further from the truth so let’s make it appear as such.

Remember: the latin alphabet is optimised for many contiguous lowercase letters, hence lowercase is a good default to maintain. However, uppercase should be used for acronyms and standard names such as: TCP and NixOS.

When writing error messages, think about the background of the expected user/consumer of the project. Consider whether the person reading it will have good technical knowledge and tailor your approach as such.

If the reader of these error messages is expected to be a (Rust) developer (e.g. code in question is in a library or building block for something else), they will be familiar with how to fix simple problems and as such may not benefit from unnecessary advice. For these readers, a good concise error message will contain all the information needed to point them towards how to fix it. In this case, suggestions are at best superfluous and at worst unhelpful.

If the reader of these error messages is not expected to be a developer (e.g. these messages will appear in some GUI), think carefully about where you want to send that reader next. Whereas a developer might be okay to submit an issue, a non-developer will appreciate being explicitly pointed in the right direction.

Going up the stack, the reader’s knowledge of low-level details becomes less reliable hence they must lean more heavily on the help we give them.

As an error passes further and further up the stack, more context messages may be added to it. Be aware of how errors will bubble up to avoid repeating the same information multiple times.

Note that all this advice applies to both error messages associated with error types and panic messages.

## Error types

All reasonable types which implement `Error` fall into one of three categories:

- Those which erase the underlying types
- Those which preserve them, for example by enumeration
- Those which preserve them opaquely

Errors which use type-erasure (e.g. `Box<dyn Error>` and `anyhow::Error`) are often easier to use when writing code, however things become very problematic later on when attempting to inspect errors—with less help from the compiler comes far more places for subtle breakages to occur, not only now, but in future.
Type-erased errors should only be used in prototypes where maintenance will never be a concern, otherwise, use concrete types.
As a general rule, type erased errors _must not be used in library crates._
Type erasure is a very strong opinion and one which may not be shared by a crate’s dependants.
The process of converting from erased errors back to a contained concrete one is unpleasant and will irritate consumers.

Errors which preserve types (e.g those annotated with `#[derive(thiserror::Error)]`) give Rust a unique advantage—not only can the golden path receive first-class support, but so too can the error path, thus allowing an even higher level of quality to be attained.
In particular, the process of responding to particular errors is far more robust with enumerated errors.

Errors which preserve types but which represent unrecoverable errors should represent their error condition as a contained `&‘static str` or `String` which is assigned where the error is constructed.
When constructing these errors, special care must be taken to ensure that the message is consistent with other errors in the codebase.
The field used to hold the reason for the error in these cases should be named `reason`.

If one error wraps another, the inner error should be held in a field named `cause`.

## Panic calmly

Panics must only be used if a program enters an unrecoverable state.
Further, that unrecoverable state must not be as a result of user input—a program which a user can easily crash is not a good program.

In Rust, panics are very aggressive.
Not only are they unrecoverable within the same thread, but also if the default panic strategy is overridden (e.g. by a user who wants a smaller binary and hence sets `profile.release.panic = "abort"` in their `Cargo.toml`), we have no guarantee that the usual cleanup is performed.
In this case, we must rely on the OS to do its best with the resources, but we have no guarantee that this will be sufficient.
By default therefore, try not to panic.

If, however, a panic is inevitable, be sure that the message signals who is at fault—if it’s an internal error, start the message with `internal error:`.

The over-use of `.unwrap()` is a major red flag, as the resulting code is likely very fragile, relying on possibly-unclear preconditions in the code above it.
If that code changes—which is very likely in a program under active development—production code may panic.
As a rule of thumb, `.unwrap()` should only be used in code in tiny scopes, where errors can only possibly originate from the programmer—e.g. in `Regex::new("...").unwrap()`, a panic can only occur if the raw regex constant is invalid.
In general though, unwrapping should be replaced with either:

- Good use of the type system to allow the compiler to enforce preconditions
- Calls to `.expect` which document the required precondition (these must follow the same convention as panic messages)

The first option is very much preferable.

We can remove panics originating from `.unwrap()` calls by using the `?` operator to pass errors up the call-stack.
In the case of unwrapping options in a function which returns a result, calling the `.ok_or`/`.ok_or_else` methods may be required.

We can also remove panics originating from `.unwrap()` calls by using pattern-matching.
If a call to `x.unwrap()` is guarded by `if x.is_some()`/`if x.is_ok()`, instead make use of pattern matching: `if let Some(x) = x` or `if let Ok(x) = x`.

When calls to `.unwrap()` are removed, the surrounding code is not only more robust, but you may notice that it is often much cleaner.
This is not a coincidence—it is a nudge from the Rust developers to encourage fault-tolorant code and practices.

There are two exceptions to this avoidance of panicking: tests and panic-propagation.

If the `?` operator is used in tests, the origin of the error is lost, making the test harder to debug.
A failed `.unwrap()` will result in a trace pointing to where the panic occurred.

If a thread panics whilst it has acquired a `Mutex` lock, we have no guarantee that the contents of the mutex represents a valid state and hence the lock gets _poisoned._
This means that any other thread which attempts to lock the mutex will get an error.
In this case, panicking is acceptable it effectively propagates an existing panic from another thread.

# Unsafe discipline

## Minimise unsafe

Rust’s `unsafe` keyword turns off a small but important number of the compiler’s checks.
In effect, it is a ‘hold my beer’ marker—you tell the compiler to trust you and just watch whilst you do something crazy.
But the compiler is not the only entity whose trust we require, when we use `unsafe`, we also ask our _users_ to trust that we know exactly what we are doing.
We do not want to break trust.

Therefore, we must endeavour to minimise the use of unsafe in our code.
If there is a place where `unsafe` is used but not required for a strict functional requirement, drop it and replace it with a safe equivalent.
Such sections are inherently much harder to maintain as they require not only excellent documentation, but dilligence, careful consideration and above all time.
The more unsafe code a project contains, the slower it will be able to move forward in the long-run.

If `unsafe` is mandatory, minimise the scope of the `unsafe` blocks and functions in use.
Even if this means adding an extra line or two, maximise the amount of help the compiler can give us whilst minimising the number of unsafe interactions which will require audit.
A function of 50 lines wrapped in a single large `unsafe` block is far harder to maintain than a 53 line function containing three one-line `unsafe` blocks.

Note that ‘it’s faster’ is not a good reason to use `unsafe` constructs.
Even if true, unless profiling can categorically show that safety checks are a significant cost in a small, hot section of the codebase, the significant increase in burden is simply not worth it.
Rust is already an extremely fast language.

## Document preconditions

Both `unsafe` functions and `unsafe` blocks must document their preconditions in a comment with the prefix `// SAFETY: `.
If something goes wrong in future, this will help the future maintainer understand which conditions have been violated.
These comments must be carefully maintained as changes are made.

# Project structure discipline

## Use `mod.rs` to declare a module-root

When declaring a module `foo` which comprises multiple files, Rust allows two directory structures as follows.

Firstly, all files can be grouped into a single directory containing a `mod.rs`—

```
foo/
├── mod.rs
├── file_in_foo.rs
└── another_file_in_foo.rs
```

Secondly, all files can be grouped into a single directory, except for the module root which is stored next to the directory and shares its name—

```
foo.rs
foo/
├── file_in_foo.rs
└── another_file_in_foo.rs
```

Always prefer the first way as `mod.rs` are expected to have a specific structure, hence the reader knows what they’ll see when opening one of these.
Some editors group files and folders separately, so in the above example, the fact that `foo.rs` is related to the `foo/` directory will not be obvious when there are many other entries in between.

## Define `Error` and `Result` in standard locations

A crate’s `Error` and `Result` types are likely the most important types it contains, and should therefore be declared in a standard place.

In library crates, the `Error` type should be defined in the crate-root, `lib.rs`.
The custom `Result` type alias, `type Result<T> = std::result::Result<T, Error>` must be immediately below it.

In binary crates, the `Error` type should be defined `error.rs` and the `Result` alias in `result.rs`, both in the crate’s root directory.

# Comment and Doc discipline

## First-sentence form

When wrapped, the first sentence of a doc comment should be at most two lines.
It should clearly and concisely explain the whole of the golden path of a function.
After reading this first sentence, it should be clear _when_ to use the given function/type—don’t fall into the trap of just explaining _what_ the given item does!

## Definite vs. Indefinite articles

When referring to parameters, be concrete and specific.
Where possible, refer parameters by their name and if an article must be used (i.e. ‘a’/’an’ and ‘the’), always prefer the definite article, ‘the.’
Leave no room for ambiguity and hence misunderstanding.

# Further reading

Checkout the [`rust-analyzer` style guide](https://github.com/rust-lang/rust-analyzer/blob/master/docs/dev/style.md).
Note that those style guidelines are intended for a single project, hence it is possible for its developers to make more specific requirements which only make sense in the context of that project, unlike the guidelines provided here.

# (Admin, to be removed once stable)

| who’s | done     | what                                                                                                                                                                                                                                                                   |
| ----- | -------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Ed    | yes      | all function calls which return () should end with ;                                                                                                                                                                                                                   |
| Ed    | yes      | all explicit return statements must end with ;                                                                                                                                                                                                                         |
| Ed    | yes      | if let Some(x) = x (pattern matched must be the same as the value being matched, or share the first letter, same true for match)                                                                                                                                       |
| Ed    | yes      | impl ordering (unsafe then impl StandardTrait for T, impl NonStandardTrait for T, more general first                                                                                                                                                                   |
| Ed    | yes      | derive ordering: Clone then Copy then standard (what order?) then non-standard in alphabetical order (or dependency order?)                                                                                                                                            |
| Ed    | yes      | Semantic newlines                                                                                                                                                                                                                                                      |
| Ed    | yes      | Code grouping                                                                                                                                                                                                                                                          |
| Ed    | yes      | Forbid struct construction with Self::XXX {...}, for associated types, use the type name explicitly. NB: this is not enums!                                                                                                                                            |
| Ed    | yes      | Forbid use XXX::\* unless XXX is an enum, and even then, this must be local (definitely do this if rustfmt would start awkwardly wrapping?)                                                                                                                            |
| Ed    | yes      | Forbid use of use super::... except for use super::\* in unit tests                                                                                                                                                                                                    |
| Ed    | yes      | Struct population must not mix computed and assigned/renamed fields                                                                                                                                                                                                    |
| Ed    | yes      | Use a consistent order for naming                                                                                                                                                                                                                                      |
| Ed    | yes      | thiserror message forms                                                                                                                                                                                                                                                |
| Ed    | yes      | panic!() message forms                                                                                                                                                                                                                                                 |
| Ed    | yes      | .expect() message forms                                                                                                                                                                                                                                                |
| Ed    | yes      | One-block declarations: consts then statics then lazy_statics (?) then lets.                                                                                                                                                                                           |
| Ed    | yes      | Avoid `mut` as much as possible                                                                                                                                                                                                                                        |
| Ed    | yes      | Unsafe functions / functions containing unsafe must list preconditions (to what extent does clippy help with this?)                                                                                                                                                    |
| Ed    | yes      | Generic type parameters must be one letter                                                                                                                                                                                                                             |
| Ed    | yes      | Generic lifetime names must not contain underscores                                                                                                                                                                                                                    |
| Ed    | yes      | Generic lifetime names must have a meaningful name if one exists (if long-lived?)                                                                                                                                                                                      |
| Ed    | yes      | Function doc’s first sentence must be no more than 160 chars (wrapped)                                                                                                                                                                                                 |
| Ed    | yes      | Definite vs indefinite articles in docs                                                                                                                                                                                                                                |
| Ed    | REJECTED | Comments must be no wider than 80 characters from their leftmost point                                                                                                                                                                                                 |
| Ed    | yes      | impl LocalTrait for ForeignType order: std first, then non-std but popular, then others in a reasonable order                                                                                                                                                          |
| Ed    | yes      | Use Self as reasonably possible (in function return types, value construction Self{}/Self()/Self)                                                                                                                                                                      |
| Ed    | REJECTED | No use of declarations in format-like macros (the key=value part at the end), prefer local lets                                                                                                                                                                        |
| Ed    | yes      | Use `.map(\|_\| ())` rather than `.map(drop)`                                                                                                                                                                                                                          |
| Ed    | yes      | If a where clause exists, all type bounds must be within it (no mixing)                                                                                                                                                                                                |
| --    | FUTURE   | Clippy lints to turn on: no broken links,                                                                                                                                                                                                                              |
| Ed    | yes      | Prefer scoping to drop (avoid all drop calls?)                                                                                                                                                                                                                         |
| Ed    | REJECTED | Always use clap-derive not the old API                                                                                                                                                                                                                                 |
| --    | FUTURE   | Prefer iterator combinators if infallible, consider avoiding if fallible                                                                                                                                                                                               |
| Ed    | yes      | No method calls on struct literals                                                                                                                                                                                                                                     |
| Ed    | yes      | Scoped mutability—names must match: let x = { let mut x = …; …; x}                                                                                                                                                                                                     |
| Ed    | yes      | Prefer Foo::new() or equivalent to Foo{}                                                                                                                                                                                                                               |
| Ed    | REJECTED | All imports must be in one block (until rustfmt stabilises its many sorted blocks)                                                                                                                                                                                     |
| Ed    | yes      | Imports must be nested (no Java-style repetition)                                                                                                                                                                                                                      |
| Ed    | yes      | Error type must be defined in lib.rs or error.rs in crate root                                                                                                                                                                                                         |
| Ed    | yes      | Always declare Result<T>, this must be done immediately below the error type                                                                                                                                                                                           |
| Ed    | yes      | Use mod.rs, do not use foo.rs next to foo/                                                                                                                                                                                                                             |
| Ed    | FUTURE   | Constructing boxed errors?                                                                                                                                                                                                                                             |
| Ed    | yes      | When using builders, provide a MyType::builder() -> MyTypeBuilder?                                                                                                                                                                                                     |
| Ed    | yes      | MyTypeBuilder must have a .build() -> MyType (duh)                                                                                                                                                                                                                     |
| Ed    | yes      | Prefer builders by value? (avoid awkwardly relying on clone() calls getting optimised away)                                                                                                                                                                            |
| Ed    | yes      | Use explicit Ok(()) explicitly rather than Ok(returns_unit()) (split into two lines)                                                                                                                                                                                   |
| Ed    | yes      | No method calls on (curly brace?) macro results?                                                                                                                                                                                                                       |
| Ed    | REJECTED | Prefer explicit matches (avoid catch-all \_ => () cases)                                                                                                                                                                                                               |
| Ed    | yes      | Start by defining the error states (the error path must be first-class, even if not optimised)                                                                                                                                                                         |
| --    | UNSCOPED | Leverage the type system to uphold invariants and prevent misuse                                                                                                                                                                                                       |
| Ed    | yes      | Minimise use of unsafe and scope as closely as possible to where you actually need unsafe                                                                                                                                                                              |
| --    | FUTURE   | Use rustfmt, but use THESE SPECIFIC (insert here) settings, do not use #[rustfmt::skip]!                                                                                                                                                                               |
| Ed    | yes      | Prefer let xxx: Foo to turbofish to add type annotations on the last thing in a chain (unless really annoying!) i.e. prefer to signpost what the goal of a large chain is, also can be slightly more reliable in some cases                                            |
| Ed    | yes      | For locally-written Serde, use an explicit return and put the helper types at the end of the function, don’t write macro helpers for the helper types! (Type names don’t need to be crate-level unique, just distinct enough but within reason e.g. no type shadowing) |
| Ed    | yes      | Shadowing: two levels of variable shadowing okay, type shadowing is never okay!                                                                                                                                                                                        |
| Ed    | yes      | In mod.rs equivalent, put all mods first, then all pub uses then pub(crate) uses all uses in separate blocks (grouped nicely) then #[cfg(..)] after all                                                                                                                |
| Ed    | OBVIOUS? | All use statements must come before all code                                                                                                                                                                                                                           |
| Ed    | yes      | When read from top-to-bottom, try to make this feel like a tour of the API (guide the reader)                                                                                                                                                                          |
| Ed    | yes      | Redundant type annotations—does clippy prevent let foo: Foo = (...).collect::<Foo>();?                                                                                                                                                                                 |
| Ed    | REJECTED | No .map(Self) at the end of a chain (e.g. to construct Result<Self>                                                                                                                                                                                                    |
| Ed    | yes      | No let x = &expr unless indexing/slicing                                                                                                                                                                                                                               |
| Ed    | yes      | Use `vec![]` rather than Vec::new(), use Foo::new() rather than Foo::with_capacity(0) (for which Foo? yes: Vec, HashMap, BTreeMap)                                                                                                                                     |
| Ed    | yes      | Don’t pattern-match on pointer parameters i.e. don’t `.map(\|&x\| x)`, use `.map(\|x\| *x)`                                                                                                                                                                            |
| Ed    | yes      | Don’t populate tuples with lots of computation. If the line breaks, put things in variables                                                                                                                                                                            |
| Ed    | yes      | Don’t pattern-match in fn functions (like one would expect in closures)                                                                                                                                                                                                |
| Ed    | yes      | Put pub fields first in mixed structs. If this breaks Ord, write it manually                                                                                                                                                                                           |
| Ed    | yes      | Prefer pattern matching to field-access where all fields should be considered                                                                                                                                                                                          |
| Ed    | yes      | Don’t assign or compare &lit (e.g. &0, &""), put the & as close to where needed as possible and deref instead                                                                                                                                                          |
| Ed    | REJECTED | In tests, #[test] (or equivalent) should be the final attribute                                                                                                                                                                                                        |
| Ed    | yes      | If hex is used, make it lowercase                                                                                                                                                                                                                                      |
| Ed    | OBVIOUS  | All public top-level items must be documented                                                                                                                                                                                                                          |
| Ed    | FUTURE   | Put test helper structs at the end of the function they’re used in, after // test structs                                                                                                                                                                              |
| Ed    | yes      | Prefer to call API response structs Response (no shadowing due to independent scopes)                                                                                                                                                                                  |
| Ed    | yes      | Unpack to ensure all fields are considered                                                                                                                                                                                                                             |
| Ed    | yes      | advice use of `.ok().unwrap_or_else(\|\|...)` if `.unwrap_or_else(\|_\| …)` is seen                                                                                                                                                                                    |
| Ed    | UNSCOPED | which files #![...] annotations are placed in                                                                                                                                                                                                                          |
| Ed    | yes      | unused trait method parameters should use let \_ = param rather than named \_foo or annotations                                                                                                                                                                        |
| Ed    | yes      | format! parameters which consist of a single identifier (no pathing) should be folded into the format string                                                                                                                                                           |
| Ed    | yes      | Builder creation: prefer Foo::builder() to FooBuilder::new() (also have no public ::new() on the builder)                                                                                                                                                              |
| Ed    | FUTURE   | Local helper types should be put behind a comment at the end, after an explicit return?                                                                                                                                                                                |
| Ed    | yes      | Prefer `.collect()` to `Foo::from_iter`                                                                                                                                                                                                                                |
| Ed    | yes      | Functions upon which functions depend should come after                                                                                                                                                                                                                |
| Ed    | yes      | Avoid let without values                                                                                                                                                                                                                                               |
| Ed    | yes      | Don’t pattern match in function parameters (closures okay)                                                                                                                                                                                                             |

[dictionary]: https://www.dictionary.com/
[thesaurus]: https://www.thesaurus.com/
