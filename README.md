# rgrep — Rust grep clone (learning project)

A CLI grep clone built incrementally to develop Rust fluency.

## Milestones

- [ ] **Milestone 1** — Read a file, split into lines, basic substring matching, print matches
- [ ] **Milestone 2** — Add CLI flags: case-insensitive search, line numbers, invert match (use `clap`)
- [ ] **Milestone 3** — Custom error handling: `Result`, `?`, hand-rolled error enum or `thiserror`
- [x] **Milestone 4** — Refactor into modules; rewrite matching logic with iterator chains (`.filter()`, `.map()`, `.enumerate()`) instead of manual loops


# Project 1: CLI Grep Clone — Detailed Implementation Plan

2-week fixed scope. Goal: real fluency with ownership, borrowing, error handling,
and iterators — not just "make it compile."

---

## Milestone 1: Read, Match, Print

**Goal:** simplest possible end-to-end pipeline. No flags, no error handling beyond `.expect()`.

### Steps
1. `cargo init`, confirm `cargo run` works on a hello-world.
2. Hardcode a pattern and filename first — don't touch `std::env::args()` yet. Get
   file-read-and-match working in isolation.
3. Use `std::fs::read_to_string` to read the file into a `String`.
4. Split into lines, iterate, check each line for the pattern with `.contains()`,
   print matches.
5. *Then* wire up `std::env::args()` manually (not `clap` yet) — deliberately
   primitive, so milestone 3's real error handling feels like an upgrade, not a default.

### Usage example
```
$ echo -e "fn main() {\n    println!(\"hello\");\n}" > test.txt
$ cargo run -- println test.txt
    println!("hello");
```

### Concepts forced
- Ownership of `String` from `read_to_string`
- Borrowing `&str` when iterating lines
- Why `.contains()` takes `&str`, not `String`

### Self-check
Explain why `line.contains(pattern)` doesn't take ownership of `line`.

---

## Milestone 2: Real CLI with Flags (`clap`)

**Goal:** replace manual `env::args()` parsing with `clap`, add three flags.

### Steps
1. Add `clap` (derive feature) to `Cargo.toml`.
2. Define `struct Args` with `#[derive(Parser)]` — fields: `pattern: String`,
   `file: String`, `ignore_case: bool` (`-i`), `line_number: bool` (`-n`),
   `invert: bool` (`-v`).
3. Wire `Args::parse()` into `main`.
4. Implement each flag's behavior in the matching loop.

### Usage examples
```
$ cargo run -- -n println test.txt
2: println!("hello");

$ cargo run -- -i PRINTLN test.txt
    println!("hello");

$ cargo run -- -v println test.txt
fn main() {
}

$ cargo run -- -in PRINTLN test.txt
2: println!("hello");

$ cargo run -- --help
Usage: grep-clone [OPTIONS] <PATTERN> <FILE>
Arguments:
  <PATTERN>
  <FILE>
Options:
  -i, --ignore-case
  -n, --line-number
  -v, --invert
```

### Concepts forced
- Struct field ownership
- Clone vs borrow (lowercasing allocates — notice where it happens)
- Light exposure to derive macros

### Self-check
Why lowercase the pattern once outside the loop but the line inside it?

---

## Milestone 3: Real Error Handling

**Goal:** replace every `.expect()`/`panic!` with proper `Result` propagation.

### Steps
1. `main` returns `Result<(), GrepError>`.
2. Hand-roll an error enum first:
   ```rust
   enum GrepError {
       FileNotFound(String),
       IoError(std::io::Error),
   }
   ```
   Implement `Display` manually — feel the tedium before the macro removes it.
3. Convert `read_to_string`'s error into `GrepError` via `.map_err()`, then `?`.
4. Swap to `thiserror`, regenerate the same enum, compare.
5. Branch specifically on `io::ErrorKind::NotFound` for a useful message.

### Usage examples
```
$ cargo run -- println missing.txt
Error: file not found: missing.txt

$ cargo run -- println test.txt
    println!("hello");

$ cargo run -- println /root/somefile.txt
Error: could not read file: Permission denied (os error 13)
```

Process should exit non-zero on error — check with `echo $?` after a failing run,
should print `1`.

### Concepts forced
- `Result` propagation with `?`
- Error enums
- The `From` trait (needed for `?` to auto-convert `io::Error`)

### Self-check
What does `impl From<io::Error> for GrepError` do, and why does `?` need it?

---

## Milestone 4: Modules + Iterator Refactor

**Goal:** split into modules, rewrite the matching loop as an iterator chain.

### Steps
1. Split into `main.rs` (arg parsing, orchestration) and `matcher.rs` (matching
   logic) — forces `pub`/private and `mod`/`use` decisions.
2. Write:
   ```rust
   fn find_matches<'a>(
       lines: impl Iterator<Item = &'a str>,
       pattern: &str,
       ignore_case: bool,
       invert: bool,
   ) -> impl Iterator<Item = (usize, &'a str)>
   ```
   (Simplify the signature if lifetimes fight too hard.)
3. Rewrite the manual loop as `.enumerate().filter(...).map(...)`.
4. Write 3-4 unit tests against `find_matches` directly, no file I/O.

### Usage example
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_simple_match() {
        let lines = vec!["fn main() {", "    println!(\"hello\");", "}"];
        let results: Vec<_> = find_matches(lines.into_iter(), "println", false, false).collect();
        assert_eq!(results, vec![(1, "    println!(\"hello\");")]);
    }

    #[test]
    fn invert_excludes_match() {
        let lines = vec!["fn main() {", "    println!(\"hello\");", "}"];
        let results: Vec<_> = find_matches(lines.into_iter(), "println", false, true).collect();
        assert_eq!(results, vec![(0, "fn main() {"), (2, "}")]);
    }
}
```
Run with `cargo test` — should pass with no `cargo run` / file I/O involved at all.

### Concepts forced
- Iterator adapters and laziness
- Your first real explicit lifetime annotation
- `impl Trait` in signatures
- Basic testing

### Self-check
Why does returning `impl Iterator` instead of `Vec<(usize, &str)>` matter here?

---

## Stretch (only if Milestone 4 finishes early)

Stdin support when no filename is given:
```
$ cat test.txt | cargo run -- println
    println!("hello");
```
Introduces `dyn Read` / trait objects to unify file and stdin sources. Skip if
you're at the 2-week boundary already.

---

## Checkpoint (end of week 2)

1. Did the borrow checker stop feeling adversarial and start feeling like a
   second pair of eyes?
2. Did this cost you sleep, training, or RecoLing time you couldn't spare?
3. Are you excited to keep going, or relieved it's over?

**Kill switch:** if by day 5 of week 1 you're still fighting basic ownership
errors without an "oh, I see why" moment, don't push to Project 2 on schedule —
take stock first.