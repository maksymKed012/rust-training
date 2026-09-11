# Reading list

Recommendations accumulated during coaching sessions, roughly in the order the
concepts came up. Updated as the project progresses.

## The Rust Book (https://doc.rust-lang.org/book/)

- **Ch 3 — Common Programming Concepts**: variables/mutability, functions as
  expressions, `if`/`else` as an expression (no ternary operator in Rust).
- **Ch 4 — Understanding Ownership** (4.1–4.3): ownership, moves, `&`/`&mut`
  borrowing, `String` vs `&str`. Underlies most of the early ownership bugs
  (`.clone()` on `&str` being shallow, the dangling-reference quiz).
- **Ch 5 — Using Structs**: general shape behind `Args`, though `clap`'s derive
  macro does more than a plain struct.
- **Ch 6 — Enums and Pattern Matching**: `enum`, `match`, `if let` — covers
  `GrepError` and `if let Err(e) = run()`.
- **Ch 8.2 — Storing UTF-8 Encoded Text with Strings**: `String`/`&str` in
  depth; why `.to_lowercase()` allocates (case folding can change byte length)
  — the reasoning behind needing `Cow` at all.
- **Ch 9 — Error Handling**: `panic!` vs `Result`, the `?` operator, designing
  custom error types (§9.2 in particular) — milestone 3's core material.
  - **9.2 — Recoverable Errors with `Result`**, "A Shortcut for Propagating
    Errors: the `?` Operator" section specifically: builds up from a manual
    `match` to `?` step by step.
- **Ch 10.1 & 10.2 — Generics, Traits**: `Cow<'a, str>`'s generic + trait-bound
  shape; `Display`/`Debug` as traits, default/derived implementations.
- **Ch 10.3 — Validating References with Lifetimes**: what `'a` actually is
  (a named, compiler-checked scope — not a type, not a reference), lifetime
  elision rules (why you don't need to write `'a` most of the time).
- **Ch 13.1 — Closures**: what a closure captures from its environment —
  explains why a free `fn` (`to_grep_err`) couldn't see `file_name` but an
  inline closure could.
- **Appendix C — Derivable Traits**: quick reference for what `#[derive(Debug)]`
  and friends actually generate and require.

Not yet needed, coming up with milestone 4:
- **Ch 13.2+ — Iterators**: `find_matches` returning `impl Iterator`, iterator
  adapter chains (`.enumerate().filter().map()`).
- **Ch 7 — Managing Growing Projects with Packages, Crates, and Modules**:
  splitting `main.rs` → `main.rs` + `matcher.rs`.

## std docs

- [`std::fs::read_to_string`](https://doc.rust-lang.org/std/fs/fn.read_to_string.html)
- [`Result::expect`](https://doc.rust-lang.org/std/result/enum.Result.html#method.expect)
- [`str::lines`](https://doc.rust-lang.org/std/primitive.str.html#method.lines)
- [`Vec`](https://doc.rust-lang.org/std/vec/struct.Vec.html)
- [`std::fmt::Debug`](https://doc.rust-lang.org/std/fmt/trait.Debug.html) /
  [`std::fmt::Display`](https://doc.rust-lang.org/std/fmt/trait.Display.html)
- [`str::contains`](https://doc.rust-lang.org/std/primitive.str.html#method.contains)
  and the [`Pattern`](https://doc.rust-lang.org/std/str/pattern/trait.Pattern.html)
  trait it requires (only `&str`, `char`, etc. — not owned `String`)
- [`str::to_lowercase`](https://doc.rust-lang.org/std/primitive.str.html#method.to_lowercase)
  vs [`str::make_ascii_lowercase`](https://doc.rust-lang.org/std/primitive.str.html#method.make_ascii_lowercase)
  (allocating vs in-place, and why general Unicode case folding can't be
  in-place but ASCII can)
- [`std::borrow::Cow`](https://doc.rust-lang.org/std/borrow/enum.Cow.html) —
  borrowed-or-owned in one type via `Deref`; note `Deref` rescues you in
  method-call position but *not* in generic trait-bound argument position
  (`&Cow<str>` passed to something requiring `Pattern` needs an explicit
  `.as_ref()`/`&*` deref first)
- [`std::env::args`](https://doc.rust-lang.org/std/env/fn.args.html)
- [`TryInto`](https://doc.rust-lang.org/std/convert/trait.TryInto.html) vs `as`
  casts (fallible/checked vs silent truncating/wrapping)
- [`std::process::exit`](https://doc.rust-lang.org/std/process/fn.exit.html)
- [`Iterator::enumerate`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.enumerate)

## Crates

- [`clap` derive tutorial](https://docs.rs/clap/latest/clap/_derive/index.html) —
  positional args vs `#[arg(short, long)]` options; `bool` fields default to
  `false`/`SetTrue` semantics automatically.
- [`thiserror`](https://docs.rs/thiserror/latest/thiserror/) — `#[derive(Error)]`
  + `#[error("...")]` per variant generates `Display`; `#[from]` generates the
  `From` impl `?` needs to auto-convert error types across a function boundary.

## Tools

- `cargo clippy` — run this yourself before asking for a style/idiom review;
  it catches `needless_return`, redundant borrows, `to_string`-in-`format!`,
  etc. automatically.
- `rust-analyzer` (VS Code extension, `rust-lang.rust-analyzer`) — go-to-
  definition into std/crate source, inline diagnostics, required for the
  debug codelens and the `rustfmt`-on-save setup in `.vscode/settings.json`.
