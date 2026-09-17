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
- **Ch 7 — Managing Growing Projects with Packages, Crates, and Modules**:
  splitting `main.rs` → `main.rs` + `matcher.rs`; module privacy, `mod`/`use`,
  `super` for relative paths into the parent module (used by `mod tests`).
- **Ch 10.2 — "Returning Types that Implement Traits"**: `impl Iterator<Item = ...>`
  in return position; why every branch of the function body must produce the
  *same* concrete type (the `Empty<_>` vs `Filter<...>` mismatch), and `Box<dyn
  Trait>` as the escape hatch when branches genuinely differ.
- **Ch 13.2 — Processing a Series of Items with Iterators**: laziness,
  `.enumerate().filter().map()`, closure parameter destructuring vs. capturing
  from the enclosing scope (`move`), `Iterator::next`/`count`/`size_hint`.
- **Ch 11.1 & 11.3 — Writing Automated Tests / Test Organization**:
  `#[cfg(test)] mod tests`, `#[test]`, `assert_eq!`; unit tests (inline,
  access private items) vs. integration tests (`tests/` dir, public API only).
- **Ch 3.2 — Shadowing**: `Some(p)` binds a *new* name inside a match arm; it
  doesn't refer back to the matched variable (`path`) at all — you could even
  shadow it by naming the binding `path` again, a separate variable of the
  inner type in that arm's scope only.
- **Ch 18.2 — Trait Objects**: `Box<dyn Read>` to unify `File` and `Stdin`
  behind one runtime-chosen type — the case where `Box<dyn Trait>` is the
  actually-correct tool (two genuinely different concrete types), not a
  workaround like the `Empty<_>`/`Filter<...>` case earlier.

## Rust vs C++ build model

- No preprocessor — macros (`macro_rules!`, derive macros like `clap`'s)
  operate on the token stream/AST, hygienically, not textual substitution.
- Compilation unit is the **crate** (whole module tree), not a per-file
  translation unit — why no headers/forward declarations, and why cross-module
  references just work.
- Codegen still goes through LLVM (like clang), and linking is still a real,
  separate step via the system linker — crate-to-crate boundaries are roughly
  where C++'s separately-compiled-library boundary sits.

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
  / [`Iterator::filter`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.filter)
  (predicate takes `&Item`) / [`Iterator::map`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.map)
  (closure takes `Item` by value, returns any `B`)
- [`Iterator::next`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#tymethod.next)
  / [`Iterator::count`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.count)
  / [`Iterator::size_hint`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.size_hint)
  / [`ExactSizeIterator`](https://doc.rust-lang.org/std/iter/trait.ExactSizeIterator.html)
  (why `Filter` doesn't implement it — can't know match count without running
  the predicate)
- [`std::iter::empty`](https://doc.rust-lang.org/std/iter/fn.empty.html)
- [`Box<dyn Trait>`](https://doc.rust-lang.org/book/ch18-02-trait-objects.html) —
  needed when different branches of a function must return genuinely different
  concrete iterator types; `impl Trait` return position can't do this (commits
  to one concrete type for the whole function)
- [Rust Reference — the `cfg` attribute](https://doc.rust-lang.org/reference/conditional-compilation.html#the-cfg-attribute) —
  `#[cfg(test)]`, compile-time conditional inclusion (not a preprocessor pass)
- [`std::sync::OnceLock`](https://doc.rust-lang.org/std/sync/struct.OnceLock.html) —
  lazily-initialized `static`, needed for anything requiring heap allocation
  (e.g. an owned `String`) at "global" scope, since `const`/`static` initializers
  must be compile-time evaluable
- [`std::io::Read`](https://doc.rust-lang.org/std/io/trait.Read.html) —
  the trait unifying `File` and `Stdin`; `read_to_string(&mut self, ...)` takes
  `&mut self`, so the binding holding the reader must be declared `mut`
- [`std::fs::File::open`](https://doc.rust-lang.org/std/fs/struct.File.html#method.open)
  vs [`std::fs::read_to_string`](https://doc.rust-lang.org/std/fs/fn.read_to_string.html) —
  the former only opens (fallible on `NotFound`), the latter opens *and* reads
  in one non-decomposable call, which is why it stopped fitting once stdin
  needed a separate open step
- [`std::io::stdin`](https://doc.rust-lang.org/std/io/fn.stdin.html) /
  [`Stdin`](https://doc.rust-lang.org/std/io/struct.Stdin.html) — implements
  `Read` directly (locks internally per call); `.lock()` for `StdinLock`
  (adds `BufRead`, holds the lock across multiple calls)
- [`Option::as_deref`](https://doc.rust-lang.org/std/option/enum.Option.html#method.as_deref) —
  `Option<String>` → `Option<&str>` by borrowing, not moving; needed to reuse
  a filename after passing it to `File::open` (moving it via `.unwrap()` would
  consume it, an ownership bug from the same family as C++ use-after-move,
  except caught at compile time)
- [`Result::map`](https://doc.rust-lang.org/std/result/enum.Result.html#method.map)
  vs [`Result::map_err`](https://doc.rust-lang.org/std/result/enum.Result.html#method.map_err) —
  mirror images (`Ok` side vs `Err` side); both are **eager** (run the closure
  immediately, unlike `Iterator` adapters), which is why a closure inside them
  can safely borrow a local that an `Iterator::map` closure could not

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
