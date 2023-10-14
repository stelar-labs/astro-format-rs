# Astro Format

Astro Format is a library for transcoding between an array of implemented types and a single buffer.

## Authors

- Roy R. O. Okello: [Email](mailto:royokello@protonmail.com) & [GitHub](https://github.com/royokello)

## Usage

### Installation

- Using Crates by adding `astro-format = "2.0.0"` to `Cargo.toml` under `[dependencies]`

## API

### Encode

- `encode<T, I>(iterable: I) -> Result<Vec<u8>, Box<dyn Error>>`
- Bounds -> T: IntoBytes, I: IntoIterator<Item = T>

```
let list: Vec<&[u8]> = vec![&[1,2,3], &[4,5,6], &[7,8,9]];
let encoded = astro_format::encode(list).unwrap();
```

### Decode

- `decode<'a, T, B>(buffer: &'a B) -> Result<Vec<T>, Box<dyn std::error::Error>>`
- Bounds -> T: TryFromBytes<'a>, B: AsRef<[u8]>

```
let decoded: Vec<&[u8]> = astro_format::decode(&encoded).unwrap();
```

### IntoBytes

- `into_bytes(&self) -> Vec<u8>`
- Support for Rust Data Types

### TryFromBytes

- `try_from_bytes(value: &'a [u8]) -> Result<Self, Box<dyn std::error::Error>>`
- Support for Rust Data Types

## License

MIT License

Copyright Stelar Labs

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

## Disclaimer

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
