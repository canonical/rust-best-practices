# Code discipline

## When to use `Self`

Use `Self` wherever possible to reduce the number of types which the reader must keep in mind as instead of needing to remember an extra type which may or may not be important, the reader is instead reminded that the current code strongly relates to the `impl` block it is contained in.
By maximising the use of `Self` it also highlights where it _isn’t_ possible to use it, for example when the return type of a function has a slightly different set of generic parameters.

✅ Do this:

```rust
{{#include snippet_helpers/code_discipline.rs}}
# use std::cmp::Ordering;
# #[derive(Eq, PartialEq)]
# struct Node;
impl Node {
    pub fn new(parent: &Self) -> Self {
        // ...
#       Node
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
#       Some(Ordering::Less)
    }
}
```

⚠️ Avoid this:

```rust
{{#include snippet_helpers/code_discipline.rs}}
# use std::cmp::Ordering;
# #[derive(Eq, PartialEq)]
# struct Node;
impl Node {
    pub fn new(parent: &Node) -> Node {
        // ...
#       Node
    }
}

impl PartialOrd<Node> for Node { // NB: Rhs=Self is also the default for PartialOrd.
    fn partial_cmp(&self, other: &Node) -> Option<Ordering> {
        // ...
#       Some(Ordering::Less)
    }
}
```

## When not to use `Self`

Do not use `Self` when constructing associated types.

Traits often include type fields as part of the interface they describe.
These types may be referred to with `Self::AssociatedType`.
Do not use these to construct values as it prevents the next reader from understanding which type is in use and which fields and methods are available.
Use these `Self::*` types to define interfaces, but use concrete types to define implementations.

The only exception is for trait items which return a `Result<_, Self::Err>`, where `Err` is set to the crate’s `Error` type.
In this case, it is okay to use the crate’s `Result` type alias instead.

✅ Do this:

```rust
{{#include snippet_helpers/code_discipline.rs}}
# type MyType = Arbitrary;
# struct SomeStruct {
#     some: (),
#     fields: (),
# }
impl Responder for MyType {
    type Response = SomeStruct;
    type Err = Error;

    fn respond(&self, _input: Input) -> Result<Self::Response> {
        let some = ();
        let fields = ();
        Ok(SomeStruct{
            some,
            fields,
        })
    }
}
```

⚠️ Avoid this:

```rust
{{#include snippet_helpers/code_discipline.rs}}
# type MyType = Arbitrary;
# struct SomeStruct {
#     some: (),
#     fields: (),
# }
# {
# type Result<T, E> = std::result::Result<T, E>;
impl Responder for MyType {
    type Response = SomeStruct;
    type Err = Error;

    fn respond(&self, _input: Input) -> Result<SomeStruct, Error> {
        let some = ();
        let fields = ();
        Ok(Self::Response {
            some,
            fields,
        })
    }
}
# }
```

## Struct population

Structs, like tuples, provide an excellent way to group together related information of different types.
Unlike tuples however, they force values to be named, thus avoiding any forgettable-ordering problems.
When each field is given a value, there are three possible forms to choose from:

- The field has its value moved in from a variable with the same name,
- The field has its value moved in from a variable with a different name,
- The field takes its value from the result of some computation.

The first form is the cleanest for struct population as not only does it use the least characters, it also implies that the concepts being handled are very compatible and that there is a clear flow of data present.
This makes the code easier to read, hence wherever reasonable, we should aim for this form.

The second form is acceptable if the name of the field and the name of the value are similar, for example one name being a sub-string of the other.
If it is not possible to tune naming to make this true, then this is a sign of messy concepts at play and hence that a refactor is needed.

The third form can be the most problematic as large computations often draw too much attention, effectively hiding both smaller computations and the other two field-forms above.
Drawing attention in this way indicates that the code awkwardly interleaves separate actions—the same section of code is both populating a struct and doing detailed computation to determine its contents.
To avoid some fields sticking out like a sore thumb, either all fields of a struct should be computed or none should be.
In case of a mix, refactor computations into a new `let` declarations, matching the order of the fields as closely as possible and use the first form above.
This avoids the reader having to wade through alphabet soup.

The order in which fields are populated must be the same as the type declaration.
Each line which populates a field in a given struct must be independent.
If there is a dependency between field declarations, for example if some shared state is mutated during their construction, use `let` declarations and the first form instead.

