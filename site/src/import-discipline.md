# Import discipline

## Don’t import all

In general, do not use `*` from a crate.
Consider a source file which does this twice from two different dependencies, making use of items from each.
Now, consider what happens when these crates are updated and some of these items are removed—the compiler will complain of undefined symbols, but will have absolutely no idea where these came from.
Even more concerning is the fact that updates can now cause name-clashes!
By sticking to explicit imports only, we help both ourselves and our future maintainers.

A corollary of this is that preludes, regardless of their initial convenience, should not be used by us in production code.
Nevertheless, they remain a handy tool for others to use when prototyping, so we should still consider creating and exposing them where appropriate.

Do not bring enum variants into scope using `*` as this obscures the types and in some cases the fact that an enum is being handled.
If the name of an enum is too long, can't reasonably be edited and the problematic usage is in a small scope, it may be renamed locally using `use ... as ...`.
The new name should be an acronym of the type used, e.g. `TaskStatus` would be shortened to `Ts`.
Due to scoping rules around `use`, these renaming statements should be placed at the top of the function definition which requires it.

The only exception to these rules is that in the context of a unit test module, inserting use `super::*` is acceptable as it is a well-defined idiom.

✅ Do this:

```rust
{{#include snippet_helpers/import_discipline.rs}}
use some_crate::{SpecificItem1, SpecificItem2};
use some_other_crate::SpecificItem3;
use another_crate::SomeEnum;

fn some_fn(some_enum: SomeEnum) -> SomeEnum {
    use SomeEnum as Se;
    match some_enum {
        Se::Variant1 => { /* ... */ }
        Se::Variant2 => { /* ... */ },
    }
    Se::Variant2
}
```

⚠️ Avoid this:

```rust
{{#include snippet_helpers/import_discipline.rs}}
use some_crate::*;
use some_other_crate::prelude::*;
use another_crate::{SomeEnum, SomeEnum::*};

fn some_fn(some_enum: SomeEnum) -> SomeEnum {
    match some_enum {
        Variant1 => { /* ... */ }
        Variant2 => { /* ... */ }
    }
    Variant2
}
```

## Import grouping

At the time of writing, stable `rustfmt` does not yet express an opinion on the order of imports, hence for now we must do this ourselves.
To clearly delimit whether an import is from the standard library, from a third party library or our own work, these imports should be split into three blocks, ordered as follows:

- `std`, `core`, `alloc`
- Third party crates
- `self`, `super`, `crate`

Note that this order follows the currently-unstable `rustfmt` option—

```toml
import_group = "StdExternCrate"
```

✅ Do this:

```rust,ignore
use std::path::PathBuf;

use camino::Utf8PathBuf;
use tokio::runtime::Runtime;

use crate::{Error, Result};
```

⚠️ Avoid this:

```rust,ignore
{{#include snippet_helpers/import_discipline.rs}}
use camino::Utf8PathBuf;
use crate::{Error, Result};
use std::path::PathBuf;
use tokio::runtime::Runtime;
```

## Import form

Excessive repetition harms readability by adding lots of visual noise.
For this reason, avoid Java-style imports where every single imported item gets its own line with a complete and exhaustive path and instead use the cleaner nested syntax.
If a path contains _n_ parts, merge the first _n-1,_ so that only the final part is grouped with its siblings.

✅ Do this:

```rust
{{#include snippet_helpers/import_discipline.rs}}
use allocative::Allocative;
use derive_more::Display;
use starlark::environment::{FrozenModule, Module};
use starlark::eval::Evaluator;
use starlark::values::{AllocValue, Freeze, ProvidesStaticType, StarlarkValue, ValueLike};
use starlark_derive::{starlark_value, NoSerialize, Trace};
```

⚠️ Avoid this:

```rust
{{#include snippet_helpers/import_discipline.rs}}
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

## Import `self` explicitly

When importing from a child module `foo`, always `use self::foo` as this avoids future name-clashes with a dependency also called `foo`.

✅ Do this:

```rust,ignore
mod foo;
mod bar;

pub use self::foo::Foo;
pub use self::bar::Bar;
```

⚠️ Avoid this:

```rust,ignore
mod foo;
mod bar;

pub use foo::Foo;
pub use bar::Bar;
```
