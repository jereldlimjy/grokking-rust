/// Finds a pair in the array whose sum is equal to the given target.
///
///
/// # Arguments
///
/// * `arr` - A vector of `i32` integers sorted in ascending order.
/// * `target_sum` - An `i32` target sum.
///
/// # Returns
///
/// * A tuple of type `(i32, i32)` with the indices of the two numbers.
/// * (-1, -1) if no such pair exists.
///
/// # Complexity
///
/// * Time Complexity: O(N)
///   * Linear time as we process each element in the vector at most once.
/// * Space Complexity: O(1)
///   * Constant space as we store only a fixed number of `usize` variables.
pub fn search(arr: Vec<i32>, target_sum: i32) -> (i32, i32) {
    let mut ptr1: usize = 0;
    let mut ptr2: usize = arr.len() - 1;

    while ptr1 < ptr2 {
        let curr_sum = arr[ptr1] + arr[ptr2];

        if curr_sum == target_sum {
            return (ptr1 as i32, ptr2 as i32);
        } else if curr_sum < target_sum {
            ptr1 += 1;
        } else {
            ptr2 -= 1;
        }
    }

    (-1, -1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pair_with_target_sum_positives() {
        assert_eq!(search(vec![1, 2, 3, 4, 6], 6), (1, 3));
    }

    #[test]
    fn test_pair_with_target_sum_negatives() {
        assert_eq!(search(vec![-8, -6, -4, -2], -12), (0, 2));
    }

    #[test]
    fn test_pair_with_target_sum_mixed() {
        assert_eq!(search(vec![-4, 2, 5, 9, 11], 1), (0, 2));
    }

    #[test]
    fn test_no_pair_with_target_sum_() {
        assert_eq!(search(vec![1, 2, 3, 4], 10), (-1, -1));
    }
}
