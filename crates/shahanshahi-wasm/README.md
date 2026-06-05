# shahanshahi-wasm

WebAssembly bindings for the [`shahanshahi`](https://crates.io/crates/shahanshahi) calendar library — deterministic offline conversion between the **Shahanshahi (Imperial Iranian) civil calendar** and the proleptic Gregorian calendar.

Built with [`wasm-bindgen`](https://github.com/rustwasm/wasm-bindgen) and [`wasm-pack`](https://rustwasm.github.io/wasm-pack/).

## Install (npm)

```bash
npm install @melliran/shahanshahi-wasm
```

Or build from source (run from the workspace root):

```bash
wasm-pack build --scope melliran --target web     crates/shahanshahi-wasm  # ESM / browser
wasm-pack build --scope melliran --target bundler crates/shahanshahi-wasm  # webpack / vite
wasm-pack build --scope melliran --target nodejs  crates/shahanshahi-wasm  # Node.js
```

## API

All functions are synchronous. Invalid input throws a JavaScript `Error`.

### `gregorianToShahanshahi(year, month, day)`

```js
import init, { gregorianToShahanshahi } from '@melliran/shahanshahi-wasm';
await init();

gregorianToShahanshahi(1976, 3, 21);
// → { year: 2535, month: 1, day: 1 }
```

### `shahanshahiToGregorian(year, month, day)`

```js
import init, { shahanshahiToGregorian } from '@melliran/shahanshahi-wasm';
await init();

shahanshahiToGregorian(2535, 1, 1);
// → { year: 1976, month: 3, day: 21 }
```

### `isShahanshahiLeapYear(year)`

```js
import init, { isShahanshahiLeapYear } from '@melliran/shahanshahi-wasm';
await init();

isShahanshahiLeapYear(2534); // → true
isShahanshahiLeapYear(2535); // → false
```

### Error handling

```js
import init, { gregorianToShahanshahi } from '@melliran/shahanshahi-wasm';
await init();

try {
  gregorianToShahanshahi(1976, 13, 1); // month 13 is invalid
} catch (e) {
  console.error(e.message);
}
```

## TypeScript

TypeScript definitions are generated automatically by `wasm-pack` and included in the package.

## See also

- [`shahanshahi`](https://crates.io/crates/shahanshahi) — the core Rust library
- [`shahanshahi-cli`](https://crates.io/crates/shahanshahi-cli) — command-line tool
- [SPEC.md](https://github.com/melliran/shahanshahi/blob/main/SPEC.md) — calendar algorithm and era bounds
