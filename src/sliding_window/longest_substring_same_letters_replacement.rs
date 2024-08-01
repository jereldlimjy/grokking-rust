use std::collections::HashMap;

/// Finds the length of the longest substring having the same letters after replacement of max 'k' letters.
///
/// # Arguments
///
/// * `str` - A string.
/// * `k` - A `u32` integer representing the maximum number of letters that can be replaced.
///
/// # Returns
///
/// * The length of the longest substring.
///
/// # Complexity
///
/// * Time Complexity: O(N)
///   * O(N) as each element in the vector is processed at most twice, once when traversing and another when shrinking the window.
/// * Space Complexity: O(1)
///   * Constant space as the hash map stores at most 26 letters.
pub fn find_length(str: String, k: u32) -> u32 {
    let mut longest_repeating_char_len = 0;
    let mut char_map = HashMap::new();
    let mut window_start: usize = 0;
    let mut longest_len: u32 = 0;

    for (idx, curr_char) in str.chars().enumerate() {
        char_map
            .entry(curr_char)
            .and_modify(|count| *count += 1)
            .or_insert(1);

        let curr_char_len = char_map.get(&curr_char).unwrap();

        if *curr_char_len > longest_repeating_char_len {
            longest_repeating_char_len = *curr_char_len;
        }

        while idx - window_start + 1 - longest_repeating_char_len > k as usize {
            // shrink window
            let char_to_remove = str.as_bytes()[window_start] as char;

            char_map
                .entry(char_to_remove)
                .and_modify(|count| *count -= 1);

            window_start += 1;
        }

        let window_len = idx - window_start + 1;

        longest_len = longest_len.max(window_len as u32);
    }

    longest_len
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn longest_substring_after_replacement_1() {
        assert_eq!(find_length("aabccbb".to_string(), 2), 5);
    }

    #[test]
    fn longest_substring_after_replacement_2() {
        assert_eq!(find_length("abbcb".to_string(), 1), 4);
    }

    #[test]
    fn longest_substring_after_replacement_3() {
        assert_eq!(find_length("abccde".to_string(), 1), 3);
    }
}
