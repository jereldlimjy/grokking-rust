use std::collections::HashSet;

/// Checks if the given vector contains any duplicate elements.
///
/// # Arguments
///
/// * `nums` - A vector of integers.
///
/// # Returns
///
/// * `true` if there are duplicate elements in the vector.
/// * `false` otherwise.
///
/// # Complexity
///
/// * Time Complexity: O(N)
///   * O(N) for iterating through the vector once, O(1) lookup and insertion.
/// * Space Complexity: O(N)
///   * O(N) as the elements in the vector are stored in the hash set.
#[allow(dead_code)]
pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut seen = HashSet::new();

    for num in nums.iter() {
        if seen.contains(num) {
            return true;
        }

        seen.insert(num);
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn has_duplicate_only_positives() {
        assert_eq!(contains_duplicate(vec![1, 2, 3, 4, 4]), true);
    }

    #[test]
    fn has_duplicate_only_negatives() {
        assert_eq!(contains_duplicate(vec![-92, -100, -92, -40, -50]), true);
    }

    #[test]
    fn has_duplicate() {
        assert_eq!(contains_duplicate(vec![1, 90, -200, 300, 1]), true);
    }

    #[test]
    fn no_duplicate_only_positives() {
        assert_eq!(contains_duplicate(vec![1, 2, 3, 4, 5]), false);
    }

    #[test]
    fn no_duplicate_only_negatives() {
        assert_eq!(contains_duplicate(vec![-1, -200, -500, -90, -4]), false);
    }

    #[test]
    fn no_duplicate() {
        assert_eq!(contains_duplicate(vec![1, 100, 300, -400, 101]), false);
    }

    #[test]
    fn single_element() {
        assert_eq!(contains_duplicate(vec![1]), false);
    }
}
