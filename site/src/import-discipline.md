# Import discipline

## Don’t import all

In general, do not use `*` from a crate.
Consider a source file which does this twice from two different dependencies, making use of items from each.
Now, consider what happens when these crates are updated and some of these items are removed—the compiler will complain of undefined symbols, but will have absolutely no idea where these came from.
Even more concerning is the fact that updates can now cause name-clashes!
By sticking to explicit imports only, we help both ourselves and our future maintainers.

A corollary of this is that preludes, regardless of their initial convenience, should not be used by us in production code.
Nevertheless, they remain a handy tool for others to use when prototyping, so we should still consider creating and exposing them where appropriate.

The only exception to these rules is that in the context of a unit test module, inserting use `super::*` is acceptable as it is a well-defined idiom.

The rule for using `*` from enums is slightly different.
Here, it is acceptable to import `*` to bring all variants of an enum into scope.
However, this should not be done at the top level, only locally to improve the readability of long match statements.
There, they should be placed as close as possible to the relevant match, preferably on the line immediately preceding it.

✅ Do this:

```rust
use some_crate::{SpecificItem1, SpecificItem2};
use some_other_crate::SpecificItem3;

// ...

fn some_fn(some_enum: SomeEnum) -> {
    // ...

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

// ...

fn some_fn(some_enum: SomeEnum) -> {
    // ...

    match {
        Variant1 => {...},
        Variant2 => {...},
    }
}
```

## Import grouping

At the time of writing, stable `rustfmt` does not yet express an opinion on the order of imports, hence for now we must do this ourselves.
To clearly delimit whether an import is from the standard library, from a third party library or our own work, these imports should be split into three blocks, ordered as follows:

- `std`, `core`, `alloc`
- Third party crates
- `self`, `super`, `crate`

Note that this order follows the currently-unstable `rustfmt` option—

```
import_group = "StdExternCrate"
```

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

Excessive repetition harms readability by adding lots of visual noise.
For this reason, avoid Java-style imports where every single imported item gets its own line with a complete and exhaustive path and instead use the cleaner nested syntax.
If a path contains _n_ parts, merge the first _n-1,_ so that only the final part is grouped with its siblings.

✅ Do this:

```rust
use allocative::Allocative;
use derive_more::Display;
use starlark::environment::{FrozenModule, Module};
use starlark::eval::Evaluator;
use starlark::values::{AllocValue, Freeze, ProvidesStaticType, StarlarkValue, ValueLike};
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

## Import `self` explicitly

When importing from a child module `foo`, always `use self::foo` as this avoids future name-clashes with a dependency also called `foo`.

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
