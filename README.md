# Astro Format

Astro Format is a library for efficiently transcoding arrays into a single buffer and native rust types into hexadecimal strings.

## Usage

### Cargo.toml

```text
[dependencies]
astro-format = "0.8.0"
```

### Module.rs

```text
astro_format::{ string, arrays };
```

## API

### Array

```text
let ars: Vec<Vec<u8>>;

let buf = arrays::encode(&ars);

let ars = arrays::decode(&buf).unwrap();
```

### String

```text
let buf: Vec<u8>;

let enc = string::encode::bytes(&buf);

let dec = string::decode::as_bytes(&enc).unwrap();
```

2022-05-01
