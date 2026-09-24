# rust-gol

A terminal implementation of **Conway's Game of Life** written in Rust.

The board is stored as a *sparse hash map* of live cells, so the universe is
effectively unbounded — there is no fixed grid and no wrapping. The simulation
runs on its own thread while a second thread renders the visible window to the
terminal with ANSI colors and Unicode borders. A third thread exists as a
placeholder for keyboard input, which is not wired up yet.

```
╔════════════════════════════════════════════════════════════╗
║  ∙ ∙ ∙ ∙ ∙ ∙ ∙ ∙ ∙ ∙ ∙ ∙ ∙ ∙ ∙ ∙ ∙ ∙ ∙ ∙ ∙ ∙ ∙ ∙ ∙ ∙ ∙ ∙ ∙  ║
║  ∙ ∙ ∙ ∙ ⏺ ⏺ ∙ ∙ ∙ ∙ ∙ ∙ ∙ ⏺ ∙ ∙ ∙ ∙ ∙ ∙ ∙ ∙ ∙ ∙ ∙ ∙ ∙ ∙ ∙  ║
║  ∙ ∙ ∙ ∙ ∙ ⏺ ∙ ∙ ∙ ∙ ∙ ∙ ⏺ ⏺ ∙ ∙ ∙ ∙ ∙ ∙ ∙ ∙ ∙ ∙ ∙ ∙ ∙ ∙ ∙  ║
║  ∙ ∙ ∙ ∙ ∙ ∙ ∙ ∙ ∙ ∙ ∙ ∙ ∙ ∙ ∙ ∙ ∙ ∙ ∙ ∙ ∙ ∙ ∙ ∙ ∙ ∙ ∙ ∙ ∙  ║
╚════════════════════════════════════════════════════════════╝
Live cells  (view): 6
Live cells (total): 15
Cycle count       : 42
```

## Features

- **Sparse, unbounded board** — cell coordinates are `i128`, so patterns can
  grow far outside the visible window without allocating a grid.
- **Classic B3/S23 rules** — a dead cell is born with exactly 3 live neighbours;
  a live cell survives with 2 or 3.
- **Terminal renderer** — colored live/dead cells, box-drawing borders and a
  live footer (cells in view, total live cells, cycle count).
- **Map files** — load an initial pattern from a `.gol` file.
- **Warm-up cycles** — optionally pre-simulate `N` generations behind an
  `indicatif` progress bar before the animation starts.
- **Adjustable speed** — frame interval in milliseconds.
- **Quit with `Ctrl+C`** — there is no in-app key handling yet, so a signal is
  how you stop the program.
- **Resize aware** — the view width follows the terminal width.

## Requirements

- A Rust toolchain with **Cargo** (edition 2021).
- A terminal that supports ANSI escape sequences and UTF-8.

## Build

```sh
cargo build --release
```