✅ Do this:

```rust
{{#include snippet_helpers/code_discipline.rs}}
struct Entry<K, V> {
    id: u64,
    key: K,
    value: V,
    pretty_date_modified: String,
}

# type V = Arbitrary;
# impl<K: Clone> Entry<K, V> {
fn get_entry(&self, key: &K) -> Result<Entry<K, V>> {
    let key = key.clone();
    let id = self.id_of(&key)?;
    let value = self.get(&key)?;
    let pretty_date_modified = self.date_modified(&key)?
        .format_as("yyyy-MM-dd@hh:mm:ss");
    Ok(Entry {
        id,
        key,
        value,
        pretty_date_modified,
    })
}
#
#   fn get(&self, _: &K) -> Result<V> {
#       Ok(Arbitrary)
#   }
#
#   fn id_of(&self, _: &K) -> Result<u64> {
#       Ok(0)
#   }
#
#   fn date_modified(&self, _: &K) -> Result<Arbitrary> {
#       Ok(Arbitrary)
#   }
# }
```

⚠️ Avoid this:

```rust
{{#include snippet_helpers/code_discipline.rs}}
struct Entry<K, V> {
    id: u64,
    key: K,
    value: V,
    pretty_date_modified: String,
}

# type V = Arbitrary;
# impl<K: Clone> Entry<K, V> {
fn get_entry(&self, key: &K) -> Result<Entry<K, V>> {
    let value_stored_at_key = self.get(key)?;
    Ok(Entry {
        id: self.id_of(key)?,
        pretty_date_modified: self.date_modified(key)?
            .format_as("yyyy-MM-dd@hh:mm:ss"),
        key: key.clone(),
        value: value_stored_at_key,
    })
}
#
#   fn get(&self, _: &K) -> Result<V> {
#       Ok(Arbitrary)
#   }
#
#   fn id_of(&self, _: &K) -> Result<u64> {
#       Ok(0)
#   }
#
#   fn date_modified(&self, _: &K) -> Result<Arbitrary> {
#       Ok(Arbitrary)
#   }
# }
```

## Tuple population

Tuples are most easily read when they are short as once line-breaks occur, the structure being created gets harder to discern.
Keep things visually simple—if the formatter chooses to break tuple population into multiple lines, instead introduce new `let` declarations to move computation away from the tuple population.

✅ Do this:

```rust,ignore
{{#include snippet_helpers/code_discipline.rs}}
# fn snippet() -> Result<(Arbitrary, Arbitrary)> {
let key = some_long_computation()?
    .something_else()
    .another_thing();
let value = some_other_long_computation()
    .chained_with_something_else()?;
# let ret =
(key, value)
# ;
# Ok(ret)
# }
```

⚠️ Avoid this:

```rust
{{#include snippet_helpers/code_discipline.rs}}
# fn snippet() -> Result<(Arbitrary, Arbitrary)> {
# let ret =
(
    some_long_computation()?
        .something_else()
        .another_thing(),
    some_other_long_computation()
        .chained_with_something_else()?,
)
# ;
# Ok(ret)
# }
```

## Prefer `collect` when interacting with `FromIterator`

The `FromIterator` trait defines a method `from_iter` which is called by `Iterator::collect`.
We therefore have two methods of collecting into an iterator, `Foo::from_iter` and `collect()` with appropriate type bounds.
Prefer the latter as this makes the order of operations the same as what is read from top to bottom.

✅ Do this:

```rust
# let collection = [0];
# let filter_closure = |_: &i32| true;
let my_vec: Vec<_> = collection.into_iter()
    .filter(filter_closure)
    .collect();
```

⚠️ Avoid this:

```rust
# let collection = [0];
# let filter_closure = |_: &i32| true;
let my_vec = Vec::from_iter(
    collection.into_iter()
        .filter(filter_closure)
);
```

## Empty `Vec` construction

To construct an empty `Vec`, we have three options: `vec![]`, `Vec::new()` and `Vec::with_capacity(_)`.
If the size of the `Vec` resulting from an operation can be reasonably estimated, prefer `Vec::with_capacity` as this will avoid unnecessary reallocations.
Otherwise, if a zero-sized `Vec` is required, use `Vec::new()` as this indicates most clearly that no operation is being performed as the docs for `Vec::new` guarantee no allocations, but those for `vec!` do not.
Consider also that `vec![expr; n]` where `n` is zero will still evaluate (and then immediately drop) `expr`.

