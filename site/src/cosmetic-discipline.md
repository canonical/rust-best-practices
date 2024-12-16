# Cosmetic discipline

## Spacing

Use blank lines semantically, rather than aesthetically.
They should be used consistently, regardless of the size of a section of code, to delimit _strongly associated_ sections.
There are no hard and fast rules for this strong association, but the following heuristics are quite effective.

- If a variable is declared and only used in the block of code which follows it, that declaration and block are strongly associated.
  Do not put a blank line here.
- If a variable is used in multiple blocks of code, not just the one which follows it, that declaration is not strongly associated with the block immediately after it.
  Put a blank line here.
- If a variable is declared and then checked, the declaration and check are strongly associated and must not be separated by a blank line.
  If the check contains more than three lines, the declaration and check start to form their own strongly associated block so require a blank line after.

✅ Do this:

```rust
let x = foo();
if !x.is_valid() {
    return Err(Error::Invalid);
}
println!("{x}");

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

println!("{x}");
let y = baz();
if !y.is_valid() {
    return Err(Error::Invalid);
}

return Ok(y);
```

## Grouping

Don’t interleave unrelated code.
Remember, to a new reader, this will look deliberate and they become confused about how variables relate.
Keep it clean and group together strongly intradependent sections of code.

This is particularly significant where closures are bound to variables—if a closure is defined as such halfway through a function, does not capture anything and then is only used at the end, the reader will have to keep more things in mind for no good reason.
If values are captured, declare such closures close as possible to where they’re needed.
Otherwise, define an `fn` function, rather than a closure with `|| ...`.
Also, consider whether a closure is required at all—although it may be tempting to define helper closures, code may feel cleaner with a simpler, more top-to-bottom flow control pattern.
Logic should always feel clean and be easy to follow.

_The following snippets assume that functions `foo`, `bar` and `baz` are free of side-effects._

✅ Do this:

```rust
let x = foo();
let b = baz();
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

## Hex values

Unless there is an existing convention, hex values should be lowercase as this avoids creating visually-impenetrable rectangles.
By using lowercase, we provide more ‘handles’ for the eye to use.

✅ Do this:

```rust
const SOME_SPECIFIC_IMPORTANT_VALUE: u64 = 0xab5c4d320974a3bc;
```

⚠️ Avoid this:

```rust
const SOME_SPECIFIC_IMPORTANT_VALUE: u64 = 0xAB5C4D320974A3BC;
```
