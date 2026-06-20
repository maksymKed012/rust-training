# rgrep — Rust grep clone (learning project)

A CLI grep clone built incrementally to develop Rust fluency.

## Milestones

- [ ] **Milestone 1** — Read a file, split into lines, basic substring matching, print matches
- [ ] **Milestone 2** — Add CLI flags: case-insensitive search, line numbers, invert match (use `clap`)
- [ ] **Milestone 3** — Custom error handling: `Result`, `?`, hand-rolled error enum or `thiserror`
- [ ] **Milestone 4** — Refactor into modules; rewrite matching logic with iterator chains (`.filter()`, `.map()`, `.enumerate()`) instead of manual loops
