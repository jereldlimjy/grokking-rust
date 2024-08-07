/// Given a sorted array, creates a new array containing squares of all the numbers of the input array in sorted order.
///
/// # Arguments
///
/// * `arr` - A vector of `i32` integers in sorted order.
///
/// # Returns
///
/// * A vector of `u32` integers containing squares of all the numbers of the input array in sorted order.
///
/// # Complexity
///
/// * Time Complexity: O(N)
///   * O(N) as we iterate through the input vector once.
/// * Space Complexity: O(N)
///   * O(N) to store the new squared vector.
pub fn make_squares(arr: Vec<i32>) -> Vec<u32> {
    let mut squared = Vec::with_capacity(arr.len());

    // alternatively, find the first positive number then move pointers outwards
    let mut ptr1: usize = 0;
    let mut ptr2 = arr.len() - 1;

    while ptr1 <= ptr2 {
        let el_1_squared = arr[ptr1].pow(2);
        let el_2_squared = arr[ptr2].pow(2);

        if el_1_squared >= el_2_squared {
            squared.insert(0, el_1_squared as u32);
            ptr1 += 1;
        } else {
            squared.insert(0, el_2_squared as u32);
            ptr2 -= 1;
        }
    }

    squared
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_squares_1() {
        assert_eq!(make_squares(vec![-2, -1, 0, -2, 3]), vec![0, 1, 4, 4, 9]);
    }

    #[test]
    fn test_make_squares_2() {
        assert_eq!(make_squares(vec![-3, -1, 0, 1, 2]), vec![0, 1, 1, 4, 9]);
    }
}
