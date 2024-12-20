# Structural discipline

## How to structure `mod.rs`

Files named `mod.rs` must only be used to specify the structure of a project, if definitions are added, they quickly become messy, ultimately detracting from their core purpose of declaring sub-modules and the current module’s interface.

The `mod.rs` file must be separated into distinct blocks in the following order, keeping the most public items first:

1. `pub mod _` declarations
2. `pub(crate) mod _` declarations
3. `mod _` declarations
4. `pub use _` declarations
5. `pub(crate) use _` declarations
6. `pub use _` declarations

Any items gated behind a `#[cfg(...)]` must be placed at the end of the file, in the same order as the above.
Like-gated items should be wrapped in a block, i.e. `#[cfg(...)] { /* items here */ }`.

No other items should be present.

Note that these guidelines also hold for `lib.rs`, with the one exception that a crate’s `Error` and `Result` types are permitted in `lib.rs`, given their central importance.

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

Always prefer the first way as `mod.rs` files are expected to have a specific structure, hence the reader knows what they’ll see when opening one of these, unlike some arbitrary `foo.rs`.
Some editors group files and folders separately, so in the above example, the fact that `foo.rs` is related to the `foo/` directory will not be obvious when there are many other entries in between.

## Define `Error` and `Result` in a standard location

A crate’s `Error` and `Result` types are likely the most important types it contains and should therefore be declared in a standard, outward-facing place.

In library crates, the `Error` type should be defined in the crate-root, `lib.rs` immediately below the `mod` and `use` declarations.
The custom `Result` type alias, `type Result<T> = std::result::Result<T, Error>` must be immediately below `Error`.

In binary crates, the `Error` type should be defined `error.rs` and the `Result` alias in `result.rs`, both in the crate’s root directory.
