use std::collections::HashMap;

/// Finds the smallest substring in the given string which has all the character occurrences of the given pattern.
///
/// # Arguments
///
/// * `str` - Input string of lowercase english letters.
/// * `pattern` - Pattern of english letters.
///
/// # Returns
///
/// * The smallest substring containing all character occurrences of the pattern.
///
///
/// # Complexity
///
/// * Time Complexity: O(N + M)
///   * O(N + M), where N and M are the length of the input string and pattern respectively, as we iterate over the string and pattern once.
/// * Space Complexity: O(M)
///   * In the worst case, the pattern has all distinct characters which will be stored in the hash map.
pub fn find_substring(str: String, pattern: String) -> String {
    let mut pattern_map = HashMap::new();
    let mut window_start: usize = 0;
    let mut matched = 0;
    let mut smallest_substring = "".to_string();

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

        // if correct matches, try to shrink the window
        while matched == pattern_map.keys().count() && window_start < idx {
            let curr_string = (str[window_start..(idx + 1)]).to_string();

            if smallest_substring == "".to_string() || curr_string.len() < smallest_substring.len()
            {
                smallest_substring = curr_string;
            }

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

    smallest_substring
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smallest_window_containing_substring_1() {
        assert_eq!(
            find_substring("aabdec".to_string(), "abc".to_string()),
            "abdec".to_string()
        );
    }

    #[test]
    fn test_smallest_window_containing_substring_2() {
        assert_eq!(
            find_substring("aabdec".to_string(), "abac".to_string()),
            "aabdec".to_string()
        );
    }

    #[test]
    fn test_smallest_window_containing_substring_3() {
        assert_eq!(
            find_substring("abdbca".to_string(), "abc".to_string()),
            "bca".to_string()
        );
    }

    #[test]
    fn test_smallest_window_containing_substring_4() {
        assert_eq!(
            find_substring("adcad".to_string(), "abc".to_string()),
            "".to_string()
        );
    }
}
