# wareki-rs

A fast, safe, and precise library for converting between the Gregorian Calendar and the Japanese Era (Wareki) calendar.
Written in Rust, with native bindings for Python, Node.js, and Go.

It strictly handles leap years and era boundaries (e.g., the transition from Heisei to Reiwa on May 1st, 2019).

## Features

- **High Performance & Safe**: Core logic is implemented in Rust (`cargo`).
- **Precision**: Relies on `chrono` for precise date handling, including leap years (e.g., Reiwa 6 / 2024 is a leap year).
- **Multiple Input Formats**: The `from_wareki` function accepts full era names (e.g. `令和`), single Kanji (e.g. `令`), and Romaji abbreviations (e.g. `R` or `r`).
- **Native Bindings**:
  - Python (via `PyO3` / `maturin`)
  - Node.js (via `napi-rs`)
  - Go (via `CGO` & Rust C-FFI)
- **Unified Dev Environment**: Pre-configured DevContainer that provides the toolchains for all 4 languages.

## Supported Eras

- `令和` (Reiwa)
- `平成` (Heisei)
- `昭和` (Showa)
- `大正` (Taisho)
- `明治` (Meiji)

---

## 1. Rust (Core)

Add `wareki-core` to your `Cargo.toml`.

```rust
use wareki_core::{to_wareki, from_wareki, Era};
use chrono::NaiveDate;

fn main() {
    // Gregorian to Wareki
    let w = to_wareki(2026, 2, 23).unwrap();
    println!("{}{}年", w.era_name(), w.year); // "令和8年"

    // Wareki to Gregorian (chrono::NaiveDate)
    let date = from_wareki("令和", 8, 2, 23).unwrap();
    println!("{}", date); // "2026-02-23"

    // Short names and Romaji are also supported
    assert_eq!(from_wareki("R", 8, 2, 23).unwrap(), date);
}
```

---

## 2. Python (`wareki-python`)

Built with PyO3. Returns native `datetime.date` objects.

### Installation (Development)
```bash
cd bindings/python
python -m venv .venv
source .venv/bin/activate
pip install maturin
maturin develop
```

### Usage
```python
import wareki
import datetime

# Gregorian to Wareki
w = wareki.to_wareki(2026, 2, 23)
print(f"{w.era_name}{w.year}年") # "令和8年"

# Wareki to Gregorian
dt = wareki.from_wareki("令和", 8, 2, 23)
print(dt.isoformat()) # "2026-02-23"

# Short names
dt2 = wareki.from_wareki("R", 8, 2, 23)
```

---

## 3. Node.js (`wareki-node`)

Built with `napi-rs` (`napi4`).

### Installation
```bash
cd bindings/node
npm install
npm run build
```

### Usage
```javascript
const wareki = require('./bindings/node/index.js');

// Gregorian to Wareki
const w = wareki.toWareki(2026, 2, 23);
console.log(`${w.eraName}${w.year}年`); // "令和8年"

// Wareki to Gregorian (Returns ISO 8601 string)
const ds = wareki.fromWareki("令和", 8, 2, 23);
console.log(ds); // "2026-02-23"
```

---

## 4. Go (`wareki-go`)

Relies on CGO to link the statically compiled Rust binary.

### Installation
First, compile the C library via Rust:
```bash
cargo build --release -p wareki-c
```

### Usage
```go
package main

import (
	"fmt"
	"github.com/user/wareki-go"
)

func main() {
	// Gregorian to Wareki
	w, err := wareki.ToWareki(2026, 2, 23)
	if err == nil {
		fmt.Printf("%s%d年\n", w.EraName, w.Year) // "令和8年"
	}

	// Wareki to Gregorian (Returns time.Time)
	dt, err := wareki.FromWareki("令和", 8, 2, 23)
	if err == nil {
		fmt.Println(dt.Format("2006-01-02")) // "2026-02-23"
	}
}
```
*Note: To run the Go program, you might need to supply `CGO_LDFLAGS` pointing to the built `libwareki_c.a` within the `target/release` directory.*

---

## Development & Testing

This project uses a DevContainer. Simply open this repository in VS Code (with the Dev Containers extension) or GitHub Codespaces. It comes fully equipped with `rustup`, `python 3.12`, `node 20`, and `go 1.22`.

### Run all tests in parallel via GitHub Actions CI
The project is configured to run tests automatically on Push and Pull Requests. See `.github/workflows/ci.yml`.

### Run tests manually inside DevContainer
```bash
# Rust tests
cargo test -p wareki-core

# Python tests
(cd bindings/python && maturin develop && pytest test_wareki.py)

# Node.js tests
(cd bindings/node && npm run build && npm test)

# Go tests
cargo build --release -p wareki-c
export CGO_LDFLAGS="-L$(pwd)/target/release -lwareki_c"
export LD_LIBRARY_PATH="$(pwd)/target/release:$LD_LIBRARY_PATH"
(cd bindings/go && go test -v)
```