✅ Do this:

```rust
let my_vec = Vec::new();
# let my_vec_constraint: Vec<i32> = my_vec;
```

⚠️ Avoid this:

```rust
let my_vec = vec![];
# let my_vec_constraint: Vec<i32> = my_vec;
let my_vec = Vec::with_capacity(0);
# let my_vec_constraint: Vec<i32> = my_vec;
```

## Avoid loosely-scoped `let mut`

In many cases, mutability is used to create a given structure which is then used immutably for the remainder of its lifetime.
Whenever this happens, scope the mutable declarations to just where they are needed, thus forcing a compiler error if this condition is broken in future.
Doing this also makes code simpler to read as there are fewer things which can mutate at any one point.

```rust
{{#include snippet_helpers/code_discipline.rs}}
# type MyStructure = Arbitrary;
let my_structure = {
    let mut my_structure = MyStructure::new();
    // Mutate `my_structure` as required to construct it.
    my_structure
};
```

For greatest clarity, make sure the name of the outer (immutable) and inner (mutable) declarations have the same name, here `my_structure`.

If mutability is to retain some state whilst iterating through a structure, consider using a functional style instead.
As a simple example, if presented with the following imperative code to count the number of spaces in a string

```rust
# let my_string = "";
let mut num_spaces = 0;
for c in my_string.chars() {
    if c == ' ' {
        num_spaces += 1;
    }
}
```

Consider instead, using the functional style to avoid the mutation—

```rust
# let my_string = "";
let num_spaces = my_string.chars()
    .filter(|c| *c == ' ')
    .count();
```

## Avoid unassigned `let` declarations

Let-declarations without values indicate an odd flow of data through a function.
Instead, prefer to return the required value from the block which computes it.

✅ Do this:

```rust
{{#include snippet_helpers/code_discipline.rs}}
# fn snippet() -> Result<()> {
# let result = Result::Ok(());
let message = if result.is_ok() {
    "success!"
} else {
    "failed!"
};

# fn exec_web_request() -> Result<Message> { Ok(Message) }
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
        return Err(Error::NetworkUnavailable);
    }
};
# Ok(())
# }
```

⚠️ Avoid this:

```rust
{{#include snippet_helpers/code_discipline.rs}}
# fn snippet() -> Result<()> {
# let result = Result::Ok(());
let message;
if result.is_ok() {
    message = "success!";
} else {
    message = "failed!";
}

# fn exec_web_request() -> Result<Message> { Ok(Message) }
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
        return Err(Error::NetworkUnavailable);
    }
}
# Ok(())
# }
```

## Reference scope

In many cases, the compiler is smart enough to create temporary storage locations to store variables which are given the value `&expr`, however, when these are passed to functions, it becomes slightly harder to follow which type is being used, especially when handling `&T` where `T` is `!Copy`.
In this case, it is only a single character in the variable declaration, possibly many lines away which shows that the value `T` is not being moved, only its reference.

Instead of relying on temporary storage locations, store the value explicitly and take a reference where needed.
This way, the transfer of ownership is much more explicit.
As a rule of thumb, only use `&` at the start of the value of a `let` declaration when either indexing or slicing.

✅ Do this:

```rust
# fn from_func() {}
# fn other_func(_: &()) {}
let foo = from_func();
other_func(&foo);
```

⚠️ Avoid this:

```rust
# fn from_func() {}
# fn other_func(_: &()) {}
let foo = &from_func();
other_func(foo);
```

## Shadowing

When used in moderation, shadowing provides an excellent way to manipulate data or change its type whilst still highlighting that the ‘same’ data is being processed, however, too many levels of shadowing quickly makes code harder to follow.

Let us consider name-shadowing with variables, for which there are three cases:

1. Shadowing in contained scopes, such as `if let Some(x) = x { ... }`
2. Shadowing in the same scope with the same type, such as `let x = ...; let x = some_transform(x)`
3. Shadowing in the same scope with different types, such as `let x: T = x.into()`

In the first case when shadowing with different scopes, use at most one level of shadowing, for example when pattern matching enum variants—

```rust
let foo = Some(());
if let Some(foo) = foo {
    // The outer `foo` is now shadowed.
    // The inner `foo` should not be shadowed.
}
```

