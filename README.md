# Astro Format

Astro Format is a library for transcoding between multiple arrays and a single buffer.

## Author

Roy R. O. Okello

[Email](mailto:royokello@protonmail.com)

[Github](https://github.com/royokello)

[Twitter](https://twitter.com/RealOkello)

## Usage

### Cargo.toml

```text
[dependencies]
astro-format = "1.2.0"
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

## License

MIT
