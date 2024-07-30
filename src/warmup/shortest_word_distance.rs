use std::u32;

/// Gets the shortest distance between 2 words in a list.
///
/// # Arguments
///
/// * `words` - A vector of strings.
/// * `word_1` - A string in `words`.
/// * `word_2` - Another string in `words`.
///
/// # Returns
///
/// * Shortest distance between `word_1` and `word_2`.
/// * `u32::MAX` if either word is not found
///
/// # Complexity
///
/// * Time Complexity: O(N)
///   * O(N) for iterating through the vector once.
/// * Space Complexity: O(1)
///   * Constant space as we use a fixed number of integer and usize variables.
pub fn shortest_distance(words: Vec<String>, word_1: String, word_2: String) -> u32 {
    let mut dist = u32::MAX;
    let mut word_1_ptr: Option<usize> = None;
    let mut word_2_ptr: Option<usize> = None;

    for (i, curr_word) in words.into_iter().enumerate() {
        if curr_word == word_1 {
            word_1_ptr = Some(i);
        } else if curr_word == word_2 {
            word_2_ptr = Some(i);
        }

        if let (Some(ptr_1), Some(ptr_2)) = (word_1_ptr, word_2_ptr) {
            let curr_dist = (ptr_1 as u32).abs_diff(ptr_2 as u32);
            if curr_dist < dist {
                dist = curr_dist
            }
        }
    }

    dist
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shortest_distance_basic() {
        let words = vec![
            "the".to_string(),
            "quick".to_string(),
            "brown".to_string(),
            "fox".to_string(),
            "quick".to_string(),
        ];
        assert_eq!(
            shortest_distance(words, "quick".to_string(), "fox".to_string()),
            1
        );
    }

    #[test]
    fn test_shortest_distance_multiple_occurrences() {
        let words = vec![
            "the".to_string(),
            "quick".to_string(),
            "brown".to_string(),
            "fox".to_string(),
            "quick".to_string(),
            "jumps".to_string(),
            "over".to_string(),
            "the".to_string(),
            "lazy".to_string(),
            "dog".to_string(),
        ];
        assert_eq!(
            shortest_distance(words, "the".to_string(), "quick".to_string()),
            1
        );
    }

    #[test]
    fn test_shortest_distance_not_found() {
        let words = vec![
            "the".to_string(),
            "quick".to_string(),
            "brown".to_string(),
            "fox".to_string(),
        ];
        assert_eq!(
            shortest_distance(words, "quick".to_string(), "cat".to_string()),
            u32::MAX
        );
    }

    #[test]
    fn test_shortest_distance_empty_list() {
        let words: Vec<String> = vec![];
        assert_eq!(
            shortest_distance(words, "quick".to_string(), "fox".to_string()),
            u32::MAX
        );
    }
}
