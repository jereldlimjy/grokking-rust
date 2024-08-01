/// Given an array of 1s and 0s, finds the length of the longest contiguous subarray having all 1s with max of 'k' replacements.
///
/// # Arguments
///
/// * `arr`- A vector of `u8` integers.
/// * `k` - A `u32` integer representing the maximum number of 0s that can be replaced.
///
/// # Returns
///
/// * Length of the longest contiguous subarray having all 1s.
///
/// # Complexity
///
/// * Time Complexity: O(N)
///   * O(N) as each element in the vector is processed at most twice.
/// * Space Complexity: O(1)
///   * Constant space as we are using a fixed number of integer and usize variables.
pub fn find_length(arr: Vec<u8>, k: u32) -> u32 {
    let mut num_zeroes = 0;
    let mut longest_len: usize = 0;
    let mut window_start: usize = 0;

    for (idx, curr_el) in arr.iter().enumerate() {
        if *curr_el == 0 {
            num_zeroes += 1;
        }

        while num_zeroes > k {
            let el_to_remove = arr[window_start];

            if el_to_remove == 0 {
                num_zeroes -= 1;
            }

            window_start += 1;
        }

        longest_len = longest_len.max(idx - window_start + 1);
    }

    longest_len as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_subarray_with_ones_1() {
        assert_eq!(find_length(vec![0, 1, 1, 0, 0, 0, 1, 1, 0, 1, 1], 2), 6);
    }

    #[test]
    fn test_longest_subarray_with_ones_2() {
        assert_eq!(
            find_length(vec![0, 1, 0, 0, 1, 1, 0, 1, 1, 0, 0, 1, 1], 3),
            9
        );
    }

    #[test]
    fn test_longest_subarray_with_ones_3() {
        assert_eq!(find_length(vec![1, 0, 0, 1, 1, 0, 1, 1], 2), 6);
    }
}
