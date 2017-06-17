extern crate is_anagram;
use is_anagram::IsAnagram;

#[test]
fn trivial() {
    assert!("banana".is_anagram_of("nabana").unwrap());
    assert!(!"lorem".is_anagram_of("ipsum").unwrap());
}
