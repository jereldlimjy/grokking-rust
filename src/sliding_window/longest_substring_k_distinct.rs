use std::collections::HashMap;

/// Finds the length of the longest substring with no more than 'k' distinct characters.
///
/// # Arguments
///
/// * `str` - A string.
/// * `k` - A positive integer representing the max number of distinct characters.
///
/// # Returns
///
/// * Length of the longest substring.
///
/// # Complexity
///
/// * Time Complexity: O(N)
///   * O(N) as we process each character in the string at most twice, once while traversing and another while shrinking.
/// * Space Complexity: O(K)
///   * O(K) as we will be storing at most K + 1 items in the hash map.
pub fn find_length(str: String, k: u32) -> u32 {
    let mut max_len = 0;
    let mut window_start: usize = 0;
    let mut chars_map = HashMap::new();

    for (idx, curr_char) in str.chars().enumerate() {
        chars_map
            .entry(curr_char)
            .and_modify(|count| *count += 1)
            .or_insert(1);
        let curr_len = (idx - window_start + 1) as u32;

        if chars_map.len() <= k as usize {
            // check if curr len is > max len
            if curr_len > max_len {
                max_len = curr_len;
            }
        } else {
            // shrink window
            while chars_map.len() > k as usize {
                let char_to_remove = str.chars().nth(window_start).unwrap();
                chars_map
                    .entry(char_to_remove)
                    .and_modify(|count| *count -= 1);

                if let Some(count) = chars_map.get(&char_to_remove) {
                    if *count == 0 {
                        chars_map.remove(&char_to_remove);
                    }
                }

                window_start += 1;
            }
        }
    }

    max_len
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_substring_1() {
        assert_eq!(find_length("araaci".to_string(), 2), 4);
    }

    #[test]
    fn test_longest_substring_2() {
        assert_eq!(find_length("araaci".to_string(), 1), 2);
    }

    #[test]
    fn test_longest_substring_3() {
        assert_eq!(find_length("cbbebi".to_string(), 3), 5);
    }
}
