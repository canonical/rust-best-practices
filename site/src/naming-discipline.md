# Naming discipline

## Name content

Naming is one of the three hardest problems in programming (along with off-by-one errors).
Every variable, every function, every type and every concept requires a good name which fits into a good naming scheme.
There is no one optimal way to come up with a good name, however when attempting to do so, the first place to look is for similar names in your project and to try to mimic these.
This should result in a name which intuitively feels like it belongs among the rest of the code.
Even doing this has its pitfalls, however, so ideally your name should:

- **say what it means**—make the name fit conceptually into the surrounding context.
  If a reader sees `fn is_in(a: &str, b: &str)`, the order is not as obvious as if they were to see `fn is_in(haystack: &str, needle: &str)`.
- **have a consistent word order**—inconsistency makes an API look dishevelled, unplanned and hence unprofessional.
  If the rest of the API uses `verb_noun` then unless there is a very good reason not to, the next function should be of the form `verb_noun`.
- **be concise**—a long name can almost always be shortened.
  More characters implies a need to disambiguate, so if no such need exists, reduce the cognitive load on the next reader by reducing the amount they must read.
  Of course, don’t take this too far—the next reader must not be expected to look elsewhere to understand the full meaning of a name, as may occur if nonstandard acronyms or abbreviations are used.
- **comprise simple words**—a long word can often be replaced by a shorter one.
  A concise name will comprise the smallest list of the smallest words which do not lose the subject’s meaning.
  Remember: [thesaurus.com][thesaurus] and [dictionary.com][dictionary] are your friends!
- **comprise correct words**—if there is any disagreement over the implications of chosen words, then there will be some reader who gets the wrong idea.
  It’s better to spend more time discussing internally than to confuse a user.
  (Example: we once had a 20-minute discussion on the choice between `NOT_SAFE` vs `UNSAFE` as an empty value for a set of safety flags, where each flag had a name like `FOO_SAFE`.)
- **be unified**—there should be one and only one name for concepts used.
  If more are used haphazardly, it implies a difference where there is none and thus muddies the water.
- **avoid including types**—type names should be omitted unless required to discriminate between two variables of different types which roughly hold the same value.
  Some examples: in a finder function `needleStr` and `haystackStr` can be more concisely expressed as `needle` and `haystack`.

Canonical policy dictates that names should use UK spelling and not US or other spelling.

Good names with the help of concise doc comments do a good job of explaining a good API.
However, if after much consideration, there don’t seem to be any good names, this is likely caused by the API not being good.
If an API cannot be easily and intuitively explained, it is not a good API and it’s time for a refactor.

A good, semantically and behaviorally consistent API hidden behind a layer of bad naming is hard to distinguish from a bad API.
Time spent getting the right naming will pay off.

Great care should be taken over all names, but extreme care should be taken over publicly exposed ones.
These names do not have the luxury of being able to be tweaked without consequence later—any appreciation for a slightly better name an external user may have will be completely overshadowed by their irritation of having to deal with a breaking change.

## Pattern match variable naming

To reduce cognitive load, pattern-matched variables should be named consistently, that is, the new variable must be either:

- The same as the variable/field it comes from
- The first letter of the variable/field it comes from

When matching structs and struct-like enum variants, try to use the original field names.

✅ Do this:

```rust,ignore
if let Some(response) = response { ... }
if let Some(response) = event.response { ... }
if let Some(event_response) = event.response { ... }

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

```rust,ignore
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

Single-letter lifetime names are acceptable if a structure is expected to be used very many times (e.g. a script interface may make heavy use of some `Value<'h>` which contains a reference to a heap upon which it is allocated).
NB: the compiler will occasionally recommend the use `'a` as it lacks wider context information.
The name `'a` is nearly always a bad one.

Lifetime parameters should aim to be concise without losing meaning.
Given the difficulty new users face when understanding lifetimes in an interface, try to give them a hand by being explicit.
Single-letter lifetime names should generally be avoided.

✅ Do this:

```rust,ignore
struct ASTQueryMatch<'cursor, 'tree> { .. }

struct Value<'h> { .. }
```

⚠️ Avoid this:

```rust,ignore
struct ASTQueryMatch<'a, 'b> { .. }

struct Value<'value> { .. }
```

## Builder naming

If a builder for a type `MyType` is provided, then it should have an associated function `builder()` which returns a `MyTypeBuilder`.
This `MyTypeBuilder` must also have a fallible `.build()` method, which returns a `MyType`.

Typical usage is hence—

```rust,ignore
let foo = Foo::builder()
    .bar(bar)
    // ...
    .build()?;
```

[dictionary]: https://www.dictionary.com/
[thesaurus]: https://www.thesaurus.com/
