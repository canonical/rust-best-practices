# Preconditions

All new code should abide by `cargo fmt`, `cargo clippy`, and `cargo clippy --tests`.
If your crate uses features, be careful to ensure that `clippy` is definitely being run on all of your code, for example by using `--all-features`.
