# Pattern matching discipline

## Exhaustively match to draw attention

Pattern matching is an excellent way to ensure that all items of data in internal structures have been considered, not only by the author of the current change, but also by the authors of any future changes.
When using internal interfaces, always consider using pattern-matching to force useful compiler errors in case important, possibly new, parts of a structure haven’t been considered.
This in turn will draw the attention of the next maintainer and help them correctly do what they need.

✅ Do this:

```rust,ignore
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

```rust,ignore
impl Ord for MyStruct {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.my, self.thing, self.with, self.some)
            .cmp(&(other.my, other.type, other.with, other.some))
    }
}
```

## Don’t pattern-match pointers

It is possible to pattern-match the pointer to a `Copy` type to obtain the value at the other end.
Although it may seem convenient, it ultimately harms readability—it is clearer to explicitly dereference the pointer we are given.

✅ Do this:

```rust,ignore
    .map(|x| *x)
```

⚠️ Avoid this:

```rust,ignore
    .map(|&x| x)
```

## Avoid numeric tuple-indexing

Although sometimes a handy shorthand, indexing tuples with `.0`, `.1` etc. deprives us of the opportunity to insert a good name in the same way that field-access on a struct would.
Instead, prefer to use pattern-matching to give human-friendly names to the data being handled.
Note that this advice does not apply in the `impl` blocks of newtype-pattern structs, i.e. tuple-structs with a single element (commonly used for wrapper types).

✅ Do this:

```rust,ignore
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

```rust,ignore
fn line_through(point1: (f64, f64), point2: (f64, f64)) -> Line {
	let gradient = (point2.1 - point1.1) / (point2.0 - point1.0);
	let y_intercept = point1.1 - gradient * point1.0;
	Line {
		gradient,
		y_intercept,
	}
}
```

## Pattern-matched parameters

Using pattern matching in `fn` parameters adds extra noise to a function’s signature by duplicating definitions held elsewhere.
Indeed, the fact that a particular parameter is to be pattern-matched inside of the function is not important to the user—it is an unwelcome implementation detail and should be hidden as such.

If parameters are to be unpacked, do this at the first line of a particular function.

Note that this guidance does not apply to closures, which are commonly used as small, locally-scoped helper functions, whose types are inferred.

✅ Do this:

```rust,ignore
impl Server {
    fn new(config: ServerConfig) -> Result<Self> {
        let Config { db_path, working_path } = config;
        // ...
    }
}
```

⚠️ Avoid this:

```rust,ignore
impl Server {
    fn new(ServerConfig { db_path, working_path }: ServerConfig) -> Result<Self> {
        // ...
    }
}
```
