use std::collections::HashMap;

/// Returns the number of good pairs in an integer array.
/// A pair `(i, j)` is called good if `nums[i] == nums[j]` and `i < j`.
///
/// # Arguments
///
/// * `nums` - A vector of integers.
///
/// # Returns
///
/// * Number of good pairs.
///
/// # Complexity
///
/// * Time Complexity: O(N)
///   * O(N) for iterating through the vector, O(1) for hash map lookup and insertion.
/// * Space Complexity: O(N)
///   * In the worst case, all the integers are stored in the hash map if they are unique.
pub fn num_good_pairs(nums: Vec<u32>) -> u32 {
    let mut freq_map = HashMap::new();
    let mut good_pairs = 0;

    for num in nums {
        freq_map
            .entry(num)
            .and_modify(|count| {
                good_pairs += *count;
                *count += 1;
            })
            .or_insert(1);
    }

    good_pairs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_good_pairs_basic() {
        assert_eq!(num_good_pairs(vec![1, 2, 3, 1, 1, 3]), 4);
    }

    #[test]
    fn test_good_pairs_every_pair() {
        assert_eq!(num_good_pairs(vec![1, 1, 1, 1]), 6);
    }

    #[test]
    fn test_good_pairs_none() {
        assert_eq!(num_good_pairs(vec![1, 2, 3]), 0);
    }
}
