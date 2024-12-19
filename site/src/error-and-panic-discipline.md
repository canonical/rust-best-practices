# Error and panic discipline

## Error messages

In Rust, it is a good idea to start writing a project by first making its `Error` type, this no only encourages message consistency, but also highlights the necessary information for these good error messages.
This in turn, makes it clearer which information should be plumbed where, avoiding the awkward and altogether too-common situation where an error condition is identified, but much work must be done to get the right information there.

Error messages should be concise.
Every second longer a user spends reading an error message and not understanding what went wrong is a second longer of their frustration.
The form of the phrases used in error messages should be consistent—if, likely from previous messages, the user is expecting—

```ignore
cannot foo the bar
```

but instead reads—

```ignore
bar is not fooable
```

they will experience unnecessary discomfort.
Keep it nice and clean.

A majority of the time, error messages should start with a verb and in most cases that verb should be prefixed with ‘cannot.’

Capitalisation of error messages should be consistent.
If it is possible that an error will not be a top-level error (e.g. it may be wrapped), then the first letter should be lowercase.
Unexpected capital letters in the middle of a line of logging look dishevelled and imply that little thought has gone into the overall design of the implementation.
This couldn’t be further from the truth so let’s make it appear as such.

Remember: the latin alphabet is optimised for many contiguous lowercase letters, hence lowercase is a good default to maintain.
However, uppercase should be used for acronyms and standard names such as: TCP and NixOS.

When writing error messages, think about the background of the expected user/consumer of the project.
Consider whether the person reading it will have good technical knowledge and tailor your approach as such.

If the reader of these error messages is expected to be a (Rust) developer (e.g. code in question is in a library or building block for something else), they will be familiar with how to fix simple problems and as such may not benefit from unnecessary advice.
For these readers, a good concise error message will contain all the information needed to point them towards how to fix it.
In this case, suggestions are at best superfluous and at worst unhelpful.

If the reader of these error messages is not expected to be a developer (e.g. these messages will appear in some GUI), think carefully about where you want to send that reader next.
Whereas a developer might be okay to submit an issue, a non-developer will appreciate being explicitly pointed in the right direction.

Going up the stack, the reader’s knowledge of low-level details becomes less reliable hence they must lean more heavily on the help we give them.

As an error passes further and further up the stack, more context messages may be added to it.
Be aware of how errors will bubble up to avoid repeating the same information multiple times.

Note that all this advice applies to both error messages associated with error types and panic messages.

## Error types

All reasonable types which implement `Error` fall into one of three categories:

- Those which erase the underlying types
- Those which preserve them, for example by enumeration
- Those which preserve them opaquely

Errors which use type-erasure (e.g. `Box<dyn Error>` and `anyhow::Error`) are often easier to use when writing code, however things become very problematic later on when attempting to inspect errors—with less help from the compiler comes far more places for subtle breakages to occur, both now and in future.
Type-erased errors should only be used in prototypes where maintenance will never be a concern, otherwise, use concrete types.
As a general rule, type erased errors _must not be used in library crates._
Type erasure is a very strong opinion and one which may not be shared by a crate’s dependants and the process of converting from erased errors back to a concrete one is unreliable and unpleasant, and hence will irritate consumers.

Errors which preserve types (e.g those annotated with `#[derive(thiserror::Error)]`) give Rust a unique advantage—not only can the golden path receive first-class support, but so too can the error path, thus allowing an even higher level of quality to be attained.
In particular, the process of responding to particular errors is far more robust with enumerated errors.

Errors which preserve types but which represent unrecoverable errors should represent their error condition as a contained `String` which is assigned where the error is constructed.
(Note that although `&‘static str` may be applicable, `String` offers more flexibility and can be expected to have a negligible performance impact.)
When constructing these errors, special care must be taken to ensure that the message is consistent with other errors in the codebase.
The field used to hold the reason for the error in these cases should be named `reason`.

```rust,ignore
#[derive(Debug, thiserror::Error)]
pub enum Error {
    // ...

    #[error("invalid {credential}: {reason}")]
    InvalidCredentials{
        credential: String,
        reason: String,
    },
}
```

If one error wraps another, the inner error should be held in a field named `source`.

```rust,ignore
#[derive(Debug, thiserror::Error)]
pub enum Error {
    // ...

    #[error("cannot access {path}: {source}")]
    IO {
        path: PathBuf,
        source: io::Error,
    },
}
```

