# Function discipline

## No-information returns

Any expression or statement which returns the unit (i.e. `()` such as `println!`) or which never returns anything (i.e. `!` such as `std::process::exit`) should end with a semicolon.

In the case of `()`, any block which ends with a function call which returns `()` relies on the return type of that function to never be changed to return something more useful.
This is a strange dependency which may cause needless compiler errors in future, hence is best avoided.
Using an explicit `;` reinforces the fact that we expect to obtain no information from a particular call.

A similar arguments holds for `!` as we expect to return no information from such a call.

If a function which returns `Result<()>` ends in a function call which also returns `Result<()>`, instead use the `?` operator and an explicit `Ok(())` return.
The intuition here is that in the expected case on the golden path, we expect no information to be returned, hence we should make our code reinforce this fact.

If a `match` statement is expected to return `()`, then it is being used as a control-flow structure.
Therefore, do-nothing `match` branches should be written as `.. => {}` rather than `.. => ()`.

✅ Do this:

```rust,ignore
fn setup_foo(&self) -> Result<()> {
    match self.foo_type {
        FooType::A => {}
        ...
    }
    self.setup_bar()?;
    Ok(())
}
```

⚠️ Avoid this:

```rust,ignore
fn setup_foo(&self) -> Result<()> {
    match self.foo_type {
        FooType::A => ()
        ...
    }
    self.setup_bar()
}
```

## Hide generic type parameters

Generic type parameters add complication to an API.
Where possible, hide generic parameters either through elision, syntactic sugar (`impl Trait`) or by leaving them unbound.

On `impl` blocks, only introduce strictly-necessary type-constraints.
Not only will this reduce the cognitive overhead of understanding large blocks, it will also help make code more easily applicable in new scenarios.

On functions, if a generic _lifetime_ parameter can be [elided][elision], it should be by using `'_`.
If a generic _type_ parameter is only used once and isn’t too complicated, use `impl Trait` to hide it.

Note that although it is possible to omit the unnamed lifetime (i.e. it may be possible to write `MyRef<'_>` as `MyRef`), this should never be done.
A type without lifetime parameters looks completely self-contained and hence as though may be freely passed around.
If a lifetime is present, _always_ communicate that fact (i.e. always prefer `MyRef<'_>` to `MyRef`).

✅ Do this:

```rust,ignore
fn transmit(tx: impl Transmitter<'_>, message: &[u8]) -> Result<()> { ... }
```

⚠️ Avoid this:

```rust,ignore
fn transmit<'a, T: Transmitter<’a>>(tx: T, message: &[u8]) -> Result<()> { ... }
```

## Unused parameters in default implementations

Occasionally, when a default trait function implementation is provided, not all of its parameters are used.
To avoid a compiler warning, explicitly add `let _ = unused_param;` lines until all these warnings are removed.

Briefly considering other approaches, it is possible to rename the parameter to something like `_unused_param`, however this will appear in docs and look dishevelled.
It is possible to use `#[allow(unused_variables)]` to suppress all unused-parameter warnings for the given function, however this will also affect the parameters we _do_ expect to use, possibly causing issues later.
Similarly, we could individually annotate each unused parameter, however this would make the overall signature much more difficult to read.

✅ Do this:

```rust,ignore
trait CustomScriptValue<'v> {
    fn at(&self, index: Value<'v>) -> Result<Value<'v>> {
        let _ = index;
        Err(Error::Unsupported { .. })
    }
}
```

⚠️ Avoid this:

```rust,ignore
trait CustomScriptValue<'v> {
    fn at(&self, _index: Value<'v>) -> Result<Value<'v>> {
        Err(Error::Unsupported { .. })
    }

    // OR

    #[allow(unused_variables)]
    fn at(&self, index: Value<'v>) -> Result<Value<'v>> {
        Err(Error::Unsupported { .. })
    }
}
```

## Builder visibility

The type `MyTypeBuilder` should not have a public constructor.
In typical usage, users of `MyType` shouldn’t need to import `MyTypeBuilder`, it should be a seamless part of `MyType`.

✅ Do this:

```rust,ignore
use crate::Foo;

let foo = Foo::builder()
    .bar(bar)
    // ...
    .build()?;
```

⚠️ Avoid this:

```rust,ignore
use crate::{Foo, FooBuilder}

let foo = FooBuilder::new()
    .bar(bar)
    // ...
    .build()?;
```

## Builder ownership

In Rust, there are two forms of the builder pattern, depending on the receiver type used.
Consider the following builder—

```rust,ignore
let frobnicator = Frobnicator::builder()
    .foo("foo")
    .bar("bar")
    .build()?;
```

The methods `foo`, `bar` and `build` can either take ownership of `self` or take `&mut self` by value.

If `self` is used, it encourages a simple flow of data, with values being neatly moved from one place to another.
This should be the default ownership model.

If `&mut self` is used, it makes conditional use of each of the builder methods simpler, however either the builder must be bound to a variable or all contained data must be cloned during `.build()`.
Although the optimiser may remove some unnecessary cloning, this should not be relied upon.
Besides, if the `.build` method takes a reference to the builder—allowing it to be called multiple times—this feels more like a _factory_ than a _builder._

[elision]: https://doc.rust-lang.org/nomicon/lifetime-elision.html