The `dev` profile is tuned for the simulation (`opt-level = 3`,
`overflow-checks = false`), so `cargo run` is already fast — but see the
[large map](#included-maps) note below.

## Usage

```sh
cargo run -- [OPTIONS]
```

| Option            | Short | Default    | Description                                  |
| ----------------- | ----- | ---------- | -------------------------------------------- |
| `--map <FILE>`    | `-m`  | `map.gol`  | Map file to load.                            |
| `--speed <MS>`    | `-s`  | `16`       | Simulation/render interval in milliseconds.  |
| `--cycle <N>`     | `-c`  | `0`        | Cycles to simulate before the animation.      |
| `--help`          | `-h`  |            | Print help.                                   |
| `--version`       | `-V`  |            | Print version.                                |

Examples:

```sh
# Run the default map at ~60 FPS
cargo run

# Load a specific pattern, slower
cargo run -- -m map2.gol -s 50

# Pre-simulate 500 generations (with a progress bar), then animate
cargo run -- -m map.gol -c 500 -s 32
```

### Controls

There is no in-app key handling yet — the event thread is an empty loop, so
`q` (and `Q`) do nothing.

| Key      | Action                                  |
| -------- | --------------------------------------- |
| `Ctrl+C` | Quit (delivers `SIGINT` to the process) |

In-app keyboard controls (`q` to quit, pause/resume, speed, panning, editing
cells) are planned — see the [roadmap](#roadmap--known-limitations).

## Map file format

A map is a flat list of live-cell coordinates separated by `|`, where each
entry is `x,y`:

```
-7,0|-6,-1|-6,1|-5,-2|-5,2|-4,-1|-4,1|-3,0|
```

Coordinates are signed integers (`i128`), so negative positions are valid and
the origin `(0,0)` is simply wherever the view starts. Parsing is lenient:
empty entries are skipped. `Board::cycle_n` counts the loaded cells as the
starting generation.

### Included maps

| File          | Cells    | Notes                                                        |
| ------------- | -------- | ------------------------------------------------------------ |
| `map.gol`     | 108      | Small scattered soup; the default map.                        |
| `map2.gol`    | 15       | Two tiny patterns.                                            |
| `map_tmp.gol` | ~1.77 M  | Very large generated soup (~15 MB). Heavy to parse and step. |

## Project layout

| File            | Lines | Responsibility                                                        |
| --------------- | ----- | --------------------------------------------------------------------- |
| `src/main.rs`   | 159   | CLI parsing, terminal setup, thread orchestration.                    |
| `src/board.rs`  | 123   | Sparse board state, Conway rules, cycle stepping, progress bar.       |
| `src/display.rs`| 161   | Terminal rendering: borders, cells, colors, footer, view geometry.    |
| `src/parser.rs` | 34    | Load/save `.gol` map files.                                           |
| `Cargo.toml`    | —     | Package manifest and dependencies.                                    |
| `rustfmt.toml`  | —     | Formatting rules (hard tabs, 80 columns, leading match pipes).        |

## How it works

### Board (`src/board.rs`)

- The board is a `HashMap<(i128, i128), CellState>` — only live cells are
  stored, so iteration cost scales with the population, not the area.
- `cycle_once()` walks every live cell, counts its live neighbours and records
  each *dead* neighbour's visit count in a scratch map. Live cells with 2–3
  neighbours survive; dead cells visited by exactly 3 live neighbours are born.
- `print_board()` renders the window starting at `(view_x, view_y)` and returns
  the number of live cells currently visible.
- `cycle_n(n, display)` steps `n` generations and draws an `indicatif` progress
  bar (useful for pre-rolling a pattern before display begins).

### Display (`src/display.rs`)

- Holds a `BufWriter<Stdout>` and writes raw ANSI escape codes for cursor
  positioning, clearing and hiding the cursor.
- Colors are 24-bit true-color values defined in `Display::new`, with glyphs
  (`⏺` live, `∙` dead) and double-line box characters as associated constants.
- `footer()` prints the three live statistics; `get_view_range()` provides the
  coordinate ranges the board iterates over.

### Runtime (`src/main.rs`)

- `clap` parses the CLI, then `termsize` supplies the initial window size.
- Raw mode is not enabled: the terminal stays in cooked mode, so `Ctrl+C`
  keeps working.
- The board and display are wrapped in `Arc<Mutex<…>>` and shared by three
  threads:
  1. **simulation** — runs the warm-up cycles, then steps the board forever;
  2. **display** — paces redraws, re-reads the terminal size every 60 ms, and
     re-renders the board;
  3. **events** — an empty placeholder loop; it does not read input yet.
- `to_sleep()` / `repeat_fn_ms()` implement frame pacing so rendering and
  simulation are roughly limited to the requested `--speed`.

## Roadmap / known limitations

The TODO block at the end of `src/main.rs` lists the intended next steps:

- [ ] Move/pan the view over the unbounded board.
- [ ] Add cells with the mouse/keyboard (only while paused).
- [ ] Pause and resume the simulation.
- [ ] Change the simulation speed at runtime.
- [ ] Introduce a message channel between threads and wrap each thread in a
      constructor / `run` / `send` API (messages: `pause`, `quit`).

Because the simulation and display loops run forever and no input is read, the
only way to stop the program is `Ctrl+C`. A clean in-app shutdown depends on the
messaging layer above.

## Author

**Yezarath** — <dao-saint@proton.me>