In the second case when shadowing in the same scope with the same type, there is no restriction placed on this and it may be done as many times as necessary.
However, if this is being used to effectively mutate a value during construction with no other values being affected, instead use the scoped-mutability pattern—

```rust
{{#include snippet_helpers/code_discipline.rs}}
# type MyThing = Arbitrary;
let my_thing = {
    let mut my_thing = MyThing::new();
    // Mutate `my_thing` to construct it...
    my_thing
};
```

In the final case, when shadowing in the same but changing types (e.g. in a conversion method), shadowing can be done at most once per variable.
This pattern is commonly seen in conversion functions—those which take ownership of their sole parameter and convert it into another type.

```rust
# struct Store<S> {
#     some_field: (),
#     state: S,
# }
# struct New;
# struct Inited;
impl Store<New> {
    fn init(self) -> Store<Inited> {
        let Self { some_field, .. } = self;
        let some_field = some_field.into();
        // ...
        Store {
            some_field,
            // ...
#           state: Inited,
        }
    }
}
```

Now let us consider name-shadowing with types.
Do not name-shadow with types.
A key benefit of Rust’s strong type system is that for any valid program, each handled value has exactly one possible type and representation, which makes it easier for the programmer to understand what is happening.
If we shadow types, the reader may think they understand what they are reading until they find something which seems impossible, because they have been quietly fooled by a definition possibly many lines away which they did not read.

## Generic type parameter constraints

Generic type parameter constraints should be grouped in the same place.
If some constraints are complex enough to be placed into a `where` clause, then all constraints should be moved from angle-bracket declarations into the `where` clause also.

If constraints are declared in angle brackets and that declaration is longer than 30 characters, move all constraints into a `where` clause.

These rules ensure that the reader cannot miss any constraints.

✅ Do this:

```rust
# struct IterWrapper<I>(std::marker::PhantomData<I>);
impl<'a, 'b, I, T> IterWrapper<I>
    where
        'b: 'a,
        I: Iterator<Item=T> + 'a,
        T: 'b,
{ /* ... */ }
```

⚠️ Avoid this:

```rust
# struct IterWrapper<I>(std::marker::PhantomData<I>);
impl<'a, 'b: 'a, I: Iterator<Item=T> + 'a, T> IterWrapper<I>
    where
        T: 'b
{ /* ... */ }
```

## Type annotations

The compiler’s type inference is usually very good, but sometimes, it needs a little extra information.
When adding the necessary annotations, we must be mindful of how much information we provide and where that information is provided.

Provide only the minimum amount of information required to help the compiler.
If the compiler does not complain if a type annotation is removed, then it should be removed.
Always make note of the return type of the function currently being written as often this provides enough information to make many possible type annotations obselete.

Let’s briefly consider collecting an iterator of `SomeLongType<...>` into a `Vec`.
The compiler will complain that it does not know which collection type to collect into and although explicitly annotating `Vec<SomeLongType<...>>` works, the information the compiler needs is just that a `Vec` is being made, so it is better to use just `Vec<_>`.
This has three benefits:

1. The code is cleaner as unnecessary information does not draw the eye
2. The code is more maintainable as the types we handle are allowed to change (the necessary properties are still implicitly enforced at function boundaries)
3. The coder’s wrists ache less

In a function implementation, Rust provides three main ways to insert annotations:

1. On `let`/`static`/`const` declarations, e.g. `let foo: Type`
2. On functions using the turbofish, e.g. `func::<T>()`
3. On trait items using the fully-qualified syntax, e.g. `<value as Trait>::trait_item`

The above the order of preference in which these should be used; we shall explore each method presently.

The best place to put annotations is on `let`/`static`/`const` declarations, as this not only makes it clearer what the values are being, but also if such a declaration involves a long chain of calls spanning many lines, it also makes it more obvious what’s being worked towards.
This pattern is also the most flexible.

Next, if type information must be given in the middle of the expression, the turbofish may be used (`::<>`).
One place to be careful though is if the turbofish is required on a `collect` in the middle of an expression, there is often a cleaner and more efficient way to achieve the same thing.

The tool of last-resort is the fully-qualified syntax which disambiguates between same-name items on the same type which come from different namespaces in scope.
If an API you are making can cause an abundance of name-clashes, your consumers will be unhappy.
In this case, refactor it as best you can.

