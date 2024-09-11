# Rust best practices

This repo provides a list of best-practices which originate from discussions with both our CTO and the company’s other tech leads.
All points listed here are be strongly considered before merging code at Canonical.
Individually, the things written here may seem unimportant or even trivial, but each of these is a crucial building block for writing good, clean code.
You wouldn’t want to hire a builder who has a reputation for installing rotten beams.

Ideally, this repo would form a spec for a new AI linter but at present we must rely on the diligence of humans.
Therefore, this document should be considered as a style guide for ‘Canonical Rust,’ and is intended to complement both the idiomatic advice laid out in Rust for Rustaceans and well thought-out, consistent API design.

If you spot any of the problems detailed here in our existing code-bases or patches, don’t be afraid to fix them—a good codebase is inherently more maintainable and will cause fewer headaches and annoyances later.
Remember that all code should aim to be locally-consistent—new code shouldn’t stick out like a sore thumb.
Note also that the perfect is the enemy of the good—sole focus on fine-tuning code at the cost of all forward progress doesn’t keep the lights on.

_Disclaimer: this is not a complete list, more items can and likely will be added in future._
_If you find an item which you believe should be in this document, please do open an issue._
