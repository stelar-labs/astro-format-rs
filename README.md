# Rust Astro Format

Astro Format is a library for efficiently encoding and decoding a set of bytes into a single buffer format.

## API

### Cargo.toml

```
[dependencies]
astro-format = "0.2.0"
```

### Import

```
astro_format::{encode, decode};
```

### Encode

```
let set: Vec<Vec<u8>>;

let astro = encode(&set);
```

### Decode

```
let astro: Vec<u8>;

let set = decode(&astro);
```

2022-04-08