Note that exposing the error type originating from dependencies as these may accidentally expose internal details in a public API.
In these cases, if using enumerated errors, consider adding an `Internal` variant which holds a type which hides the internal details as follows—

```rust,ignore
#[derive(Debug, thiserror::Error)]
pub enum Error {
    // ...

    #[error(transparent)]
    Internal(#[from] InternalError),
}

#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub struct InternalError(#[from] InternalErrorImpl);

#[derive(Debug, thiserror::Error)]
enum InternalErrorImpl {
    // ...
}
```

## Error conversion

Errors returned from other crates should be converted to current the crate’s `Error` type at the earliest reasonable opportunity.
The intuition here is that within our crates, we should be talking our own error language and that calling functions and methods in other crates crosses an interface boundary, so to propagate their errors for too long creates a slow transition rather than a clean, abrupt change.
Long call-chains often handle many different error types and converting early into the common crate-local error type will allow natural error propagation.
By maintaining the same convention with short chains, our code becomes more predictable and hence easier to read.

✅ Do this:

```rust
{{#include snippet_helpers/error_and_panic_discipline.rs}}
# fn snippet() -> Result<()> {
# use std::env;
# use url::Url;
let override_url = env::var("URL")
    .ok()
    .map(|override_url| {
        Url::parse(&override_url).map_err(|source| Error::MalformedEnvUrl {
            env_var: "URL",
            source,
        })
    })
    .transpose()?;
# Ok(())
# }
```

⚠️ Avoid this:

```rust
{{#include snippet_helpers/error_and_panic_discipline.rs}}
# use std::env;
# fn snippet() -> Result<()> {
let override_url = env::var("URL")
    .ok()
    .map(|override_url| url::Url::parse(&override_url))
    .transpose()
    .map_err(|source| Error::MalformedEnvUrl { // The error to be mapped comes from somewhere in the chain above!
        env_var: "URL",
        source,
    })?;
# Ok(())
# }
```

## Panic calmly

Panics must only be used if a program enters an unrecoverable state.
Further, that unrecoverable state must not be as a result of user input—a program which a user can easily crash is not a good program.

In Rust, panics are very aggressive, especially as if the default panic strategy is overridden (e.g. by a user who wants a smaller binary and hence sets `profile.release.panic = "abort"` in their `Cargo.toml`), we have no guarantee that the usual cleanup is performed.
In such a situation, we must rely on the OS to do its best with the resources it understands but we have no guarantee that this will be sufficient.
By default therefore, don’t panic.

If, however, a panic is inevitable, be sure that the message signals who is at fault—if it’s an internal error, start the message with `internal error:`.

The over-use of `.unwrap()` is a major red flag, as the resulting code is likely very fragile, relying on possibly-unclear preconditions in the code above it.
If that code changes—as is very likely in code still under active development—production code may panic.
As a rule of thumb, `.unwrap()` should only be used in code in tiny scopes, where errors can only possibly originate from the programmer—e.g. in `Regex::new("...").unwrap()`, where a panic can only occur if the raw regex constant is invalid.
In general though, unwrapping should be replaced with either:

- Good use of the type system to allow the compiler to enforce preconditions
- Calls to `.expect` which document the required preconditions (these must follow the same convention as panic messages)

The first option is very much preferable.
What follows is two common situations and solutions.

We can remove panics originating from `.unwrap()` calls by using the `?` operator to pass errors up the call-stack.
In the case of unwrapping options in a function which returns a result, calling the `.ok_or`/`.ok_or_else` methods may be required.

We can also remove panics originating from `.unwrap()` calls by using pattern-matching.
If a call to `x.unwrap()` is guarded by `if x.is_some()`/`if x.is_ok()`, instead make use of pattern matching: `if let Some(x) = x` or `if let Ok(x) = x`.

When calls to `.unwrap()` are removed, the surrounding code is not only more robust, but you may notice that it is often visually cleaner.
This is not a coincidence—it is a nudge from the Rust developers to encourage fault-tolorant code and practices.

There are two exceptions to this avoidance of panicking: tests and panic-propagation.

If the `?` operator is used in tests, the origin of the error is lost, making the test harder to debug.
A failed `.unwrap()` will result in a trace pointing to where the panic occurred.

If a thread panics whilst it has acquired a `Mutex` lock, we have no guarantee that the contents of the mutex represents a valid state and hence the lock gets _poisoned._
This means that any other thread which attempts to lock the mutex will get an error.
In this case, panicking is acceptable it effectively propagates an existing panic from another thread.
