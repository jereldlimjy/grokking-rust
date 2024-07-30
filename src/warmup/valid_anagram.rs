use std::collections::HashMap;

/// Checks if the given 2 strings are anagrams.
/// An Anagram is a word or phrase formed by rearranging the letters of a different word or phrase,
/// typically using all the original letters exactly once.
///
/// # Arguments
///
/// * `s` - A string s.
/// * `t` - A string t.
///
/// # Returns
/// * `true` if `t` is an anagram of `s`.
/// * `false` otherwise.
///
/// # Complexity
///
/// * Time Complexity: O(N)
///   * O(N) for iterating through s and t (they must have the same length), O(1) for hash map lookup and insertion.
/// * Space Complexity: O(1)
///   * O(1) as the size of hash map is proportional to the number of unique characters - 26 chars in the worst case.
pub fn is_anagram(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }

    let mut freq_map: HashMap<char, i32> = HashMap::new();

    for (s_char, t_char) in s.chars().zip(t.chars()) {
        freq_map
            .entry(s_char)
            .and_modify(|count| *count += 1)
            .or_insert(1);
        freq_map
            .entry(t_char)
            .and_modify(|count| *count -= 1)
            .or_insert(-1);
    }

    freq_map.into_values().all(|count| count == 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_anagram_lowercase() {
        assert_eq!(is_anagram("listen".to_string(), "silent".to_string()), true);
    }

    #[test]
    fn is_not_anagram() {
        assert_eq!(is_anagram("rat".to_string(), "car".to_string()), false);
    }

    #[test]
    fn is_not_anagram_empty_string() {
        assert_eq!(is_anagram("listen".to_string(), "".to_string()), false);
    }
}
