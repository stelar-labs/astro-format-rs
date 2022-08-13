# Astro Format

Astro Format is a library for efficiently transcoding between multiple arrays and a single buffer.

## Usage

### Cargo.toml

```text
[dependencies]
astro-format = "1.0.0"
```

### Module.rs

```text
use astro_format;
```

## API

### Encode

```text
let initial_arrays: Vec<&[u8]> = Vec::new();

let encoded_buffer: Vec<u8> = astro_format::encode(&initial_arrays);
```

### Decode

```text
let decoded_arrays: Vec<&[u8]> = astro_format::decode(&encoded_buffer)?;
```

2022-08-10
