/// Given an array of sorted numbers, moves all non-duplicate number instances at the beginning of the array in-place.
///
/// # Arguments
///
/// * `arr` - A vector of sorted `i32` integers.
///
/// # Returns
///
/// * The length of the subarray that has no duplicates in it.
///
/// # Complexity
///
/// * Time Complexity: O(N)
///   * O(N) as we iterate through the vector once.
/// * Space Complexity: O(1)
///   * Constant space as we only store `non_duplicate_idx` in a `usize` variable.
pub fn move_elements(mut arr: Vec<i32>) -> u32 {
    if arr.len() == 1 {
        return 1;
    }

    let mut non_duplicate_idx = 1;

    for i in 1..arr.len() {
        if arr[i] != arr[non_duplicate_idx - 1] {
            arr[non_duplicate_idx] = arr[i];
            non_duplicate_idx += 1;
        }
    }

    non_duplicate_idx as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_non_duplicate_instances_1() {
        assert_eq!(move_elements(vec![2, 3, 3, 3, 6, 9, 9]), 4);
    }

    #[test]
    fn test_find_non_duplicate_instances_2() {
        assert_eq!(move_elements(vec![2, 2, 2, 11]), 2);
    }

    #[test]
    fn test_find_non_duplicate_instances_3() {
        assert_eq!(move_elements(vec![2, 2, 2]), 1);
    }
}
