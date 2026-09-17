# Project 2: Pong, with a swappable renderer — Implementation Plan

A small game "engine" scoped down to exactly what's needed to ship Pong. Goal
is still fluency, not shipping a game — this project specifically targets the
concepts Project 1 (rgrep) didn't reach: entity storage without a GC,
dependency inversion via traits (with a real static-vs-dynamic-dispatch
decision to make), frame-based mutable state, and a first taste of `async`.

**Decisions locked in before starting** (see rationale in chat history /
`NOTES.md` if you want it later):
- Rendering/windowing via [`macroquad`](https://docs.rs/macroquad/latest/macroquad/) —
  not a raw graphics API. It gives you a window and drawing primitives, no
  opinionated game-loop/ECS structure of its own, which leaves the actual
  engine architecture (loop, entity storage, input) as *your* code to design.
- All macroquad calls go through a `Renderer` trait you define — game logic
  never calls `macroquad::*` directly. This is what makes "swap in a custom
  renderer later" realistic instead of aspirational.
- Target game: Pong. Two paddles, one ball, score, win condition. That's the
  spec — same role real `grep`'s behavior played for Project 1.

## Milestones

- [ ] **Milestone 1** — Scaffold the crate, open a window, draw one static rectangle
- [ ] **Milestone 2** — Game loop: input, frame-independent movement, redraw each frame
- [ ] **Milestone 3** — `Renderer` trait boundary; game logic depends on the trait, not macroquad
- [ ] **Milestone 4** — Entity storage without object graphs (ECS-lite: flat vectors + indices)
- [ ] **Milestone 5** — Collision + scoring as pure, unit-tested logic
- [ ] **Milestone 6 (stretch)** — A second, trivial `Renderer` impl, to prove the boundary actually holds

---

## Milestone 1: Window + One Static Rectangle

**Goal:** confirm the toolchain works before any game logic exists.

### Steps
1. `cargo init` inside `pong-engine/`, add `macroquad` to `Cargo.toml`.
2. Get a window open with `#[macroquad::main("Pong")] async fn main() { ... }`
   and a `loop { ... next_frame().await }` that just clears the screen.
3. Draw one fixed rectangle (a paddle, at a hardcoded position) each frame.

### Concepts forced
- `async fn main` with an `#[macroquad::main]` attribute — you haven't used
  `async`/`.await` anywhere yet. Don't go learn async in depth here; just
  notice it's required and ask *why* a rendering loop needs it at all,
  given nothing here is doing actual asynchronous I/O.
- Immediate-mode rendering (redraw everything every frame) vs. retained-mode
  (build a scene graph once, mutate it) — macroquad is immediate-mode; worth
  knowing the term even before it matters.

### Self-check
Why does `next_frame()` need to be awaited? What would happen if you called
it without `.await`, given what you know about eager vs. lazy futures from
the laziness discussion in Project 1?

---

## Milestone 2: Game Loop, Input, Movement

**Goal:** one paddle moves in response to keyboard input, independent of frame rate.

### Steps
1. Track paddle position as mutable state that persists across loop iterations.
2. Read keyboard input each frame (`macroquad::input::is_key_down` or similar).
3. Move the paddle by `velocity * delta_time`, not a fixed pixel amount per
   frame — use macroquad's per-frame delta time function.

### Concepts forced
- Mutable state that survives across loop iterations vs. the one-shot
  function calls you're used to from Project 1 — this loop *is* the program,
  not a `main` that runs once and exits.
- Frame-rate independence: multiplying by delta time so movement speed
  doesn't depend on how fast frames happen to render.

### Self-check
What visibly breaks if you move the paddle by a fixed amount per frame
instead of `speed * delta_time`, on a machine that renders at a very
different frame rate than the one you tested on?

---

## Milestone 3: The `Renderer` Trait Boundary

**Goal:** game logic never mentions `macroquad` by name again.

### Steps
1. Define a trait covering exactly the drawing operations Pong needs —
   nothing speculative, just what milestone 1–2's code actually calls
   (something like: clear the screen, draw a rectangle, draw text for score).
2. Implement that trait for a wrapper struct around macroquad's calls.
3. Rewrite the game loop so it only ever calls methods on *the trait*, never
   `macroquad::*` directly.
4. Decide: does the function/struct holding your game state take
   `impl Renderer` / `<R: Renderer>` (generic, static dispatch), or
   `Box<dyn Renderer>` / `&mut dyn Renderer` (trait object, dynamic
   dispatch)? You've now used `Box<dyn Read>` for a case where the concrete
   type was chosen *at runtime* (file vs. stdin). Here, the backend is
   chosen once, at compile time — argue for yourself whether that changes
   which tool fits.

### Concepts forced
- Dependency inversion via traits: the same mechanism as `Box<dyn Read>`
  unifying `File`/`Stdin`, applied to decouple game logic from a specific
  rendering backend.
- A *second*, contrasting instance of the generics-vs-trait-objects decision
  from Project 1 — this time the answer may reasonably differ, and you
  should be able to say why.

### Self-check
If you chose `Box<dyn Renderer>`, what would change in your reasoning if you
knew, for certain, that this project would only ever have one `Renderer`
implementation compiled into the binary at a time?

---

## Milestone 4: Entity Storage Without a GC

**Goal:** paddles and the ball live in flat storage, not a graph of structs
holding references to each other.

### Steps
1. Represent each game object's data (position, velocity, size) as plain
   fields in a struct, stored in a `Vec` — not as separate named variables
   (`paddle1`, `paddle2`, `ball`) scattered through the game state.
2. When the ball needs to check "did I hit a paddle," look the paddle up
   *by index* from the shared store — don't give the ball struct a direct
   reference/pointer to a paddle struct.
3. Refactor Milestone 2/3's movement and drawing code to iterate the store
   instead of touching named variables directly.

### Concepts forced
- Why a naive object graph (ball holding `&mut Paddle`, paddle holding
  `&mut GameState`, etc.) becomes a fight with the borrow checker almost
  immediately once two things need to mutate each other or a shared parent —
  same category of problem as a C++ intrusive doubly-linked structure with
  raw/shared pointers in all directions, except Rust refuses to compile the
  aliased-mutable-reference version at all rather than letting it become a
  data race or dangling pointer later.
- Indices (`usize`) as a deliberate substitute for references: an index is
  always "valid" to look up (no dangling-pointer risk the way a raw pointer
  has), at the cost of you now being responsible for knowing whether the
  entity at that index still logically exists — a different, self-managed
  kind of bug class, worth noticing as a trade rather than a strict win.

### Self-check
What compiler error do you hit if you try to give the ball's collision-check
function a `&mut Paddle` directly (instead of an index into the store) while
also needing `&mut` access to the ball itself in the same function? Why does
looking both up by index from one shared store sidestep it?

---

## Milestone 5: Collision + Scoring — Pure, Testable Logic

**Goal:** same pattern as Project 1's Milestone 4 — the actual decision logic
has zero dependency on rendering or input, so it's directly unit-testable.

### Steps
1. Write a free function that takes plain position/size data (no `Renderer`,
   no macroquad types anywhere in its signature) and returns whether two
   rectangles overlap (AABB collision).
2. Write a scoring function: given the ball's position and the screen
   bounds, did a point just get scored, and for which side?
3. Wire both into the game loop from Milestone 4.
4. Write unit tests directly against both functions — no window, no frame
   loop involved in the tests at all, same shape as `find_matches`'s tests.

### Concepts forced
- Reinforces the Project 1 lesson: keep decision logic decoupled from I/O
  specifically so it's testable without standing up the whole runtime
  environment (a window, in this case, instead of a file).

### Self-check
None needed up front — once your tests pass, that's the checkpoint. If a
test fails, diagnose it the same way you diagnosed `find_matches` failures:
what does the assertion tell you actually happened vs. what you expected.

---

## Milestone 6 (stretch): Prove the Boundary Holds

**Goal:** without touching any game logic, add a second `Renderer`
implementation — doesn't need to be practical, just needs to compile and
run. A "print an ASCII grid to the terminal every frame" renderer is enough.

If this requires touching anything outside the `Renderer` trait impl and the
one line that constructs it, Milestone 3's boundary wasn't actually clean —
that's useful information, not a failure.

---

## Checkpoint

Same three questions as Project 1, revisit them once Milestone 5 is done:
borrow-checker friction vs. second-pair-of-eyes, time cost vs. benefit,
excited vs. relieved.
