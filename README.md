# Anagram identifier

A quick thing I threw together after seeing [this tweet](https://twitter.com/fermatslibrary/status/875340896379817984).

## What it does

```rust
extern crate is_anagram;
use is_anagram::IsAnagram;

fn main() {
    assert!("banana".is_anagram_of("nabana").unwrap());
    assert!(!"lorem".is_anagram_of("ipsum").unwrap());
}
```

## License

Licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
