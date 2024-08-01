/// Finds the maximum sum of any contiguous subarray of size 'k' in an array of positive numbers.
///
/// # Arguments
///
/// * `k` - A positive integer representing the size of the subarray.
/// * `arr` - A vector of `u32` integers.
///
/// # Returns
///
/// * Maximum sum of any contiguous subarray of size 'k'.
///
/// # Complexity
///
/// * Time Complexity: O(N)
///   * O(N) for iterating over the vector once.
/// * Space Complexity: O(1)
///   * Constant space as we use a fixed number of integer and usize variables.
pub fn find_max_sum_subarray(k: u32, arr: Vec<u32>) -> u32 {
    let mut window_start: usize = 0;
    let mut window_sum: u32 = 0;
    let mut max_sum: u32 = 0;

    for (idx, curr_int) in arr.iter().enumerate() {
        window_sum += *curr_int;

        if idx >= (k - 1) as usize {
            if window_sum > max_sum {
                max_sum = window_sum;
            }

            window_sum -= arr[window_start];
            window_start += 1;
        }
    }

    max_sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_subarray_1() {
        assert_eq!(find_max_sum_subarray(3, vec![2, 1, 5, 1, 3, 2]), 9);
    }

    #[test]
    fn test_max_subarray_2() {
        assert_eq!(find_max_sum_subarray(2, vec![2, 3, 4, 1, 5]), 7);
    }
}
