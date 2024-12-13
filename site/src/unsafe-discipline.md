# Unsafe discipline

## Minimise unsafe

Rust’s `unsafe` keyword enables a small but important number of additional powers which the compiler is unable to check.
In effect, it is a ‘hold my beer’ marker—you tell the compiler to trust you and just watch whilst you do something either incredibly impressive or incredibly harmful.
But the compiler is not the only one whose trust we require—when we use `unsafe`, we also ask our _users_ to trust that we know exactly what we are doing.
Under no circumstance do we want to break that trust.

Therefore, we must endeavour to minimise the use of unsafe constructs in our code.
If there is a place where `unsafe` is used but not required for a strict functional requirement, drop it and replace it with a safe equivalent.
If left, such a section is inherently much harder to maintain as it require not only excellent documentation but also dilligence, careful consideration of preconditions and actions, and above all time.
The more unsafe code a project contains, the slower it will be able to move forward in the long-run.

If `unsafe` is mandatory, minimise the scope of the `unsafe` blocks and functions in use.
Even if this means adding an extra line or two, maximise the amount of help the compiler can give us whilst minimising the number of unsafe interactions which will require audit.
A function of 50 lines wrapped in a single large `unsafe` block is far harder to maintain than a 56 line function containing three one-line `unsafe` blocks.

Note that ‘because it’s faster’ is not a good reason alone to use `unsafe` constructs.
Even if true, unless profiling can categorically show that safety checks add a _globally-significant_ cost safety should always take a higher priority.
Rust is an extremely fast language anyway.

## Document preconditions

Both `unsafe` functions and `unsafe` blocks must document their preconditions in a comment with the prefix `// SAFETY: `.
If something goes wrong in future, this will help the future maintainer understand which conditions have been violated.
These comments must be carefully maintained and updated as changes are made.
