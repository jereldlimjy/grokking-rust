use std::collections::HashSet;

/// Checks if the given sentence is a pangram.
/// A pangram is a sentence that contains every letter of the alphabet at least once.
///
/// # Arguments
///
/// * `sentence` - A string slice that represents the sentence to be checked.
///
/// # Returns
///
/// * `true` if the sentence is a pangram.
/// * `false` otherwise.
///
/// # Complexity
///
/// * Time Complexity: O(N)
///   * O(N) for iterating through the sentence, O(1) for inserting into the hash set.
/// * Space Complexity: O(1)
///   * We are only adding lowercase alphabets, hence we add at most 26 chars to the hash set regardless of input size.
pub fn check_if_pangram(sentence: String) -> bool {
    let mut chars = HashSet::new();

    for chr in sentence.to_ascii_lowercase().chars() {
        if chr.is_ascii_lowercase() {
            chars.insert(chr);
        }
    }

    chars.len() == 26
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_pangram() {
        assert_eq!(
            check_if_pangram("TheQuickBrownFoxJumpsOverTheLazyDog".to_string()),
            true
        );
    }

    #[test]
    fn is_not_pangram() {
        assert_eq!(check_if_pangram("This is not a pangram".to_string()), false);
    }

    #[test]
    fn is_not_pangram_digits() {
        assert_eq!(
            check_if_pangram(
                "This is not a pangram. Here is a random number: 123456789".to_string()
            ),
            false
        );
    }
}
