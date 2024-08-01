/// Returns a given string with all the vowels reversed.
/// The vowels are 'a', 'e', 'i', 'o', and 'u', and they can appear in both lower and upper cases, more than once.
///
/// # Arguments
///
/// * `s` - A string.
///
/// # Returns
///
/// * A new string with the vowels reversed.
///
/// # Complexity
///
/// * Time Complexity: O(N)
///   * O(N) for iterating over the string.
/// * Space Complexity: O(N)
///   * O(N) for creating the vector of chars.
#[allow(dead_code)]
pub fn reverse_vowels(s: String) -> String {
    let mut s_vec: Vec<char> = s.chars().collect();
    let mut start: usize = 0;
    let mut end: usize = s_vec.len().saturating_sub(1); // handle empty strings

    while start < end {
        while !is_vowel(s_vec[start]) && start < end {
            start += 1;
        }

        while !is_vowel(s_vec[end]) && start < end {
            end -= 1;
        }

        // swap
        s_vec.swap(start, end);
        start += 1;
        end -= 1;
    }

    s_vec.into_iter().collect()
}

fn is_vowel(c: char) -> bool {
    matches!(c.to_ascii_lowercase(), 'a' | 'e' | 'i' | 'o' | 'u')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_string() {
        assert_eq!(reverse_vowels("".to_string()), "".to_string())
    }

    #[test]
    fn test_hello() {
        assert_eq!(reverse_vowels("hello".to_string()), "holle".to_string())
    }

    #[test]
    fn test_aeiou() {
        assert_eq!(reverse_vowels("AEIOU".to_string()), "UOIEA".to_string())
    }

    #[test]
    fn test_designgurus() {
        assert_eq!(
            reverse_vowels("DesignGUrus".to_string()),
            "DusUgnGires".to_string()
        )
    }
}
