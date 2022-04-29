# Astro Format

Astro Format is a library for efficiently transcoding arrays into a single buffer and native rust types into hexadecimal strings.

## Usage

### Cargo.toml

```
[dependencies]
astro-format = "0.7.0"
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

let ars = arrays::decode(&buf).unwrap();
```

### String

```
let buf: Vec<u8>;

let enc = string::encode::bytes(&buf);

let dec = string::decode::as_bytes(&enc).unwrap();
```

2022-04-30
