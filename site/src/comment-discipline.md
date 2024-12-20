# Comment discipline

## First doc-sentence form

When wrapped, the first sentence of a doc comment should be at most two lines.
It should clearly and concisely explain the whole of the golden path of a function.
After reading this first sentence, it should be clear _when_ to use the given function/type—don’t fall into the trap of just explaining _what_ the given item does.

✅ Do this:

```rust
{{#include snippet_helpers/comment_discipline.rs}}
# impl Arbitrary {
/// This function reports an increase in the number of steps taken by this
/// thread.
fn add_steps(&self, delta: i64) -> Result<()> {
    /* ... */
#   Ok(())
}
# }
```

⚠️ Avoid this:

```rust
{{#include snippet_helpers/comment_discipline.rs}}
# impl Arbitrary {
/// This function adds a given delta to the current step counter.
fn add_steps(&self, delta: i64) -> Result<()> {
    /* ... */
#   Ok(())
}
# }
```

## Definite vs. Indefinite articles

When referring to parameters, be concrete and specific.
Where possible, refer parameters by their name and if an article must be used (i.e. ‘a’/‘an’ and ‘the’), always prefer the definite article, ‘the.’
Leave no room for ambiguity and hence misunderstanding.

✅ Do this:

```rust
{{#include snippet_helpers/comment_discipline.rs}}
# impl Arbitrary {
/// Increments this counter by the given `delta`.
fn incr_by(&self, delta: u64) -> Result<()> {
    /* ... */
#   Ok(())
}
# }
```

⚠️ Avoid this:

```rust
{{#include snippet_helpers/comment_discipline.rs}}
# impl Arbitrary {
/// Increments a counter by a given amount.
fn incr_by(&self, delta: u64) -> Result<()> {
    /* ... */
#   Ok(())
}
# }
```