If a type parameter seems necessary as a variable name is not sufficiently descriptive, improve the name.

✅ Do this:

```rust
# let foo = [0];
# let filter_closure = |_: &&i32| true;
# let map_closure = |_| ();
let some_meaningful_var_name: Vec<_> = foo.iter()
    .filter(filter_closure)
    .map(map_closure)
    // ...
    .collect();
```

⚠️ Avoid this:

```rust
# fn snippet<'lifetimes, With, Generics>() {
# struct And<'l>(std::marker::PhantomData<&'l ()>);
# let foo = [0];
# let filter_closure = |_: &&i32| true;
# let map_closure = |x| SomeExtremelyLongType(std::marker::PhantomData);
# struct SomeExtremelyLongType<W, G, A>(std::marker::PhantomData<(W, G, A)>);
let x = foo.iter()
    .filter(filter_closure)
    .map(map_closure)
    // ...
    .collect::<Vec<SomeExtremelyLongType<With, Generics, And<'lifetimes>>>>();
# }
```

## Avoid explicit `drop` calls

If a value must be dropped early, create a new scope with curly braces rather than using an explicit `drop` call.
This will highlight the lifetimes of the values in use whilst also avoiding the reader missing the `drop` call, which may be nestled among many busy-looking lines of code.

The `drop` function should not be used to discard values in call chains.
If a single value is to be ignored, use the ‘toilet operator,’ `|_| ()`, i.e.
a closure which takes one argument, ignores it and returns a unit.
This form is consistent with similar closures which ignore parts of their inputs, for example when extracting the key from a key-value pair—

```rust
# [(1, 2)].iter()
    .map(|(key, _)| key)
# ;
```

There are two exceptions to this.

If `|_| ...` is used and the ignored parameter is an `Error`, we should highlight that an error is intentionally being ignored, for example by using `.ok()` on a `Result` being handled.

If converting from some `Result<T>` to a `Result<()>` at the end of the last expression of a function, instead of the ignore marker, use the `?` operator and an explicit `Ok(())`.
This highlights that we care only about side-effects, and that no information is returned in the successful case.

✅ Do this:

```rust
{{#include snippet_helpers/code_discipline.rs}}
# use std::fs::OpenOptions;
# use std::io::Write;
# impl Log {
async fn log(&self, message: String) -> Result<()> {
    {
        let mut file = OpenOptions::new()
            .append(true)
            .open(&self.log_file_path)?;
        file.write_all(message.as_bytes())?;
    }
    self.transmit_log(&message).await?;
    Ok(())
}
# }
```

⚠️ Avoid this:

```rust
{{#include snippet_helpers/code_discipline.rs}}
# use std::fs::OpenOptions;
# use std::io::Write;
# impl Log {
async fn log(&self, message: String) -> Result<()> {
    let mut file = OpenOptions::new()
        .append(true)
        .open(&self.log_file_path)?;
    file.write_all(message.as_bytes())?;
    drop(file);
    self.transmit_log(&message)
        .await
        .map(drop)
}
# }
```

## Constructors vs. structs with all fields public

For structs created by a consumer and which represent an ‘object’ in the code’s model (i.e. those which aren’t just used to transfer data between components), prefer to expose constructor functions or a builder rather than exposing public fields.
There are several benefits here:

- They format more nicely in call chains
- They allow parameter type conversions to occur implicitly
- They allow some fields to be computed in terms of others using implementation-specific details

Structs with all-public fields cannot benefit from any of the above and moreover, if it is later decided that any of these properties is beneficial, we face either a breaking change to fix it or extra complication to work around it.

For structs created by the consumer which just transfer data (e.g. `FooConfig` may be passed to `fn foo`), if there is a reasonable `Default` implementation, prefer to make all fields public to reduce boilerplate.
To defend against the future addition of new fields causing breaking changes, consider marking the struct as `#[non_exhaustive]`.
If there is no reasonable `Default` implementation, use a builder instead.

## Method-specific data-structures

Avoid namespace pollution by putting helper types into the most local scope they may be reasonably defined in.
Further, if common patterns start to arise, make sure to use standard names for each type, for example `Test` for test-specific builder-pattern test cases or `Message`/`Response` for serde-structs which represent a foreign API.
Note that by using predictable names in very predictable, local-scale patterns, the reader is able to safely forget details unimportant to them.
When helper types are local to a function, place the definitions at the bottom, below a marker comment such as `// test types` or `// serde types`.
Such marker comments let the reader know that unless they wish to know the finer details about how a test works or how a foreign API is handled, they do not need to read any further.

