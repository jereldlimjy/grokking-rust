use std::collections::HashMap;

/// Given a string and a pattern, find out if the string contains any permutation of the pattern.
///
/// # Arguments
///
/// * `str` - Input string of lowercase english letters.
/// * `pattern` - Pattern of english letters.
///
/// # Returns
///
/// * `true` if input string contains permutation of pattern.
/// * `false` otherwise.
///
/// # Complexity
///
/// * Time Complexity: O(N + M)
///   * O(N + M), where N and M are the length of the input string and pattern respectively, as we iterate over the string and pattern once.
/// * Space Complexity: O(M)
///   * In the worst case, the whole patten has distinct characters which will all go into the hash map.
pub fn find_permutation(str: String, pattern: String) -> bool {
    let mut pattern_map = HashMap::new();
    let mut window_start: usize = 0;

    for chr in pattern.chars() {
        pattern_map
            .entry(chr.to_ascii_lowercase())
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    for (idx, curr_char) in str.chars().enumerate() {
        pattern_map.entry(curr_char).and_modify(|count| *count -= 1);

        if idx >= pattern.len() - 1 {
            // check if current window is a valid permutation
            // can also have a matched counter variable to check
            if pattern_map.values().all(|count| *count == 0) {
                return true;
            }

            // remove outgoing char
            let char_to_remove = str.chars().nth(window_start).unwrap();

            pattern_map
                .entry(char_to_remove)
                .and_modify(|count| *count += 1);

            window_start += 1;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_permutation_1() {
        assert_eq!(
            find_permutation("oidbcaf".to_string(), "ABC".to_string()),
            true
        );
    }

    #[test]
    fn test_permutation_2() {
        assert_eq!(
            find_permutation("odicf".to_string(), "dc".to_string()),
            false
        );
    }

    #[test]
    fn test_permutation_3() {
        assert_eq!(
            find_permutation("bcdxabcdy".to_string(), "bcdyabcdx".to_string()),
            true
        );
    }

    #[test]
    fn test_permutation_4() {
        assert_eq!(
            find_permutation("aaacb".to_string(), "abc".to_string()),
            true
        );
    }
}
