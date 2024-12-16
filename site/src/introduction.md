# Introduction

This book provides a list of best-practices which originate from discussions with both our CTO and the company’s other tech leads.
All points listed here are strongly considered before merging code at Canonical.
Individually, the things written here may seem unimportant or even trivial, but each of these is a crucial building block for writing good, clean code.
You wouldn’t want to hire a builder who has a reputation for installing rotten beams.

This book aims to facilitate peaceful collaboration by expressing an opinion on many common discrepancies, allowing debate to focus on more interesting areas.
It encourages uniformity between projects and teams with the aim of allowing engineers in one part of an organisation to feel reasonably at-home in codebases from other parts.
This goal is notably different from individual works in unique contexts, where the use of highly novel approaches is a healthy form of self-expression.
These guidelines should be considered as a style guide for ‘Canonical Rust,’ and are intended to complement both the idiomatic advice laid out in Rust for Rustaceans and well thought-out, consistent API design.

If you spot any of the problems detailed here in our existing code-bases or patches, don’t be afraid to fix them—a good codebase is inherently more maintainable and will cause fewer headaches and annoyances later.
Remember that all code should aim to be locally-consistent—new code shouldn’t stick out like a sore thumb.

_Disclaimer: this is not a complete list, more items can and likely will be added in future._
_If you find an item which you believe should be in this document, please do open an [RFC][new-rfc]._

[new-rfc]: https://github.com/canonical/rust-best-practices/issues/new/choose