This pattern is particularly useful to significantly reduce the blast-radius of incoming remote API changes by closely matching the expected form with a set of local (de)serialization types independent to those understood internally by our crate.
Let’s say we have a function called `get_image_info`, which makes a web-request to get information associated with given container image name (e.g.
author, description, latest version).
To nicely transfer data from some remote format into one we govern, say `ImageInfo`, add an explicit `return` at the end of `get_image_info` and _below_ this, create a new type called `Response`, which implements `Deserialize`.
Add a comment which says `// serde types.` to let the reader know that everything beyond this point only relates to modelling the remote API—thus saving them time as they will likely only care about these details if something is broken.
Add as many new local types as are necessary to maintain a 1:1 relationship between Rust types and the remote’s format—

```rust,ignore
async fn get_image_info(&self, name: &str) -> Result<ImageInfo> {
    let response: Response = serde_json::from_str(get_response().await?.text());
    let info = ImageInfo {
        name: name.to_owned(),
        version: response.metadata.version,
        authors: response.metadata.authors,
        latest_release: response.releases.last()
            .map(|release| ...),
    }
    return Ok(info);

    // serde types.
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

As a rule of thumb, `serde` annotations should not be present on the types used in the core of a crate, where the content of those types is taken entirely from a remote API or is destined to be sent to one.
Remote APIs should not govern internal representation.

## Method calls on closing brackets and braces

Expressions which end in a `}` such as control structures and struct literals should not have methods called upon them as the formatter moves method calls onto the line below.
This adds an unwelcome surprise as the scope of what the reader is currently looking at will appear to increase, adding to cognitive load and potential confusion.
To avoid this, use a binding (`let some_var = ...; some_var.foo()`).

When designing APIs, if a public struct will be filled by consumers for the purpose of calling a single method on it, consider instead reversing the dependency by using a free function which takes the struct as its first parameter in the fashion of a config struct.

Expressions which end in a `)` or `]` follow the same rule unless that expression is quite short.

✅ Do this:

```rust
{{#include snippet_helpers/code_discipline.rs}}
# fn snippet() -> Result<()> {
# struct FooConfig {
#     bar: &'static str,
#     baz: &'static str,
# }
# fn foo(_: FooConfig) -> Result<()> {
#     Ok(())
# }
foo(FooConfig {
    bar: "asdf",
    baz: "fdsa",
})?;

# let some_condition = true;
# let value_a = "";
# let value_b = "";
let value = if some_condition {
    value_a
} else {
    value_b
};
value.to_string()
# ;

// ...
# ['?'].iter()
    .filter(|c| {
        let exceptions = [ 'a', 'b', 'c', 'd' ];
        !exceptions.contains(c)
    })
# ;
// ...
# Ok(())
# }
```

⚠️ Avoid this:

```rust
{{#include snippet_helpers/code_discipline.rs}}
# fn snippet() -> Result<()> {
# struct Foo {
#     bar: &'static str,
#     baz: &'static str,
# }
# impl Foo {
#     fn do_thing(self) -> Result<()> {
#         Ok(())
#     }
# }
Foo {
    bar: "asdf",
    baz: "fdsa",
}
.do_thing()?;

# let some_condition = true;
# let value_a = "";
# let value_b = "";
if some_condition {
    value_a
} else {
    value_b
}
.to_string()
# ;

// ...
# ['?'].iter()
    .filter(|c| ![ 'a', 'b', 'c', 'd' ].contains(c))
# ;
// ...
# Ok(())
# }
```

## Format arg inlining

Arguments to `format!`-like macros should aim to be as similar as possible to the string they are intended to produce.
Whenever a single variable is used in a format argument, it should be inlined to avoid the reader needing to dart back and forth between the format string and its arguments (both of which may stretch over multiple lines).

_NB: older Rust versions may not support this syntax._

✅ Do this:

```rust
# let path = "";
# let file = "";
format!("{path}/{file}")
# ;
```

⚠️ Avoid this:

```rust
# let path = "";
# let file = "";
format!("{}/{}", path, file)
# ;
```
