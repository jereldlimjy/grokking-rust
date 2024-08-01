/// Finds the length of the smallest contiguous subarray whose sum is greater than or equal to 's'.
///
/// # Arguments
///
/// * `s` - A positive integer.
/// * `arr` - A vector of `u32` integers.
///
/// # Returns
///
/// * Length of the smallest contiguous subarray with sum >= `s`.
/// * 0 if no such subarray exists.
///
/// # Complexity
///
/// * Time Complexity: O(N)
///   * Each element is processed at most twice: once when expanding the window and once when shrinking it.
/// * Space Complexity: O(1)
///   * Constant space as we use a fixed number of integer and usize variables.
pub fn find_min_subarray(s: u32, arr: Vec<u32>) -> u32 {
    let mut window_start: usize = 0;
    let mut window_sum = 0;
    let mut smallest_len = u32::MAX;

    for (idx, curr_int) in arr.iter().enumerate() {
        window_sum += *curr_int;

        while window_sum >= s {
            let window_len = (idx - window_start + 1) as u32;

            if window_len < smallest_len {
                smallest_len = window_len;
            }

            window_sum -= arr[window_start];
            window_start += 1;
        }
    }

    if smallest_len == u32::MAX {
        0
    } else {
        smallest_len
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_subarray_1() {
        assert_eq!(find_min_subarray(7, vec![2, 1, 5, 2, 3, 2]), 2);
    }

    #[test]
    fn test_min_subarray_2() {
        assert_eq!(find_min_subarray(7, vec![2, 1, 5, 2, 8]), 1);
    }

    #[test]
    fn test_min_subarray_3() {
        assert_eq!(find_min_subarray(8, vec![3, 4, 1, 1, 6]), 3);
    }
}
