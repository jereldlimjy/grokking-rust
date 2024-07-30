/// Checks if the given string is a palindrome.
/// A phrase is a palindrome if, after converting all uppercase letters into lowercase letters and
/// removing all non-alphanumeric characters, it reads the same forward and backward.
///
/// # Arguments
///
/// * `s` - a string that represents the phrase to be checked.
///
/// # Returns
///
/// * `true` if the phrase is a palindrome.
/// * `false` otherwise.
///
/// # Complexity
///
/// * Time Complexity: O(N)
///   * O(N) for converting into a vector of chars as well as iterating through the vector.
/// * Space Complexity: O(N)
///   * O(N) for storing the vector of chars.
pub fn is_palindrome(s: String) -> bool {
    if s.len() < 2 {
        return true;
    }

    let s_vec: Vec<char> = s.to_ascii_lowercase().chars().collect();
    let mut start: usize = 0;
    let mut end = s_vec.len() - 1;

    while start < end {
        while start < end && !s_vec[start].is_alphanumeric() {
            start += 1;
        }

        while start < end && !s_vec[end].is_alphanumeric() {
            end -= 1;
        }

        // check if equal
        if s_vec[start] != s_vec[end] {
            return false;
        }

        start += 1;
        end -= 1;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_palindrome_sentence_1() {
        assert_eq!(
            is_palindrome("A man, a plan, a canal, Panama!".to_string()),
            true
        );
    }

    #[test]
    fn is_palindrome_sentence_2() {
        assert_eq!(
            is_palindrome("Was it a car or a cat I saw?".to_string()),
            true
        );
    }

    #[test]
    fn is_palindrome_empty_string() {
        assert_eq!(is_palindrome("".to_string()), true);
    }

    #[test]
    fn is_palindrome_space() {
        assert_eq!(is_palindrome(" ".to_string()), true);
    }

    #[test]
    fn is_not_palindrome() {
        assert_eq!(is_palindrome("hello 123".to_string()), false);
    }
}
