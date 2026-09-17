# Notes — Milestone 4 (modules, iterators, tests, stdin stretch goal)

Short notes on concepts covered and the specific questions that came up,
roughly in order. Not a tutorial — a memory aid for things that weren't
obvious the first time.

## `impl Trait` in return position

- `Iterator<Item = X>` isn't a positional generic like `vector<T>` — `Item` is
  an *associated type*, set by name (`Item = X`), not a template slot.
- `impl Iterator<Item = X>` return type commits the function to **one**
  concrete type for its whole body. Every branch must produce the *same*
  underlying adapter-chain type — you can't have one branch return
  `std::iter::empty()` and another return a `Filter<...>` chain.
- Escape hatch when branches genuinely differ: `Box<dyn Iterator<Item = X>>`
  (or `Box<dyn Read>`, same idea) — heap allocation + vtable dispatch, in
  exchange for allowing different concrete types at runtime.
- Question: *"is `Box<dyn Read>` here the same kind of thing as the earlier
  `Empty`/`Filter` mismatch?"* — no, it's the case where a trait object is
  actually necessary (`File` and `Stdin` are structurally different types,
  no restructuring unifies them), vs. the `Empty`/`Filter` case where
  redesigning the input could have avoided needing `Box` at all.

## Lifetimes

- `find_matches<'a>(pattern: &'a str, file_content: &'a str) -> impl
  Iterator<Item = (usize, &'a str)>` — unifying both inputs under one `'a` is
  a *simplification*, not a hard requirement. Verified two independent
  lifetimes (`'a` for `file_content`, `'b` for `pattern`, with `where 'a:
  'b` and `+ 'b` on the return) compiles fine too. One name is fine here
  because both borrows come from the same call site and live equally long.
- The real, minimal requirement when a closure is `move`d into a returned
  iterator: whatever it captured must outlive the iterator, not necessarily
  share an identical lifetime with anything else in the signature.

## Closures: capture vs. destructuring (easy to conflate)

- **Capture** = closure reaching into its *enclosing scope* for a variable
  (`pattern`, `ignore_case` in `find_matches`).
- **Destructuring** = naming pieces of the closure's own *parameter*, which
  arrived from whatever iterator adapter sits upstream (`.enumerate()`
  produces `(usize, &str)` tuples — `filter`'s `|(i, line)| ...` destructures
  that incoming parameter; it hasn't captured anything).
- `filter`'s predicate takes `&Self::Item` (a reference) — match ergonomics
  usually let you write `|(i, line)|` directly against a `&(usize, &str)`
  without an explicit `&` in the pattern.
- `map`'s closure takes `Self::Item` **by value** and returns any `B` —
  different from `filter`'s by-reference, bool-returning predicate.

## Laziness — the recurring theme

- `Iterator` adapters (`.filter()`, `.map()`, `.enumerate()`) are **lazy**:
  building the chain does nothing; the closure only runs once something
  drives the iterator (`for`, `.collect()`, `.next()`) — possibly long after
  the function that built the chain has returned.
- `Result`/`Option` combinators (`.map()`, `.map_err()`, `.and_then()`) are
  **eager**: the closure runs immediately, synchronously, as part of
  producing the returned value. No deferred storage of the closure at all.
- This is *the* reason a closure capturing a local by reference can be fine
  in one context and a compile error in another — it's not about which
  method name, it's whether the closure is invoked now or stashed for later.
  Gut check: "if I call this and do nothing else with the result, did real
  work already happen?" Yes → eager. No → lazy.
- Quiz snippet (`shout_lines`) that failed to compile: closure captured a
  local `String` by reference, then got embedded in a `Map` iterator
  *returned* from the function — needed `move` to own its capture instead.
  C++ comparison: `std::ranges`/views are lazy the same way, but only
  guard against dangling *temporaries* (`borrowed_range`/`dangling`) — a
  named local captured by reference in a returned view/lambda still compiles
  and dangles at runtime in C++; Rust's borrow checker catches the general
  case, not just the temporary-specific one.

## `mut` on parameters vs. `&mut` references

- `fn f(mut x: T)` — `mut` describes the callee's *own local binding*,
  unrelated to whatever the caller passed in. By-value parameters always
  get an independent local; the caller's variable never needs `mut` for this.
- Only `&mut` parameters (borrowing the *caller's* storage) require the
  caller's own variable to be declared `mut`.
- Question that came up: *"why doesn't `input_source` need to be `mut` at
  the call site in `run()` if `open_input_source` takes `mut
  input_source: Box<dyn Read>`?"* — because it's passed by value (moved),
  not by reference; the `mut` only governs the function's own copy.
- Related bug caught in the stretch-goal quiz: `let source: Box<dyn Read> =
  ...` (no `mut`) followed by `source.read_to_string(&mut buffer)` — fails,
  because `Read::read_to_string` takes `&mut self`, so the binding holding
  the reader must itself be `mut`.

## Pattern matching binds new names, doesn't reuse the matched name

- `match path { Some(p) => ... }` — `p` is a fresh binding chosen by the
  pattern, not a reference to `path` by another name. Could legally write
  `Some(path)` instead, which would *shadow* the outer `path` inside that
  arm with a new, differently-typed variable of the same name — legal, but
  easy to misread, hence picking a distinct short name (`p`).

## `Box<dyn Read>` / stdin stretch goal

- `open_file`'s old shape (`std::fs::read_to_string(path)`) fused "open" and
  "read" into one non-decomposable call — broke down once a second source
  (stdin) needed to exist, since there's no equivalent all-in-one function
  for it.
- Split into: `get_input_source(Option<&str>) -> Result<Box<dyn Read>,
  GrepError>` (decide + open source, box it) and a read step that calls
  `.read_to_string()` on the already-open `Box<dyn Read>` to get a `String`.
- `Box::new(file)` is `Box<File>`; coercing to `Box<dyn Read>` happens
  automatically when the surrounding context provides an expected type
  (return position, a `let` type annotation) but needs an explicit
  `as Box<dyn Read>` inside something like a `.map()` closure, where there's
  no such context.
- `.map_err()` only transforms the `Err` side of a `Result`; the `Ok` side
  passes through untouched — need `.map()` (mirror image) to convert
  `Ok(File)` into `Ok(Box::new(file) as Box<dyn Read>)`.
- `Option<String>::as_deref()` → `Option<&str>`, borrowing instead of moving
  — needed to use a filename both for `File::open` and later in an error
  message; `.unwrap()` on `Option<String>` directly would move the `String`
  out, making it unavailable afterward (compile-time-caught use-after-move).
