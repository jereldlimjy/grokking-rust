use std::collections::HashMap;

/// Finds all anagrams of the pattern in the given string.
///
/// # Arguments
///
/// * `str` - Input string of lowercase english letters.
/// * `pattern` - Pattern of english letters.
///
/// # Returns
///
/// * A `Vec<u32>` of starting indices.
///
///
/// # Complexity
///
/// * Time Complexity: O(N + M)
///   * O(N + M), where N and M are the length of the input string and pattern respectively, as we iterate over the string and pattern once.
/// * Space Complexity: O(M)
///   * In the worst case, the pattern has all distinct characters which will be stored in the hash map.
pub fn find_string_anagrams(str: String, pattern: String) -> Vec<u32> {
    let mut pattern_map = HashMap::new();
    let mut matched = 0;
    let mut indices_vec = Vec::with_capacity(str.len());
    let mut window_start = 0;

    for chr in pattern.chars() {
        pattern_map
            .entry(chr)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    for (idx, curr_char) in str.chars().enumerate() {
        pattern_map.entry(curr_char).and_modify(|count| {
            *count -= 1;

            if *count == 0 {
                matched += 1;
            }
        });

        // check if matched count equals unique chars in pattern
        if matched == pattern_map.keys().count() {
            indices_vec.push(window_start as u32);
        }

        // shrink window
        if idx >= pattern.len() - 1 {
            let char_to_remove = str.chars().nth(window_start).unwrap();

            pattern_map.entry(char_to_remove).and_modify(|count| {
                if *count == 0 {
                    matched -= 1;
                }

                *count += 1;
            });

            window_start += 1;
        }
    }

    indices_vec
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_anagrams_1() {
        assert_eq!(
            find_string_anagrams("ppqp".to_string(), "pq".to_string()),
            vec![1, 2]
        );
    }

    #[test]
    fn test_string_anagrams_2() {
        assert_eq!(
            find_string_anagrams("abbcabc".to_string(), "abc".to_string()),
            vec![2, 3, 4]
        );
    }
}
