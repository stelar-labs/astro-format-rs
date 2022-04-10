# Astro Format

Astro Format is a library for efficiently transcoding arrays into a single buffer and native rust types into strings.

## Usage

### Cargo.toml

```
[dependencies]
astro-format = "0.5.0"
```

### Module.rs

```
astro_format::{string, arrays};
```

## API

### Array

```
let ars: Vec<Vec<u8>>;

let buf = arrays::encode(&ars);

let ars = arrays::decode(&buf)?;
```

### String

```
let buf: Vec<u8>;

let enc = string::encode::bytes(&buf);

let dec = string::decode::as_bytes(&enc)?;
```

2022-04-10
